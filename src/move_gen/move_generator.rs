use crate::board::{coord::Coord, moves::Move, Board, piece::Piece};
use crate::move_gen::precomp_move_data::*;

use crate::bitboard::bb::BitBoard;
use crate::bitboard::bbutils::BitBoardUtils;
use super::magics::MagicBitBoards;
use crate::bitboard::precomp_bits::PrecomputedBits;


#[derive(Default, PartialEq)]
pub enum PromotionMode {
    #[default]
    All,
    QueenOnly,
    QueenAndKnight,
}


pub struct MoveGenerator {
    pub moves: Vec<Move>,
    pub promotions_to_gen: PromotionMode,
    
    pub white_to_move: bool,
    pub friendly_color: u8,
    pub enemy_color: u8,
    friendly_king_sqr: Coord,
    friendly_idx: usize,
    enemy_idx: usize,

    in_check: bool,
    in_double_check: bool,

    pub check_ray_bitmask: BitBoard,

    pub pin_rays: BitBoard,
    pub not_pin_rays: BitBoard,
    pub enemy_attack_map_no_pawns: BitBoard,
    pub enemy_attack_map: BitBoard,
    pub enemy_pawn_attack_map: BitBoard,
    pub enemy_sliding_attack_map: BitBoard,

    gen_quiet_moves: bool,

    enemy_pieces: BitBoard,
    friendly_pieces: BitBoard,
    all_pieces: BitBoard,
    empty_sqrs: BitBoard,
    empty_or_enemy_sqrs: BitBoard,
    move_type_mask: BitBoard,
}

impl MoveGenerator {
    pub fn generate_moves(&mut self, board: &Board, precomp: &PrecomputedMoveData, bbutils: &BitBoardUtils, magic: &MagicBitBoards, captures_only: bool) {
        self.moves.clear();
        self.gen_quiet_moves = !captures_only;

        self.init(board, precomp, bbutils, magic);
        self.gen_king_moves(board, bbutils);

        if !self.in_double_check {
            self.gen_sliding_moves(board, magic, precomp);
            self.gen_knight_moves(board, bbutils);
            self.gen_pawn_moves(board, precomp, magic);
        }
    }

    pub fn in_check(&self) -> bool {
        self.in_check
    }
    
    fn init(&mut self, board: &Board, precomp: &PrecomputedMoveData, bbutils: &BitBoardUtils, magic: &MagicBitBoards) {
        self.in_check = false;
        self.in_double_check = false;
        self.check_ray_bitmask = BitBoard(0);
        self.pin_rays = BitBoard(0);

        self.white_to_move = board.move_color == Piece::WHITE;
        self.friendly_color = board.move_color;
        self.enemy_color = board.opponent_color;
        self.friendly_king_sqr = board.king_square[board.move_color_idx];
        self.friendly_idx = board.move_color_idx;
        self.enemy_idx = board.opponent_color_idx;

        self.enemy_pieces = board.color_bitboards[self.enemy_idx];
        self.friendly_pieces = board.color_bitboards[self.friendly_idx];
        self.all_pieces = board.all_pieces_bitboard;
        self.empty_sqrs = !self.all_pieces;
        self.empty_or_enemy_sqrs = self.empty_sqrs | self.enemy_pieces;
        self.move_type_mask = if self.gen_quiet_moves { BitBoard::ALL } else { self.enemy_pieces };

        self.calc_attack_data(board, precomp, bbutils, magic);
    }

    fn calc_attack_data(&mut self, board: &Board, precomp: &PrecomputedMoveData, bbutils: &BitBoardUtils, magic: &MagicBitBoards) {
        self.gen_sliding_attack_map(board, magic);
        let mut start_dir_idx = 0;
        let mut end_dir_idx = 8;

        // Don't calculate unecessary directions
        if board.piece_bitboards[Piece::new(Piece::QUEEN | self.enemy_color).index()].count() == 0 {
            start_dir_idx = if board.piece_bitboards[Piece::new(Piece::ROOK | self.enemy_color).index()].count() > 0 { 0 } else { 4 };
            end_dir_idx = if board.piece_bitboards[Piece::new(Piece::BISHOP | self.enemy_color).index()].count() > 0 { 8 } else { 4 };
        }

        for dir in start_dir_idx..end_dir_idx {
            let is_diagonal = dir > 3;
            let slider = if is_diagonal { board.enemy_diagonal_sliders } else { board.enemy_orthogonal_sliders };
            if (precomp.dir_ray_mask[self.friendly_king_sqr.index()][dir] & slider).0 == 0 { continue; }

            let n = precomp.num_sqrs_to_edge[self.friendly_king_sqr.index()][dir];
            let dir_offset = precomp.direction_offsets[dir];
            let mut is_friendly_piece_along_ray = false;
            let mut ray_mask = BitBoard(0);

            for i in 0..n {
                let sqr = self.friendly_king_sqr + dir_offset * (i + 1);
                ray_mask |= sqr.to_bitboard();
                let piece = board.square[sqr.index()];

                if piece != Piece::NULL {
                    if piece.is_color(self.friendly_color) {
                        if !is_friendly_piece_along_ray {
                            is_friendly_piece_along_ray = true;
                        } else { break; }
                    } else if (is_diagonal && piece.is_bishop_or_queen()) || (!is_diagonal && piece.is_rook_or_queen()) {
                        if is_friendly_piece_along_ray {
                            self.pin_rays |= ray_mask;
                        } else {
                            self.check_ray_bitmask |= ray_mask;
                            self.in_double_check = self.in_check;
                            self.in_check = true;
                        }
                        break;
                    } else { break; }
                }
            }
            if self.in_double_check { break; }
        };

        self.not_pin_rays = !self.pin_rays;
        let mut opponent_knight_attacks = BitBoard(0);
        let mut knights = board.piece_bitboards[Piece::new(Piece::KNIGHT | self.enemy_color).index()];
        let friendly_king_bitboard = board.piece_bitboards[Piece::new(Piece::KING | self.friendly_color).index()];

        while knights.0 != 0 {
            let knight_sqr = knights.pop_lsb();
            let knight_attacks = bbutils.knight_attacks[knight_sqr as usize];
            opponent_knight_attacks |= knight_attacks;

            if (knight_attacks & friendly_king_bitboard).0 != 0 {
                self.in_double_check = self.in_check;
                self.in_check = true;
                self.check_ray_bitmask |= 1 << knight_sqr;
            }
        }

        let enemy_pawns_bitboard = board.piece_bitboards[Piece::new(Piece::PAWN | self.enemy_color).index()];
        self.enemy_pawn_attack_map = BitBoardUtils::pawn_attacks(enemy_pawns_bitboard, !self.white_to_move);
        if self.enemy_pawn_attack_map.contains_square(self.friendly_king_sqr.square()) {
            self.in_double_check = self.in_check;
            self.in_check = true;
            let possible_pawn_attack_origins = if board.white_to_move { bbutils.white_pawn_attacks[self.friendly_king_sqr.index()] } else {
                bbutils.black_pawn_attacks[self.friendly_king_sqr.index()]};
            let pawn_check_map = enemy_pawns_bitboard & possible_pawn_attack_origins;
            self.check_ray_bitmask |= pawn_check_map;
        }

        let enemy_king_sqr = board.king_square[self.enemy_idx];
        self.enemy_attack_map_no_pawns = self.enemy_sliding_attack_map | opponent_knight_attacks | bbutils.king_moves[enemy_king_sqr.index()];
        self.enemy_attack_map = self.enemy_attack_map_no_pawns | self.enemy_pawn_attack_map;

        if !self.in_check {
            self.check_ray_bitmask = BitBoard::ALL;
        }
    }

    fn gen_sliding_attack_map(&mut self, board: &Board, magic: &MagicBitBoards) {
        self.enemy_sliding_attack_map = BitBoard(0);
        self.update_slide_attack(board, magic, board.enemy_orthogonal_sliders, true);
        self.update_slide_attack(board, magic, board.enemy_diagonal_sliders, false);
    }

    fn update_slide_attack(&mut self, board: &Board, magic: &MagicBitBoards, mut piece_board: BitBoard, ortho: bool) {
        let blockers = board.all_pieces_bitboard & !self.friendly_king_sqr.to_bitboard();
        while piece_board.0 != 0 {
            let start = Coord::from_idx(piece_board.pop_lsb() as i8);
            let move_board = magic.get_slider_attacks(start, blockers, ortho);
            self.enemy_sliding_attack_map |= move_board;
        }
    }

    fn is_pinned(&self, sqr: Coord) -> bool {
        ((self.pin_rays >> sqr.index()) & 1).0 != 0
    }

    fn gen_king_moves(&mut self, board: &Board, bbutils: &BitBoardUtils) {
        let legal_mask = !(self.enemy_attack_map | self.friendly_pieces);
        let mut king_moves = bbutils.king_moves[self.friendly_king_sqr.index()] & legal_mask & self.move_type_mask;
        while king_moves.0 != 0 {
            let target_sqr = king_moves.pop_lsb() as i8;
            self.moves.push(Move::from_start_end(self.friendly_king_sqr.square(), target_sqr));
        }

        if !self.in_check && self.gen_quiet_moves {
            let castle_blockers = self.enemy_attack_map | board.all_pieces_bitboard;
            if board.current_state.has_kingside_castle_right(self.white_to_move) {
                let castle_mask = if self.white_to_move { PrecomputedBits::WHITE_KINGSIDE_MASK } else { PrecomputedBits::BLACK_KINGSIDE_MASK };
                if (castle_mask & castle_blockers).0 == 0 {
                    let target = if self.white_to_move { Coord::G1 } else { Coord::G8 };
                    self.moves.push(Move::from_start_end_flagged(self.friendly_king_sqr.square(), target.square(), Move::CASTLING));
                }
            }
            if board.current_state.has_queenside_castle_right(self.white_to_move) {
                let castle_mask = if self.white_to_move { PrecomputedBits::WHITE_QUEENSIDE_MASK_2 } else { PrecomputedBits::BLACK_QUEENSIDE_MASK_2 };
                let castle_block_mask = if self.white_to_move { PrecomputedBits::WHITE_QUEENSIDE_MASK } else { PrecomputedBits::BLACK_QUEENSIDE_MASK };
                if (castle_mask & castle_blockers).0 == 0 && (castle_block_mask & board.all_pieces_bitboard).0 == 0 {
                    let target = if self.white_to_move { Coord::C1 } else { Coord::C8 };
                    self.moves.push(Move::from_start_end_flagged(self.friendly_king_sqr.square(), target.square(), Move::CASTLING));
                }
            }
        }
    }

    fn gen_sliding_moves(&mut self, board: &Board, magic: &MagicBitBoards, precomp: &PrecomputedMoveData) {
        let move_mask = self.empty_or_enemy_sqrs & self.check_ray_bitmask & self.move_type_mask;
        let mut orthogonal_sliders = board.friendly_orthogonal_sliders;
        let mut diagonal_sliders = board.friendly_diagonal_sliders;

        if self.in_check {
            orthogonal_sliders &= self.not_pin_rays;
            diagonal_sliders &= self.not_pin_rays;
        }

        while orthogonal_sliders.0 != 0 {
            let start = Coord::from_idx(orthogonal_sliders.pop_lsb() as i8);
            // ! Incredibly stange bug where H8 is considered a legal starting position even though
            // ! there is no rook there. Probably has to do with castling. 
            // if board.square[start.index()].piece_type() == Piece::NONE {
            //     log_debug(format!("Tried to generate {:?} rook move\n{}", start, crate::fen::fen_from_position(board)));
            //     continue;
            // };
            let mut move_sqrs = magic.get_rook_attacks(start, self.all_pieces) & move_mask;
            if self.is_pinned(start) {
                move_sqrs &= precomp.align_mask[start.index()][self.friendly_king_sqr.index()];
            }
            while move_sqrs.0 != 0 {
                let target = move_sqrs.pop_lsb() as i8;
                self.moves.push(Move::from_start_end(start.square(), target));
            }
        }

        while diagonal_sliders.0 != 0 {
            let start = Coord::from_idx(diagonal_sliders.pop_lsb() as i8);
            let mut move_sqrs = magic.get_bishop_attacks(start, self.all_pieces) & move_mask;
            if self.is_pinned(start) {
                move_sqrs &= precomp.align_mask[start.index()][self.friendly_king_sqr.index()];
            }
            while move_sqrs.0 != 0 {
                let target = move_sqrs.pop_lsb() as i8;
                self.moves.push(Move::from_start_end(start.square(), target));
            }
        }
    }

    fn gen_knight_moves(&mut self, board: &Board, bbutils: &BitBoardUtils) {
        let friendly_knight_piece = Piece::new(Piece::KNIGHT | self.friendly_color);
        let mut knights = board.piece_bitboards[friendly_knight_piece.index()] & self.not_pin_rays;
        let move_mask = self.empty_or_enemy_sqrs & self.check_ray_bitmask & self.move_type_mask;

        while knights.0 != 0 {
            let knight_sqr = knights.pop_lsb() as i8;
            let mut move_sqrs = bbutils.knight_attacks[knight_sqr as usize] & move_mask;
            while move_sqrs.0 != 0 {
                let target = move_sqrs.pop_lsb() as i8;
                self.moves.push(Move::from_start_end(knight_sqr, target));
            }
        }
    }

    fn gen_pawn_moves(&mut self, board: &Board, precomp: &PrecomputedMoveData, magic: &MagicBitBoards) {
        let push_dir = if self.white_to_move { 1i8 } else { -1i8 };
        let push_offset = push_dir * 8;

        let friendly_pawn_piece = Piece::new(Piece::PAWN | self.friendly_color);
        let pawns = board.piece_bitboards[friendly_pawn_piece.index()];

        let prom_rank_mask = if self.white_to_move { BitBoardUtils::RANK_8 } else { BitBoardUtils::RANK_1 };
        let single_push = pawns.shifted(push_offset) & self.empty_sqrs;
        let mut push_proms = single_push & prom_rank_mask & self.check_ray_bitmask;

        let capture_edge_file_mask = if self.white_to_move { BitBoardUtils::NOT_FILE_A } else { BitBoardUtils::NOT_FILE_H };
        let capture_edge_file_mask_2 = if self.white_to_move { BitBoardUtils::NOT_FILE_H } else { BitBoardUtils::NOT_FILE_A };
        let mut capture_a = (pawns & capture_edge_file_mask).shifted(push_dir * 7) & self.enemy_pieces;
        let mut capture_b = (pawns & capture_edge_file_mask_2).shifted(push_dir * 9) & self.enemy_pieces;

        let mut single_push_no_proms = single_push & !prom_rank_mask & self.check_ray_bitmask;
        let mut capture_proms_a = capture_a & prom_rank_mask & self.check_ray_bitmask;
        let mut capture_proms_b = capture_b & prom_rank_mask & self.check_ray_bitmask;

        capture_a &= self.check_ray_bitmask & !prom_rank_mask;
        capture_b &= self.check_ray_bitmask & !prom_rank_mask;

        if self.gen_quiet_moves {
            while single_push_no_proms.0 != 0 {
                let target_sqr = single_push_no_proms.pop_lsb() as i8;
                let start_sqr = target_sqr - push_offset;
                if !self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()] {
                    self.moves.push(Move::from_start_end(start_sqr, target_sqr));
                }
            }

            let double_push_target_rank_mask = if self.white_to_move { BitBoardUtils::RANK_4 } else { BitBoardUtils::RANK_5 };
            let mut double_push = single_push.shifted(push_offset) & self.empty_sqrs & double_push_target_rank_mask & self.check_ray_bitmask;
            while double_push.0 != 0 {
                let target_sqr = double_push.pop_lsb() as i8;
                let start_sqr = target_sqr - push_offset * 2;
                if !self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()] {
                    self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::PAWN_TWO_FORWARD));
                }
            }
        }

        while capture_a.0 != 0 {
            let target_sqr = capture_a.pop_lsb() as i8;
            let start_sqr = target_sqr - push_dir * 7;
            if !self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()] {
                self.moves.push(Move::from_start_end(start_sqr, target_sqr));
            }
        }
        while capture_b.0 != 0 {
            let target_sqr = capture_b.pop_lsb() as i8;
            let start_sqr = target_sqr - push_dir * 9;
            if !self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()] {
                self.moves.push(Move::from_start_end(start_sqr, target_sqr));
            }
        }

        while push_proms.0 != 0 {
            let target_sqr = push_proms.pop_lsb() as i8;
            let start_sqr = target_sqr - push_offset;
            if !self.is_pinned(Coord::from_idx(start_sqr)) {
                self.gen_proms(start_sqr, target_sqr);
            }
        }
        while capture_proms_a.0 != 0 {
            let target_sqr = capture_proms_a.pop_lsb() as i8;
            let start_sqr = target_sqr - push_dir * 7;
            if !self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()] {
                self.gen_proms(start_sqr, target_sqr);
            }
        }
        while capture_proms_b.0 != 0 {
            let target_sqr = capture_proms_b.pop_lsb() as i8;
            let start_sqr = target_sqr - push_dir * 9;
            if !self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()] {
                self.gen_proms(start_sqr, target_sqr);
            }
        }

        if board.current_state.en_passant_file > 0 {
            let ep_file_idx = board.current_state.en_passant_file - 1;
            let ep_rank_idx = if self.white_to_move { 5 } else { 2 };
            let target_sqr = ep_rank_idx * 8 + ep_file_idx;
            let captured_pawn_sqr = target_sqr - push_offset;
            if self.check_ray_bitmask.contains_square(captured_pawn_sqr) {
                let mut pawns_that_can_ep = pawns & BitBoardUtils::pawn_attacks(BitBoard(1 << target_sqr), !self.white_to_move);
                while pawns_that_can_ep.0 != 0 {
                    let start_sqr = pawns_that_can_ep.pop_lsb() as i8;
                    if (!self.is_pinned(Coord::from_idx(start_sqr)) || precomp.align_mask[start_sqr as usize][self.friendly_king_sqr.index()] == precomp.align_mask[target_sqr as usize][self.friendly_king_sqr.index()]) && !self.in_check_after_ep(board, magic, start_sqr, target_sqr, captured_pawn_sqr) {
                        self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::EN_PASSANT_CAPTURE));
                    }
                }
            }
        }
    }

    fn gen_proms(&mut self, start_sqr: i8, target_sqr: i8) {
        self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::QUEEN_PROMOTION));
        if self.gen_quiet_moves {
            if self.promotions_to_gen == PromotionMode::All {
                self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::KNIGHT_PROMOTION));
                self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::ROOK_PROMOTION));
                self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::BISHOP_PROMOTION));
            } else if self.promotions_to_gen == PromotionMode::QueenAndKnight {
                self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, Move::KNIGHT_PROMOTION));
            }
        }
    }

    fn in_check_after_ep(&self, board: &Board, magic: &MagicBitBoards, start_sqr: i8, target_sqr: i8, captured_pawn_sqr: i8) -> bool {
        let enemy_ortho = board.enemy_orthogonal_sliders;
        if enemy_ortho.0 != 0 {
            let masked_blockers = self.all_pieces ^ (1 << captured_pawn_sqr | 1 << start_sqr | 1 << target_sqr);
            let rook_attacks = magic.get_rook_attacks(self.friendly_king_sqr, masked_blockers);
            return (rook_attacks & enemy_ortho).0 != 0;
        }
        false
    }
}


impl Default for MoveGenerator {
    fn default() -> Self {
        MoveGenerator {
            moves: Vec::new(),
            promotions_to_gen: PromotionMode::All,
            white_to_move: true,
            friendly_color: Piece::WHITE,
            enemy_color: Piece::BLACK,
            friendly_king_sqr: Coord::new(0, 0),
            friendly_idx: Board::WHITE_INDEX,
            enemy_idx: Board::BLACK_INDEX,
            in_check: false,
            in_double_check: false,
            check_ray_bitmask: BitBoard(0),
            pin_rays: BitBoard(0),
            not_pin_rays: BitBoard(0),
            enemy_attack_map_no_pawns: BitBoard(0),
            enemy_attack_map: BitBoard(0),
            enemy_pawn_attack_map: BitBoard(0),
            enemy_sliding_attack_map: BitBoard(0),
            gen_quiet_moves: true,
            enemy_pieces: BitBoard(0),
            friendly_pieces: BitBoard(0),
            all_pieces: BitBoard(0),
            empty_sqrs: BitBoard(0),
            empty_or_enemy_sqrs: BitBoard(0),
            move_type_mask: BitBoard(0)
        }
    }
}

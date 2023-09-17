use bevy::prelude::*;
use crate::{
    board::moves::Move,
    board::board::Board,
    board::piece::*,
    move_gen::precomp_move_data::PrecomputedMoveData,
    board::coord::*,
};


#[derive(Resource)]
pub struct PseudoLegalMoveGenerator {
    pub moves: Vec<Move>,
    pub white_to_move: bool,
    pub friendly_color: u8,
    pub opponent_color: u8,
    pub friendly_king_sqr: Coord,
    pub friendly_color_idx: usize,
    pub opponent_color_idx: usize,

    pub gen_quiets: bool,
    pub gen_under_promotions: bool,
}

impl PseudoLegalMoveGenerator {
    pub fn generate_moves(&mut self, board: &Res<Board>, precomp: &Res<PrecomputedMoveData>) {
        self.init(board);
        self.gen_king_moves(board, precomp);
        self.gen_sliding_moves(board, precomp);
        self.gen_knight_moves(board, precomp);
        self.gen_pawn_moves(board, precomp);
    }
    pub fn illegal(&self, board: &Board, precomp: &Res<PrecomputedMoveData>) -> bool {
        return self.sqr_attacked(board, precomp, board.king_square[1 - board.color_to_move_idx], board.color_to_move);
    }
    pub fn sqr_attacked(&self, board: &Board, precomp: &Res<PrecomputedMoveData>, attack_sqr: Coord, attacker_color: u8) -> bool {
        let attacker_color_idx = if attacker_color == Piece::WHITE { Board::WHITE_INDEX } else { Board::BLACK_INDEX };
        let friendly_color_idx = 1 - attacker_color_idx;
        let friendly_color = if attacker_color == Piece::WHITE { Piece::BLACK } else { Piece::WHITE };

        let mut start_dir_idx = 0;
        let mut end_dir_idx = 8;

        let opponent_king_sqr = board.king_square[attacker_color_idx];
        if precomp.king_distance[opponent_king_sqr.index()][attack_sqr.index()] == 1 {
            return true;
        }
        if board.get_piece_list(Piece::QUEEN, attacker_color_idx).count() == 0 {
            start_dir_idx = if board.get_piece_list(Piece::ROOK, attacker_color_idx).count() > 0 { 0 } else { 4 };
            end_dir_idx = if board.get_piece_list(Piece::BISHOP, attacker_color_idx).count() > 0 { 8 } else { 4 };
        }
        for dir in start_dir_idx..end_dir_idx {
            let is_diagonal = dir > 3;
            let n = precomp.num_sqrs_to_edge[attack_sqr.index()][dir];
            let direction_offset = precomp.direction_offsets[dir];
            for i in 0..n {
                let sqr_idx = attack_sqr.square() as i32 + direction_offset * (i + 1);
                let piece = board.square[sqr_idx as usize];
                if piece != Piece::NULL {
                    if piece.is_color(friendly_color) { break; }
                    else {
                        let ptype = piece.piece_type();
                        if (is_diagonal && piece.is_bishop_or_queen()) || (!is_diagonal && piece.is_rook_or_queen()) {
                            return true;
                        } else {
                            break;
                        }
                    }
                }
            }
        };
        let knight_attack_sqrs = &precomp.knight_moves[attack_sqr.index()];
        for i in 0..knight_attack_sqrs.len() {
            if board.square[knight_attack_sqrs[i].index()].value() == (Piece::KNIGHT | attacker_color) {
                return true;
            }
        };
        for i in 0..2 {
            if precomp.num_sqrs_to_edge[attack_sqr.index()][precomp.pawn_attack_dirs[friendly_color_idx][i].index()] > 0 {
                let s = attack_sqr.square() as i32 + precomp.direction_offsets[precomp.pawn_attack_dirs[friendly_color_idx][i].index()];
                let piece = board.square[s as usize];
                if piece.value() == (Piece::PAWN | attacker_color) { return true; }
            }
        };

        return false;
    }
    fn init(&mut self, board: &Board) {
        self.moves.clear();

        self.white_to_move = board.color_to_move == Piece::WHITE;
        self.friendly_color = board.color_to_move;
        self.opponent_color = board.opponent_color;
        self.friendly_king_sqr = board.king_square[board.color_to_move_idx];
        self.friendly_color_idx = if board.white_to_move { Board::WHITE_INDEX } else { Board::BLACK_INDEX };
        self.opponent_color_idx = 1 - self.friendly_color_idx;
    }
    fn gen_king_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        for i in 0..precomp.king_moves[self.friendly_king_sqr.index()].len() {
            let target_sqr = precomp.king_moves[self.friendly_king_sqr.index()][i];
            let piece_on_target = board.square[target_sqr.index()];

            if piece_on_target.is_color(self.friendly_color) { continue; }

            let is_capture = piece_on_target.is_color(self.opponent_color);
            if !is_capture && !self.gen_quiets { continue; }

            self.moves.push(Move::from_start_end(self.friendly_king_sqr.square(), target_sqr.square()));
            
            if !is_capture && !self.sqr_attacked(board, precomp, self.friendly_king_sqr, self.opponent_color) {
                if (target_sqr == Coord::F1 || target_sqr == Coord::F8) && self.has_kingside_castle_right(board) {
                    if !self.sqr_attacked(board, precomp, target_sqr, self.opponent_color) {
                        let castle_kingside_sqr = target_sqr + 1;
                        if board.square[castle_kingside_sqr.index()] == Piece::NULL {
                            self.moves.push(Move::from_start_end_flagged(self.friendly_king_sqr.square(), castle_kingside_sqr.square(), Move::CASTLING));
                        }
                    }
                }
                else if (target_sqr == Coord::D1 || target_sqr == Coord::D8) && self.has_queenside_castle_right(board) {
                    if !self.sqr_attacked(board, precomp, target_sqr, self.opponent_color) {
                        let castle_queenside_sqr = target_sqr - 1;
                        if board.square[castle_queenside_sqr.index()] == Piece::NULL && board.square[castle_queenside_sqr.index() - 1] == Piece::NULL {
                            self.moves.push(Move::from_start_end_flagged(self.friendly_king_sqr.square(), castle_queenside_sqr.square(), Move::CASTLING));
                        }
                    }
                }
            }
        }
    }
    fn gen_sliding_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        let rooks = board.get_piece_list(Piece::ROOK, self.friendly_color_idx);
        for i in 0..rooks.count() {
            self.gen_sliding_piece_moves(board, precomp, rooks[i as usize], 0, 4);
        }
        let bishops = board.get_piece_list(Piece::BISHOP, self.friendly_color_idx);
        for i in 0..rooks.count() {
            self.gen_sliding_piece_moves(board, precomp, bishops[i as usize], 4, 8);
        }
        let queens = board.get_piece_list(Piece::QUEEN, self.friendly_color_idx);
        for i in 0..rooks.count() {
            self.gen_sliding_piece_moves(board, precomp, queens[i as usize], 0, 8);
        }
    }
    fn gen_sliding_piece_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>, start_sqr: Coord, start_dir_idx: usize, end_dir_idx: usize) {
        for dir_idx in start_dir_idx..end_dir_idx {
            let cur_dir_offset = precomp.direction_offsets[dir_idx];
            for n in 0..precomp.num_sqrs_to_edge[start_sqr.index()][dir_idx] {
                let target_sqr = start_sqr.square() as i32 + cur_dir_offset * (n + 1);
                let target_sqr_piece = board.square[target_sqr as usize];

                if target_sqr_piece.is_color(self.friendly_color) { break; }

                let is_capture = target_sqr_piece != Piece::NULL;
                if self.gen_quiets || is_capture {
                    self.moves.push(Move::from_start_end(start_sqr.square(), target_sqr as u8));
                }
                if is_capture { break; }
            }
        }
    }
    fn gen_knight_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        let knights = board.get_piece_list(Piece::KNIGHT, self.friendly_color_idx);
        for i in 0..knights.count() {
            let start_sqr = knights[i as usize];
            for knight_move_idx in 0..precomp.knight_moves[start_sqr.index()].len() {
                let target_sqr = precomp.knight_moves[start_sqr.index()][knight_move_idx];
                let target_sqr_piece = board.square[target_sqr.index()];
                let is_capture = target_sqr_piece.is_color(self.opponent_color);
                if self.gen_quiets || is_capture {
                    if target_sqr_piece.is_color(self.friendly_color) { continue; }
                    self.moves.push(Move::from_start_end(start_sqr.square(), target_sqr.square()));
                }
            }
        }
    }
    fn gen_pawn_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        let pawns = board.get_piece_list(Piece::PAWN, self.friendly_color_idx);
        let pawn_offset = if self.friendly_color == Piece::WHITE { 8 } else { -8 };
        let start_rank = if self.white_to_move { 1 } else { 6 };
        let final_rank_before_prom = if self.white_to_move { 6 } else { 1 };

        let en_passant_file = ((board.current_game_state >> 4) & 15) as i32 - 1;
        let mut en_passant_sqr = -1;
        if en_passant_file != -1 {
            en_passant_sqr = 8 * (if self.white_to_move { 5 } else { 2 }) + en_passant_file;
        }

        for i in 0..pawns.count() {
            let start_sqr = pawns[i as usize];
            let rank = start_sqr.rank();
            let one_step_from_prom = rank == final_rank_before_prom;

            if self.gen_quiets {
                let square_one_forward = start_sqr.square() as i32 + pawn_offset;
                if board.square[square_one_forward as usize] == Piece::NULL {
                    if one_step_from_prom {
                        self.make_promotion_moves(start_sqr, Coord::from_idx(square_one_forward as u8));
                    } else {
                        self.moves.push(Move::from_start_end(start_sqr.square(), square_one_forward as u8));
                    }
                    if rank == start_rank {
                        let square_two_forward = square_one_forward + pawn_offset;
                        if board.square[square_two_forward as usize] == Piece::NULL {
                            self.moves.push(Move::from_start_end_flagged(start_sqr.square(), square_two_forward as u8, Move::PAWN_TWO_FORWARD));
                        }
                    }
                }
            }

            for j in 0..2 {
                if precomp.num_sqrs_to_edge[start_sqr.index()][precomp.pawn_attack_dirs[self.friendly_color_idx][j].index()] > 0 {
                    let pawn_capture_dir = precomp.direction_offsets[precomp.pawn_attack_dirs[self.friendly_color_idx][j].index()];
                    let target_sqr = start_sqr.square() as i32 + pawn_capture_dir;
                    let target_piece = board.square[target_sqr as usize];

                    if target_piece.is_color(self.opponent_color) {
                        if one_step_from_prom {
                            self.make_promotion_moves(start_sqr, Coord::from_idx(target_sqr as u8));
                        } else {
                            self.moves.push(Move::from_start_end(start_sqr.square(), target_sqr as u8));
                        }
                    }
                    if target_sqr == en_passant_sqr {
                        let ep_captured_pawn_sqr = target_sqr + if self.white_to_move { -8 } else { 8 };
                        self.moves.push(Move::from_start_end_flagged(start_sqr.square(), target_sqr as u8, Move::EN_PASSANT_CAPTURE));
                    }
                }
            }
        }
    }

    fn make_promotion_moves(&mut self, start_sqr: Coord, target_sqr: Coord) {
        self.moves.push(Move::from_start_end_flagged(start_sqr.square(), target_sqr.square(), Move::QUEEN_PROMOTION));
        if self.gen_under_promotions {
            self.moves.push(Move::from_start_end_flagged(start_sqr.square(), target_sqr.square(), Move::KNIGHT_PROMOTION));
            self.moves.push(Move::from_start_end_flagged(start_sqr.square(), target_sqr.square(), Move::ROOK_PROMOTION));
            self.moves.push(Move::from_start_end_flagged(start_sqr.square(), target_sqr.square(), Move::BISHOP_PROMOTION));
        }
    }
    fn has_kingside_castle_right(&self, board: &Board) -> bool {
        let mask = if board.white_to_move { 1 } else { 4 };
        return (board.current_game_state & mask) != 0;
    }
    fn has_queenside_castle_right(&self, board: &Board) -> bool {
        let mask = if board.white_to_move { 2 } else { 8 };
        return (board.current_game_state & mask) != 0;
    }
}

impl Default for PseudoLegalMoveGenerator {
    fn default() -> Self {
        PseudoLegalMoveGenerator {
            moves: Vec::new(),
            white_to_move: true,
            friendly_color: Piece::WHITE,
            opponent_color: Piece::BLACK,
            friendly_king_sqr: Coord::NULL,
            friendly_color_idx: Board::WHITE_INDEX,
            opponent_color_idx: Board::BLACK_INDEX,
            gen_quiets: true,
            gen_under_promotions: true,
        }
    }
}

pub fn spawn_pseudo_move_gen(
    mut commands: Commands,
) {
    commands.insert_resource(PseudoLegalMoveGenerator::default());
}

// pub fn generate_pseudo_legal_moves(
//     mut pseudo_move_generator: ResMut<PseudoLegalMoveGenerator>,
//     mut make_move_evr: EventReader<BoardMakeMove>,
//     board: Res<Board>,
//     precomp_data: Res<PrecomputedMoveData>,
// ) {
//     for _ in make_move_evr.iter() {
//         pseudo_move_generator.generate_moves(&board, &precomp_data);
//     }
// }

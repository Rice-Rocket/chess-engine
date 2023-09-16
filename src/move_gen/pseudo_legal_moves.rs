use bevy::prelude::*;
use crate::{
    board::moves::{Move, CASTLING, PAWN_TWO_FORWARD, EN_PASSANT_CAPTURE, QUEEN_PROMOTION, ROOK_PROMOTION, KNIGHT_PROMOTION, BISHOP_PROMOTION},
    board::board::{Board, WHITE_INDEX, BLACK_INDEX},
    board::piece::{WHITE, BLACK, NONE, QUEEN, ROOK, BISHOP, KNIGHT, PAWN, is_color, piece_type, is_bishop_or_queen, is_rook_or_queen},
    move_gen::precomp_move_data::PrecomputedMoveData,
    game::representation::{self, rank_idx},
};


#[derive(Resource)]
pub struct PseudoLegalMoveGenerator {
    pub moves: Vec<Move>,
    pub white_to_move: bool,
    pub friendly_color: u32,
    pub opponent_color: u32,
    pub friendly_king_sqr: u32,
    pub friendly_color_idx: u32,
    pub opponent_color_idx: u32,

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
        return self.sqr_attacked(board, precomp, board.king_square[1 - board.color_to_move_idx as usize], board.color_to_move);
    }
    pub fn sqr_attacked(&self, board: &Board, precomp: &Res<PrecomputedMoveData>, attack_sqr: u32, attacker_color: u32) -> bool {
        let attacker_color_idx = if attacker_color == WHITE { WHITE_INDEX } else { BLACK_INDEX };
        let friendly_color_idx = 1 - attacker_color_idx;
        let friendly_color = if attacker_color == WHITE { BLACK } else { WHITE };

        let mut start_dir_idx = 0;
        let mut end_dir_idx = 8;

        let opponent_king_sqr = board.king_square[attacker_color_idx as usize];
        if precomp.king_distance[opponent_king_sqr as usize][attack_sqr as usize] == 1 {
            return true;
        }
        if board.get_piece_list(QUEEN, attacker_color_idx).count() == 0 {
            start_dir_idx = if board.get_piece_list(ROOK, attacker_color_idx).count() > 0 { 0 } else { 4 };
            end_dir_idx = if board.get_piece_list(BISHOP, attacker_color_idx).count() > 0 { 8 } else { 4 };
        }
        for dir in start_dir_idx..end_dir_idx {
            let is_diagonal = dir > 3;
            let n = precomp.num_sqrs_to_edge[attack_sqr as usize][dir];
            let direction_offset = precomp.direction_offsets[dir];
            for i in 0..n {
                let sqr_idx = attack_sqr as i32 + direction_offset * (i + 1);
                let piece = board.square[sqr_idx as usize];
                if piece != NONE {
                    if is_color(piece, friendly_color) { break; }
                    else {
                        let ptype = piece_type(piece);
                        if (is_diagonal && is_bishop_or_queen(ptype)) || (!is_diagonal && is_rook_or_queen(ptype)) {
                            return true;
                        } else {
                            break;
                        }
                    }
                }
            }
        };
        let knight_attack_sqrs = &precomp.knight_moves[attack_sqr as usize];
        for i in 0..knight_attack_sqrs.len() {
            if board.square[knight_attack_sqrs[i] as usize] == (KNIGHT | attacker_color) {
                return true;
            }
        };
        for i in 0..2 {
            if precomp.num_sqrs_to_edge[attack_sqr as usize][precomp.pawn_attack_dirs[friendly_color_idx as usize][i] as usize] > 0 {
                let s = attack_sqr as i32 + precomp.direction_offsets[precomp.pawn_attack_dirs[friendly_color_idx as usize][i] as usize];
                let piece = board.square[s as usize];
                if piece == (PAWN | attacker_color) { return true; }
            }
        };

        return false;
    }
    fn init(&mut self, board: &Board) {
        self.moves.clear();

        self.white_to_move = board.color_to_move == WHITE;
        self.friendly_color = board.color_to_move;
        self.opponent_color = board.opponent_color;
        self.friendly_king_sqr = board.king_square[board.color_to_move_idx as usize];
        self.friendly_color_idx = if board.white_to_move { WHITE_INDEX } else { BLACK_INDEX };
        self.opponent_color_idx = 1 - self.friendly_color_idx;
    }
    fn gen_king_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        for i in 0..precomp.king_moves[self.friendly_king_sqr as usize].len() {
            let target_sqr = precomp.king_moves[self.friendly_king_sqr as usize][i];
            let piece_on_target = board.square[target_sqr as usize];

            if is_color(piece_on_target, self.friendly_color) { continue; }

            let is_capture = is_color(piece_on_target, self.opponent_color);
            if !is_capture && !self.gen_quiets { continue; }

            self.moves.push(Move::from_start_end(self.friendly_king_sqr, target_sqr as u32));
            
            if !is_capture && !self.sqr_attacked(board, precomp, self.friendly_king_sqr, self.opponent_color) {
                if (target_sqr == representation::F1 as u8 || target_sqr == representation::F8 as u8) && self.has_kingside_castle_right(board) {
                    if !self.sqr_attacked(board, precomp, target_sqr as u32, self.opponent_color) {
                        let castle_kingside_sqr = target_sqr + 1;
                        if board.square[castle_kingside_sqr as usize] == NONE {
                            self.moves.push(Move::from_start_end_flagged(self.friendly_king_sqr, castle_kingside_sqr as u32, CASTLING));
                        }
                    }
                }
                else if (target_sqr == representation::D1 as u8 || target_sqr == representation::D8 as u8) && self.has_queenside_castle_right(board) {
                    if !self.sqr_attacked(board, precomp, target_sqr as u32, self.opponent_color) {
                        let castle_queenside_sqr = target_sqr - 1;
                        if board.square[castle_queenside_sqr as usize] == NONE && board.square[castle_queenside_sqr as usize - 1] == NONE {
                            self.moves.push(Move::from_start_end_flagged(self.friendly_king_sqr, castle_queenside_sqr as u32, CASTLING));
                        }
                    }
                }
            }
        }
    }
    fn gen_sliding_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        let rooks = board.get_piece_list(ROOK, self.friendly_color_idx);
        for i in 0..rooks.count() {
            self.gen_sliding_piece_moves(board, precomp, rooks[i as usize], 0, 4);
        }
        let bishops = board.get_piece_list(BISHOP, self.friendly_color_idx);
        for i in 0..rooks.count() {
            self.gen_sliding_piece_moves(board, precomp, bishops[i as usize], 4, 8);
        }
        let queens = board.get_piece_list(QUEEN, self.friendly_color_idx);
        for i in 0..rooks.count() {
            self.gen_sliding_piece_moves(board, precomp, queens[i as usize], 0, 8);
        }
    }
    fn gen_sliding_piece_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>, start_sqr: u32, start_dir_idx: usize, end_dir_idx: usize) {
        for dir_idx in start_dir_idx..end_dir_idx {
            let cur_dir_offset = precomp.direction_offsets[dir_idx];
            for n in 0..precomp.num_sqrs_to_edge[start_sqr as usize][dir_idx] {
                let target_sqr = start_sqr as i32 + cur_dir_offset * (n + 1);
                let target_sqr_piece = board.square[target_sqr as usize];

                if is_color(target_sqr_piece, self.friendly_color) { break; }

                let is_capture = target_sqr_piece != NONE;
                if self.gen_quiets || is_capture {
                    self.moves.push(Move::from_start_end(start_sqr, target_sqr as u32));
                }
                if is_capture { break; }
            }
        }
    }
    fn gen_knight_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        let knights = board.get_piece_list(KNIGHT, self.friendly_color_idx);
        for i in 0..knights.count() {
            let start_sqr = knights[i as usize];
            for knight_move_idx in 0..precomp.knight_moves[start_sqr as usize].len() {
                let target_sqr = precomp.knight_moves[start_sqr as usize][knight_move_idx];
                let target_sqr_piece = board.square[target_sqr as usize];
                let is_capture = is_color(target_sqr_piece, self.opponent_color);
                if self.gen_quiets || is_capture {
                    if is_color(target_sqr_piece, self.friendly_color) { continue; }
                    self.moves.push(Move::from_start_end(start_sqr, target_sqr as u32));
                }
            }
        }
    }
    fn gen_pawn_moves(&mut self, board: &Board, precomp: &Res<PrecomputedMoveData>) {
        let pawns = board.get_piece_list(PAWN, self.friendly_color_idx);
        let pawn_offset = if self.friendly_color == WHITE { 8 } else { -8 };
        let start_rank = if self.white_to_move { 1 } else { 6 };
        let final_rank_before_prom = if self.white_to_move { 6 } else { 1 };

        let en_passant_file = ((board.current_game_state >> 4) & 15) as i32 - 1;
        let mut en_passant_sqr = -1;
        if en_passant_file != -1 {
            en_passant_sqr = 8 * (if self.white_to_move { 5 } else { 2 }) + en_passant_file;
        }

        for i in 0..pawns.count() {
            let start_sqr = pawns[i as usize];
            let rank = rank_idx(start_sqr);
            let one_step_from_prom = rank == final_rank_before_prom;

            if self.gen_quiets {
                let square_one_forward = start_sqr as i32 + pawn_offset;
                if board.square[square_one_forward as usize] == NONE {
                    if one_step_from_prom {
                        self.make_promotion_moves(start_sqr, square_one_forward as u32);
                    } else {
                        self.moves.push(Move::from_start_end(start_sqr, square_one_forward as u32));
                    }
                    if rank == start_rank {
                        let square_two_forward = square_one_forward + pawn_offset;
                        if board.square[square_two_forward as usize] == NONE {
                            self.moves.push(Move::from_start_end_flagged(start_sqr, square_two_forward as u32, PAWN_TWO_FORWARD));
                        }
                    }
                }
            }

            for j in 0..2 {
                if precomp.num_sqrs_to_edge[start_sqr as usize][precomp.pawn_attack_dirs[self.friendly_color_idx as usize][j] as usize] > 0 {
                    let pawn_capture_dir = precomp.direction_offsets[precomp.pawn_attack_dirs[self.friendly_color_idx as usize][j] as usize];
                    let target_sqr = start_sqr as i32 + pawn_capture_dir;
                    let target_piece = board.square[target_sqr as usize];

                    if is_color(target_piece, self.opponent_color) {
                        if one_step_from_prom {
                            self.make_promotion_moves(start_sqr, target_sqr as u32);
                        } else {
                            self.moves.push(Move::from_start_end(start_sqr, target_sqr as u32));
                        }
                    }
                    if target_sqr == en_passant_sqr {
                        let ep_captured_pawn_sqr = target_sqr + if self.white_to_move { -8 } else { 8 };
                        self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr as u32, EN_PASSANT_CAPTURE));
                    }
                }
            }
        }
    }

    fn make_promotion_moves(&mut self, start_sqr: u32, target_sqr: u32) {
        self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, QUEEN_PROMOTION));
        if self.gen_under_promotions {
            self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, KNIGHT_PROMOTION));
            self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, ROOK_PROMOTION));
            self.moves.push(Move::from_start_end_flagged(start_sqr, target_sqr, BISHOP_PROMOTION));
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
            friendly_color: WHITE,
            opponent_color: BLACK,
            friendly_king_sqr: 0,
            friendly_color_idx: WHITE_INDEX,
            opponent_color_idx: BLACK_INDEX,
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

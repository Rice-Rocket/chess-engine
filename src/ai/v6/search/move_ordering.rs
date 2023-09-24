use crate::{board::{moves::Move, board::Board, piece::Piece}, move_gen::bitboard::{utils::BitBoardUtils, bb::BitBoard}};
use super::super::evaluation::eval::Evaluation;

pub struct KillerMove {
    pub move_a: Move,
    pub move_b: Move,
}

impl KillerMove {
    pub const NULL: Self = Self { move_a: Move::NULL, move_b: Move::NULL };
    
    pub fn add(&mut self, mov: Move) {
        if mov.value() != self.move_a.value() {
            self.move_b = self.move_a;
            self.move_a = mov;
        }
    }
    pub fn match_move(&self, mov: Move) -> bool {
        mov.value() == self.move_a.value() || mov.value() == self.move_b.value()
    }
}

pub struct MoveOrdering {
    pub killers: [KillerMove; Self::MAX_KILLER_MOVE_PLY],
    pub history: [[[i32; 64]; 64]; 2],
    move_scores: [i32; Self::MAX_MOVE_COUNT],
}

impl MoveOrdering {
    pub const MAX_MOVE_COUNT: usize = 218;
    pub const MAX_KILLER_MOVE_PLY: usize = 32;

    // const SQUARE_CONTROLLED_BY_ENEMY_PAWN_PENALTY: i32 = 350;
    // const CAPTURED_PIECE_VALUE_MULTIPLIER: i32 = 100;
    const MILLION: i32 = 1000000;
    const HASH_MOVE_SCORE: i32 = 100 * Self::MILLION;
    const WINNING_CAPTURE_BIAS: i32 = 8 * Self::MILLION;
    const PROMOTE_BIAS: i32 = 6 * Self::MILLION;
    const KILLER_BIAS: i32 = 4 * Self::MILLION;
    const LOSING_CAPTURE_BIAS: i32 = 2 * Self::MILLION;
    const REGULAR_BIAS: i32 = 0;

    pub fn new() -> Self {
        Self {
            move_scores: [0; Self::MAX_MOVE_COUNT],
            killers: [KillerMove::NULL; Self::MAX_KILLER_MOVE_PLY],
            history: [[[0; 64]; 64]; 2],
        }
    }
    pub fn clear_history(&mut self) {
        self.history = [[[0; 64]; 64]; 2];
    }
    pub fn clear_killers(&mut self) {
        self.killers = [KillerMove::NULL; Self::MAX_KILLER_MOVE_PLY];
    }
    pub fn clear(&mut self) {
        self.clear_history();
        self.clear_killers();
    }

    pub fn order_moves(&mut self, hash_move: Move, moves: &mut Vec<Move>, board: &Board, _bbutils: &BitBoardUtils, opp_attacks: BitBoard, opp_pawn_attacks: BitBoard, in_q_search: bool, ply: usize) {
        // let opp_pieces = board.enemy_diagonal_sliders | board.enemy_orthogonal_sliders | board.piece_bitboards[Piece::new(Piece::KNIGHT | board.opponent_color).index()];
        // let pawn_attacks = if board.white_to_move { bbutils.white_pawn_attacks } else { bbutils.black_pawn_attacks };

        for i in 0..moves.len() {
            let mov = moves[i];

            // Highly favor best moves in previous iterations and shallower searches of iterative deepening
            if mov == hash_move {
                self.move_scores[i] = Self::HASH_MOVE_SCORE;
                continue;
            }

            let mut score = 0;
            let start_sqr = mov.start();
            let target_sqr = mov.target();

            let move_piece = board.square[start_sqr.index()];
            let move_ptype = move_piece.piece_type();
            let capture_ptype = board.square[target_sqr.index()].piece_type();
            let is_capture = capture_ptype != Piece::NONE;
            let flag = moves[i].move_flag();
            let piece_value = Self::get_piece_value(move_ptype);

            if is_capture {
                // Favor capturing higher value pieces with lower value pieces
                let capture_material_data = Self::get_piece_value(capture_ptype) - piece_value;
                let opp_can_recapture = (opp_pawn_attacks | opp_attacks).contains_square(target_sqr.square());

                // Punish moves where the opponent can recapture
                if opp_can_recapture {
                    score += (if capture_material_data >= 0 { Self::WINNING_CAPTURE_BIAS } else { Self::LOSING_CAPTURE_BIAS }) + capture_material_data;
                } else {
                    score += Self::WINNING_CAPTURE_BIAS + capture_material_data;
                }
            }
            
            if move_ptype == Piece::PAWN {
                // Favor promotions
                if flag == Move::QUEEN_PROMOTION && !is_capture {
                    score += Self::PROMOTE_BIAS;
                }
            } else if move_ptype != Piece::KING {
                // Punish moves that allow pieces to be captured by pawns severly
                if opp_pawn_attacks.contains_square(target_sqr.square()) {
                    score -= 50;
                }
                // Punish moves that allow pieces to be captured slightly
                else if opp_attacks.contains_square(target_sqr.square()) {
                    score -= 25;
                }
            }
            
            if !is_capture {
                let is_killer = !in_q_search && ply < Self::MAX_KILLER_MOVE_PLY && self.killers[ply].match_move(mov);
                score += if is_killer { Self::KILLER_BIAS } else { Self::REGULAR_BIAS };
                score += self.history[board.move_color_idx][start_sqr.index()][target_sqr.index()];
            }
            self.move_scores[i] = score;
        }

        // ! TODO: USE BETTER SORTING ALGORITHM (E.G. RADIX SORT, INTROSORT)
        Self::quicksort(moves, &mut self.move_scores, 0, moves.len() as isize - 1);
    }
    const fn get_piece_value(ptype: u8) -> i32 {
        match ptype {
            Piece::QUEEN => Evaluation::QUEEN_VALUE,
            Piece::ROOK => Evaluation::ROOK_VALUE,
            Piece::BISHOP => Evaluation::BISHOP_VALUE,
            Piece::KNIGHT => Evaluation::KNIGHT_VALUE,
            Piece::PAWN => Evaluation::PAWN_VALUE,
            _ => 0,
        }
    }
    fn quicksort(moves: &mut Vec<Move>, scores: &mut [i32; Self::MAX_MOVE_COUNT], low: isize, high: isize) {
        if low <= high {
            let pivot = Self::partition(moves, scores, low, high);
            Self::quicksort(moves, scores, low, pivot - 1);
            Self::quicksort(moves, scores, pivot + 1, high);
        }
    }
    fn partition(moves: &mut Vec<Move>, scores: &mut [i32; Self::MAX_MOVE_COUNT], low: isize, high: isize) -> isize {
        let pivot_score = scores[high as usize];
        let mut i = low - 1;

        for j in low..=(high - 1) {
            if scores[j as usize] > pivot_score {
                i += 1;
                moves.swap(i as usize, j as usize);
                scores.swap(i as usize, j as usize);
            }
        };
        moves.swap((i + 1) as usize, high as usize);
        scores.swap((i + 1) as usize, high as usize);

        return i + 1;
    }
}
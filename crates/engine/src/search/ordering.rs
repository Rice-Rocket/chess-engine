use crate::{board::{coord::Coord, moves::Move, piece::Piece, Board}, precomp::Precomputed, prelude::BitBoard};

pub struct MoveOrdering {
    pub killers: [KillerMoves; Self::MAX_KILLER_MOVE_DEPTH],
    pub history: [[[i32; 64]; 64]; 2],
}

impl MoveOrdering {
    pub const MAX_KILLER_MOVE_DEPTH: usize = 32;
    
    const CONTROLLED_BY_OPP_PAWN_PENALTY: i32 = 350;
    const CAPTURED_PIECE_MULTIPLIER: i32 = 100;

    const MILLION: i32 = 1000000;
    const FIRST_MOVE_SCORE: i32 = 100 * Self::MILLION;
    const WINNING_CAPTURE_BIAS: i32 = 8 * Self::MILLION;
    const PROMOTION_BIAS: i32 = 6 * Self::MILLION;
    const LOSING_CAPTURE_BIAS: i32 = 2 * Self::MILLION;
    const KILLER_BIAS: i32 = 4 * Self::MILLION;
    const NORMAL_BIAS: i32 = 0;

    pub fn new() -> Self {
        Self {
            killers: [KillerMoves::default(); Self::MAX_KILLER_MOVE_DEPTH],
            history: [[[0; 64]; 64]; 2],
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn order(
        &self,
        firstmove: Move,
        moves: &[Move],
        board: &Board,
        opp_attacks: BitBoard,
        opp_pawn_attacks: BitBoard,
        depth: u8,
        in_q_search: bool,
    ) -> Vec<Move> {
        let opps = board.enemy_diagonal_sliders | board.enemy_orthogonal_sliders 
            | board.piece_bitboards[Piece::new(Piece::KNIGHT | board.opponent_color)];
        let mut scores = Vec::with_capacity(moves.len());

        for m in moves {
            if *m == firstmove {
                scores.push(Self::FIRST_MOVE_SCORE);
                continue;
            }

            let mut score = 0;
            let start = m.start();
            let target = m.target();

            let move_piece = board.square[start];
            let move_ptype = move_piece.piece_type();
            let capture_ptype = board.square[target].piece_type();
            let is_capture = capture_ptype != Piece::NONE;
            let flag = m.move_flag();
            let piece_value = Self::piece_value_score(move_ptype);

            if is_capture {
                let material_diff = Self::piece_value_score(capture_ptype) - piece_value;
                let opp_can_recapture = (opp_attacks | opp_pawn_attacks).contains_square(target.square());

                if opp_can_recapture {
                    score += if material_diff >= 0 { Self::WINNING_CAPTURE_BIAS } else { Self::LOSING_CAPTURE_BIAS } + material_diff;
                } else {
                    score += Self::WINNING_CAPTURE_BIAS + material_diff;
                }
            }

            if move_ptype == Piece::PAWN {
                if flag == Move::QUEEN_PROMOTION && !is_capture {
                    score += Self::PROMOTION_BIAS;
                }
            } else if move_ptype != Piece::KING {
                let to_score = Self::psqt_score(move_piece, target);
                let from_score = Self::psqt_score(move_piece, start);
                score += to_score - from_score;

                if opp_pawn_attacks.contains_square(target.square()) {
                    score -= 50;
                } else if opp_attacks.contains_square(target.square()) {
                    score -= 25;
                }
            }

            if !is_capture {
                let is_killer = !in_q_search && depth < Self::MAX_KILLER_MOVE_DEPTH as u8 && self.killers[depth as usize].matches(*m);
                score += if is_killer { Self::KILLER_BIAS } else { Self::NORMAL_BIAS };
                score += self.history[board.move_color_idx][m.start()][m.target()];
            }

            scores.push(score);
        }

        let mut moves_scores: Vec<_> = moves.iter().zip(scores).collect();
        moves_scores.sort_unstable_by_key(|(_, v)| -*v);

        moves_scores.into_iter().map(|(m, _)| *m).collect()
    }


    pub fn piece_value_score(ptype: u8) -> i32 {
        match ptype {
            Piece::PAWN => 100,
            Piece::KNIGHT => 300,
            Piece::BISHOP => 320,
            Piece::ROOK => 500,
            Piece::QUEEN => 900,
            _ => 0
        }
    }

    fn psqt_score(piece: Piece, mut square: Coord) -> i32 {
        if piece.color() == Piece::WHITE {
            square = square.flip_rank();
        }

        match piece.piece_type() {
            Piece::PAWN => Self::PSQT_PAWNS[square],
            Piece::KNIGHT => Self::PSQT_KNIGHTS[square],
            Piece::BISHOP => Self::PSQT_BISHOPS[square],
            Piece::ROOK=> Self::PSQT_ROOKS[square],
            Piece::QUEEN => Self::PSQT_QUEENS[square],
            Piece::KING => Self::PSQT_KINGS[square],
            _ => 0
        }
    }


    const PSQT_PAWNS: [i32; 64] = [
        0,   0,   0,   0,   0,   0,   0,   0,
        50,  50,  50,  50,  50,  50,  50,  50,
        10,  10,  20,  30,  30,  20,  10,  10,
        5,   5,  10,  25,  25,  10,   5,   5,
        0,   0,   0,  20,  20,   0,   0,   0,
        5,  -5, -10,   0,   0, -10,  -5,   5,
        5,  10,  10, -20, -20,  10,  10,   5,
        0,   0,   0,   0,   0,   0,   0,   0
    ];

    const PSQT_KNIGHTS: [i32; 64] = [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ];

    const PSQT_BISHOPS: [i32; 64] = [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];

    const PSQT_ROOKS: [i32; 64] = [
        0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0
    ];

    const PSQT_QUEENS: [i32; 64] = [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,   0,  5,  5,  5,  5,  0, -5,
        0,    0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ];

    const PSQT_KINGS: [i32; 64] = [
        -80, -70, -70, -70, -70, -70, -70, -80, 
        -60, -60, -60, -60, -60, -60, -60, -60, 
        -40, -50, -50, -60, -60, -50, -50, -40, 
        -30, -40, -40, -50, -50, -40, -40, -30, 
        -20, -30, -30, -40, -40, -30, -30, -20, 
        -10, -20, -20, -20, -20, -20, -20, -10, 
        20,  20,  -5,  -5,  -5,  -5,  20,  20, 
        20,  30,  10,   0,   0,  10,  30,  20
    ];
}

impl Default for MoveOrdering {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Clone, Copy)]
pub struct KillerMoves {
    pub a: Move,
    pub b: Move,
}

impl KillerMoves {
    pub fn add(&mut self, m: Move) {
        if m.value() != self.a.value() {
            self.b = self.a;
            self.a = m;
        }
    }

    pub fn matches(&self, m: Move) -> bool {
        m.value() == self.a.value() || m.value() == self.b.value()
    }
}

impl Default for KillerMoves {
    fn default() -> Self {
        Self {
            a: Move::NULL,
            b: Move::NULL,
        }
    }
}

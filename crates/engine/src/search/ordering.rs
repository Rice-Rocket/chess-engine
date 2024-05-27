use crate::{board::{coord::Coord, moves::Move, piece::Piece, Board}, precomp::Precomputed, prelude::BitBoard};

const CONTROLLED_BY_OPP_PAWN_PENALTY: i32 = 350;
const CAPTURED_PIECE_MULTIPLIER: i32 = 100;

const MILLION: i32 = 1000000;
const FIRST_MOVE_SCORE: i32 = 100 * MILLION;
const WINNING_CAPTURE_BIAS: i32 = 8 * MILLION;
const PROMOTION_BIAS: i32 = 6 * MILLION;
const LOSING_CAPTURE_BIAS: i32 = 2 * MILLION;

pub fn order(firstmove: Move, moves: &[Move], board: &Board, opp_attacks: BitBoard, opp_pawn_attacks: BitBoard, depth: u8) -> Vec<Move> {
    let opps = board.enemy_diagonal_sliders | board.enemy_orthogonal_sliders 
        | board.piece_bitboards[Piece::new(Piece::KNIGHT | board.opponent_color)];
    let mut scores = Vec::with_capacity(moves.len());

    for m in moves.iter() {
        if *m == firstmove {
            scores.push(FIRST_MOVE_SCORE);
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
        let piece_value = piece_value_score(move_ptype);

        if is_capture {
            let material_diff = piece_value_score(capture_ptype) - piece_value;
            let opp_can_recapture = (opp_attacks | opp_pawn_attacks).contains_square(target.square());

            if opp_can_recapture {
                score += if material_diff >= 0 { WINNING_CAPTURE_BIAS } else { LOSING_CAPTURE_BIAS } + material_diff;
            } else {
                score += WINNING_CAPTURE_BIAS + material_diff;
            }
        }

        if move_ptype == Piece::PAWN {
            if flag == Move::QUEEN_PROMOTION && !is_capture {
                score += PROMOTION_BIAS;
            }
        } else if move_ptype != Piece::KING {
            let to_score = psqt_score(move_piece, target);
            let from_score = psqt_score(move_piece, start);
            score += to_score - from_score;

            if opp_pawn_attacks.contains_square(target.square()) {
                score -= 50;
            } else if opp_attacks.contains_square(target.square()) {
                score -= 25;
            }
        }

        scores.push(score);
    }

    let mut moves_scores: Vec<_> = moves.iter().zip(scores).collect();
    moves_scores.sort_unstable_by_key(|(_, v)| -*v);
    
    moves_scores.into_iter().map(|(m, _)| *m).collect()
}


fn piece_value_score(ptype: u8) -> i32 {
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
        Piece::PAWN => PSQT_PAWNS[square],
        Piece::KNIGHT => PSQT_KNIGHTS[square],
        Piece::BISHOP => PSQT_BISHOPS[square],
        Piece::ROOK=> PSQT_ROOKS[square],
        Piece::QUEEN => PSQT_QUEENS[square],
        Piece::KING => PSQT_KINGS[square],
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

use crate::board::{coord::Coord, piece::Piece};
use crate::move_gen::bitboard::bb::BitBoard;
use crate::move_gen::bitboard::utils::BitBoardUtils;
use super::attack::{rook_xray_attacks_sqr, queen_attacks_sqr, knight_attacks_sqr, bishop_xray_attacks_sqr};
use super::king::enemy_blockers_for_king;
use super::{perspective::Perspective, king::enemy_blockers_for_king_sqr};
use super::pos::PositionEvaluation;


const MOBILITY_BONUS_MG: [[i32; 28]; 4] = [
    [-62,-53,-12,-4,3,13,22,28,33,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-48,-20,16,26,38,51,55,63,63,68,81,81,91,98,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-60,-20,2,3,3,11,22,31,40,40,41,48,57,57,62,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-30,-12,-8,-9,20,23,23,35,38,53,64,65,65,66,67,67,72,72,77,79,93,108,108,108,110,114,114,116]
];
const MOBILITY_BONUS_EG: [[i32; 28]; 4] = [
    [-81,-56,-31,-16,5,11,17,20,25,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-59,-23,-3,13,24,42,54,57,65,73,78,86,88,97,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-78,-17,23,39,70,99,103,121,134,139,158,164,168,169,172,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [-48,-30,-7,19,40,55,59,75,78,96,96,100,121,127,131,133,136,141,147,150,151,168,168,171,182,182,192,219]
];
pub fn mobility(pos: &PositionEvaluation, per: Perspective, mg: bool) -> i32 {
    let mut sum = 0;
    let bonus = if mg { MOBILITY_BONUS_MG } else { MOBILITY_BONUS_EG };
    let mut friendly_pieces = pos.friendly_color_bb(per);
    let mobility_area = mobility_area(pos, per);

    while friendly_pieces.0 != 0 {
        let sqr = friendly_pieces.pop_lsb();
        let c = Coord::from_idx(sqr as i8);
        let ptype = pos.square(c).piece_type();

        if ptype == Piece::KNIGHT {
            let knight_attacks = pos.attack_data().knight_attacks[c.index()];
            // Only count attack squares that land in the mobility area
            let v = (knight_attacks & mobility_area).count();
            sum += bonus[0][v as usize];
        }
        if ptype == Piece::BISHOP {
            let bishop_attacks = pos.attack_data().bishop_attacks[c.index()];
            let v = (bishop_attacks & mobility_area).count();
            sum += bonus[1][v as usize];
        }
        if ptype == Piece::ROOK {
            let rook_attacks = pos.attack_data().rook_attacks[c.index()];
            let v = (rook_attacks & mobility_area).count();
            sum += bonus[2][v as usize];
        }
        if ptype == Piece::QUEEN {
            let queen_attacks = pos.attack_data().queen_attacks[c.index()];
            let v = (queen_attacks & mobility_area).count();
            sum += bonus[3][v as usize];
        }
    };

    sum
}

pub fn mobility_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> i32 {
    let mut v = 0;
    let piece = pos.square(sqr);
    let ptype = piece.piece_type();
    if !per.is_color(piece.color()) { return 0; }
    for sqr2 in Coord::iterate_squares() {
        if mobility_area_sqr(pos, per, sqr2) == 0 { continue; }
        let has_queen = pos.square(sqr2) == per.friendly_piece(Piece::QUEEN);
        if ptype == Piece::KNIGHT && knight_attacks_sqr(pos, per, sqr2, Some(sqr)) > 0 && !has_queen { v += 1 };
        if ptype == Piece::BISHOP && bishop_xray_attacks_sqr(pos, per, sqr2, Some(sqr)) > 0 && !has_queen { v += 1 };
        if ptype == Piece::ROOK && rook_xray_attacks_sqr(pos, per, sqr2, Some(sqr)) > 0 { v += 1 };
        if ptype == Piece::QUEEN && queen_attacks_sqr(pos, per, sqr2, Some(sqr)) > 0 { v += 1 };
    };
    v
}


pub fn mobility_area(pos: &PositionEvaluation, per: Perspective) -> BitBoard {
    let close_ranks = BitBoard::from_rank(per.get_rank(1)) | BitBoard::from_rank(per.get_rank(2));
    // Find pawns that are within the close ranks or are blocked (that is they have pieces in front of them)
    let close_blocked_pawns = pos.friendly_piece_bb(per, Piece::PAWN) & (per.shift_down(pos.all_pieces_bb()) | close_ranks);
    
    // Exclude friendly queen from mobility area
    // ! Probably will not do this later and only exclude friendly queen from minor piece's (knights and bishops) mobility area
    let mut not_mobility_area = close_blocked_pawns | pos.friendly_piece_bb(per, Piece::QUEEN);
    // Exclude friendly king from mobility area
    not_mobility_area |= pos.friendly_king_sqr(per).to_bitboard();
    // Exclude squares attacked by enemy pawns from mobility area
    not_mobility_area |= BitBoardUtils::pawn_attacks(pos.enemy_piece_bb(per, Piece::PAWN), !per.is_white());
    // Exclude pinned pieces from mobility area
    not_mobility_area |= enemy_blockers_for_king(pos, per.other());

    return !not_mobility_area;
}

pub fn mobility_area_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> i32 {
    if pos.square(sqr) == per.friendly_piece(Piece::KING) { return 0; }
    if pos.square(sqr) == per.friendly_piece(Piece::QUEEN) { return 0; }
    if pos.square(Coord::new(sqr.file() - 1, per.rank_farther_by(sqr.rank(), 1))) == per.enemy_piece(Piece::PAWN) { return 0; }
    if pos.square(Coord::new(sqr.file() + 1, per.rank_farther_by(sqr.rank(), 1))) == per.enemy_piece(Piece::PAWN) { return 0; }
    if pos.square(sqr) == per.friendly_piece(Piece::PAWN) && 
        (per.rank_is_close_half(per.rank_farther_by(sqr.rank(), 1)) || pos.square(Coord::new(sqr.file(), per.rank_farther_by(sqr.rank(), 1))).piece_type() != Piece::NONE) { return 0; }
    if enemy_blockers_for_king_sqr(pos, per.other(), sqr) > 0 { return 0; }
    return 1;
}
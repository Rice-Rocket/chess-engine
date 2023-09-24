use crate::board::piece::Piece;

use super::pos::PositionEvaluation;


// Determines if there are two bishops of opposite colors
pub fn opposite_bishops(pos: &PositionEvaluation) -> bool {
    let mut color = [0, 0];
    let lst_1 = pos.board.get_piece_list(Piece::new(Piece::BLACK_BISHOP));
    if lst_1.count() == 1 {
        let c1 = lst_1.occupied_squares[0];
        color[0] = (c1.file() + c1.rank()) % 2;
    } else { return false; }
    let lst_2 = pos.board.get_piece_list(Piece::new(Piece::WHITE_BISHOP));
    if lst_2.count() == 1 {
        let c2 = lst_2.occupied_squares[0];
        color[1] = (c2.file() + c2.rank()) % 2;
    } else { return false; }
    return if color[0] == color[1] { false } else { true };
}
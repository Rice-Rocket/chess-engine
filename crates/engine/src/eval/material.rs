use crate::board::coord::Coord;
use super::state::State;

pub fn non_pawn_material(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn piece_value_bonus(state: &State, mg: bool, sqr: Coord) -> i32 {
    todo!();
}

pub fn psqt_bonus(state: &State, mg: bool, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn piece_value_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn piece_value_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn psqt_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn psqt_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::eval::state::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_non_pawn_material() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(non_pawn_material, 11335, 11577, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_piece_value_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(piece_value_bonus, 12203, 12197, state; true);
        assert_eval!(piece_value_bonus, 13630, 13485, state; false);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_psqt_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, &magics, Color::White);

        assert_eval!(psqt_bonus, 146, 32, state; true);
        assert_eval!(psqt_bonus, -126, 26, state; false);
    }
}

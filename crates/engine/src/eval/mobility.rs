use crate::board::coord::Coord;
use super::state::State;

pub fn mobility(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn mobility_area(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn mobility_bonus(state: &State, mg: bool, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn mobility_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

#[inline]
pub fn mobility_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::eval::state::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(mobility, 41, 48, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility_area() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(mobility_area, 49, 47, state);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_mobility_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(mobility_bonus, 193, 158, state; true);
        assert_eval!(mobility_bonus, 467, 293, state; false);
    }
}

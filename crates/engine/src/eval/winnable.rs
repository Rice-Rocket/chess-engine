use crate::board::coord::Coord;
use super::state::State;

pub fn winnable(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn winnable_total_mg(state: &State, v: i32) -> i32 {
    todo!();
}

pub fn winnable_total_eg(state: &State, v: i32) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::eval::state::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_winnable() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut movegen = MoveGenerator::default();
        movegen.generate_moves(&board, &precomp, &magics, false);
        let mut state = State::new(&board, &precomp, &movegen, Color::White);

        assert_eval!(winnable, 58, 58, state);
    }
}

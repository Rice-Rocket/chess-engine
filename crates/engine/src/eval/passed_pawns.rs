use crate::board::coord::Coord;
use super::state::State;

pub fn candidate_passed(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_proximity(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn passed_block(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn passed_file(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn passed_rank(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn passed_leverable(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn passed_mg(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn passed_eg(state: &State, sqr: Coord) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, color::Color, board::{Board, zobrist::Zobrist}};
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_candidate_passed() {
        assert_eval!(candidate_passed, 2, 1, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_proximity() {
        assert_eval!(king_proximity, -18, -7, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_passed_block() {
        assert_eval!(passed_block, 10, 35, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_passed_file() {
        assert_eval!(passed_file, 3, 1, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_passed_rank() {
        assert_eval!(passed_rank, 5, 4, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_passed_leverable() {
        assert_eval!(passed_leverable, 2, 1, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_passed_mg() {
        assert_eval!(passed_mg, 9, 86, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_passed_eg() {
        assert_eval!(passed_eg, 42, 92, "1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }
}

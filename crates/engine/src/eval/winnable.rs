use crate::board::coord::Coord;
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn winnable(&self, sqr: Coord) -> i32 {
        todo!();
    }

    pub fn winnable_total_mg(&self, v: i32) -> i32 {
        todo!();
    }

    pub fn winnable_total_eg(&self, v: i32) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_winnable() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(winnable, 58, 58, eval);
    }
}

use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::{SquareEvaluations, SquareValues}, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, precomp::Precomputed, prelude::BitBoard, sum_sqrs};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn isolated<W: Color, B: Color>(&self) -> BitBoard {
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let mut isolated = BitBoard(0);

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let adj_files = Precomputed::adjacent_file_mask(sqr.file() as usize);
            if (adj_files & self.board.piece_bitboards[W::piece(Piece::PAWN)]).0 == 0 {
                isolated |= sqr.to_bitboard();
            }
        }
        
        isolated
    }

    pub fn opposed<W: Color, B: Color>(&self) -> BitBoard {
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let mut opposed = BitBoard(0);

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let forward = Precomputed::forward_files(W::index(), sqr);
            if (forward & self.board.piece_bitboards[B::piece(Piece::PAWN)]).0 != 0 {
                opposed |= sqr.to_bitboard();
            }
        }

        opposed
    }

    pub fn phalanx<W: Color, B: Color>(&self) -> BitBoard {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        pawns & (pawns.shifted_2d(Coord::new(-1, 0)) | pawns.shifted_2d(Coord::new(1, 0)))
    }

    /// Returns `(supported once or more, supported twice)`
    pub fn supported<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let attacks = self.all_pawn_attacks[W::index()];
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        (pawns & attacks.0, pawns & attacks.1)
    }

    pub fn backward<W: Color, B: Color>(&self) -> BitBoard {
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let mut backward = BitBoard(0);

        let opp_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        pawns &= opp_pawns.shifted_2d(W::offset(-1, -2))
            | opp_pawns.shifted_2d(W::offset(1, -2))
            | opp_pawns.shifted_2d(W::offset(0, -1));

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let span = Precomputed::pawn_attack_span(B::index(), sqr + W::up());
            if (span & self.board.piece_bitboards[W::piece(Piece::PAWN)]).0 == 0 {
                backward |= sqr.to_bitboard();
            }
        }

        backward
    }

    pub fn doubled<W: Color, B: Color>(&self) -> BitBoard {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        pawns & pawns.shifted_2d(W::offset(0, 1)) & !pawns.shifted_2d(W::offset(-1, 1)) & !pawns.shifted_2d(W::offset(1, 1))
    }

    pub fn connected<W: Color, B: Color>(&self) -> BitBoard {
        self.supported[W::index()].0 | self.phalanx[W::index()]
    }

    const CONNECTED_BONUS_SEED: [i32; 7] = [0, 7, 8, 12, 29, 48, 86];
    pub fn connected_bonus<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut evals = SquareEvaluations::new();

        let mut connected = self.connected::<W, B>();
        connected &= BitBoard::from_ranks(W::ranks(1..=6));

        let opposed = self.opposed[W::index()];
        let phalanx = self.phalanx[W::index()];
        let supported = self.supported[W::index()];
        let blocked = self.board.piece_bitboards[W::piece(Piece::PAWN)] 
            & self.board.piece_bitboards[B::piece(Piece::PAWN)].shifted_2d(W::down());

        while connected.0 != 0 {
            let sqr = Coord::from_idx(connected.pop_lsb() as i8);
            let rank = W::rank(sqr.rank());

            evals[sqr] = Self::CONNECTED_BONUS_SEED[rank as usize]
                * (2 + phalanx.square_value(sqr.square()) - opposed.square_value(sqr.square()))
                + 21 * (supported.0.square_value(sqr.square()) + supported.1.square_value(sqr.square()));
        }

        evals
    }

    pub fn weak_unopposed_pawn<W: Color, B: Color>(&self) -> BitBoard {
        let unopposed = !self.opposed[W::index()];
        unopposed & (self.isolated[W::index()] | self.backward[W::index()])
    }

    pub fn weak_lever<W: Color, B: Color>(&self) -> BitBoard {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        pawns & enemy.shifted_2d(W::offset(-1, -1)) & enemy.shifted_2d(W::offset(1, -1))
            & !pawns.shifted_2d(W::offset(-1, 1)) & !pawns.shifted_2d(W::offset(1, 1))
    }

    /// Computes blocked pawns on the fifth and sixth ranks.
    ///
    /// Returns `(on fifth rank, on sixth rank)`
    pub fn blocked<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let blockers = self.board.piece_bitboards[B::piece(Piece::PAWN)].shifted_2d(W::down());
        (pawns & blockers & BitBoard::RANKS[W::rank(4) as usize],
        pawns & blockers & BitBoard::RANKS[W::rank(5) as usize])
    }

    pub fn doubled_isolated<W: Color, B: Color>(&self) -> BitBoard {
        let friendly_pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let mut pawns = self.isolated[W::index()];
        let mut doubled_isolated = BitBoard(0);

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let behind_pawns = Precomputed::forward_files(B::index(), sqr) & friendly_pawns;
            let opposers = Precomputed::forward_files(W::index(), sqr) & enemy_pawns;
            let adjacent = Precomputed::adjacent_file_mask(sqr.file() as usize) & enemy_pawns;
            
            if behind_pawns.count() > 0 && adjacent.count() == 0 && opposers.count() > 0 {
                doubled_isolated |= sqr.to_bitboard();
            }
        }

        doubled_isolated
    }

    /// Returns `(mg, eg)`
    pub fn pawns<W: Color, B: Color>(&self) -> (i32, i32) {
        let mut mg = 0;
        let mut eg = 0;

        let doubled_isolated = self.doubled_isolated::<W, B>();
        let mut isolated = self.isolated[W::index()];
        let mut backward = self.backward[W::index()];

        isolated &= !doubled_isolated;
        backward &= !(doubled_isolated | isolated);

        mg -= 11 * doubled_isolated.count() as i32;
        mg -= 5 * isolated.count() as i32;
        mg -= 9 * backward.count() as i32;
        eg -= 56 * doubled_isolated.count() as i32;
        eg -= 15 * isolated.count() as i32;
        eg -= 24 * backward.count() as i32;

        let doubled = self.doubled::<W, B>().count() as i32;
        let connected_bonus = self.connected_bonus::<W, B>();
        mg -= 11 * doubled;
        mg += connected_bonus.count();
        eg -= 56 * doubled;
        eg += connected_bonus.zip(self.connected_rank_bonus::<W>()).map(|(b, r)| (b as f32 * r) as i32).count();

        let weak_unopposed = self.weak_unopposed_pawn::<W, B>();
        mg -= 13 * weak_unopposed.count() as i32;
        eg -= 27 * weak_unopposed.count() as i32;

        let blocked = self.blocked::<W, B>();
        mg -= 11 * blocked.0.count() as i32;
        mg -= 3 * blocked.1.count() as i32;
        eg -= 4 * blocked.0.count() as i32;
        eg += 4 * blocked.1.count() as i32;

        eg -= 56 * self.weak_lever::<W, B>().count() as i32;

        (mg, eg)
    }

    fn connected_rank_bonus<W: Color>(&self) -> SquareValues<f32> {
        let mut eval = SquareValues::new();

        for sqr in Coord::iter_squares() {
            eval[sqr] = (W::rank(sqr.rank()) as i32 - 2) as f32 / 4.0;
        }

        eval
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("K7/Pn3P2/p2np3/r3P1P1/1RpN2k1/B1q2p2/1Pp3P1/1R1b4 w - - 0 1")]
    fn test_isolated() {
        assert_eval!(+ - isolated, 0, 3, eval);
    }

    #[test]
    #[evaluation_test("K7/Pn3P2/p2np3/r3P1P1/1RpN2k1/B1q2p2/1Pp3P1/1R1b4 w - - 0 1")]
    fn test_opposed() {
        assert_eval!(+ - opposed, 1, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QPP2n1P/P4P2/2B1N1RK w kq - 9 6")]
    fn test_phalanx() {
        assert_eval!(+ - phalanx, 2, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ2P1qB/1b1P1Ppr/QP3n1P/P4P2/2B1N1RK w kq - 9 6")]
    fn test_supported() {
        assert_eval!(* - [0, 1] supported, (2, 1), (2, 0), eval);
    }

    #[test]
    #[evaluation_test("K7/Pn3P2/p2np3/r3P1P1/1RpN2k1/B1q2p2/1Pp3P1/1R1b4 w - - 0 1")]
    fn test_backward() {
        assert_eval!(+ - backward, 2, 2, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7")]
    fn test_doubled() {
        assert_eval!(+ - doubled, 1, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7")]
    fn test_connected() {
        assert_eval!(+ - connected, 1, 3, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    // 1r3q1R/2n1n2n/pp1k1pR1/pQ3P1B/1b1P1qpr/QP1P1n1P/P4P2/2B1N1RK w kq - 1 7
    // 29, 65
    fn test_connected_bonus() {
        assert_eval!(+ - connected_bonus, 91, 0, eval);
    }

    #[test]
    #[evaluation_test("K7/Pn3P2/p2np3/r3P1P1/1RpN2k1/B1q2p2/1Pp3P1/1R1b4 w - - 0 1")]
    fn test_weak_unopposed_pawn() {
        assert_eval!(+ - weak_unopposed_pawn, 1, 3, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n1n2n/pp1k1pRp/pQP3PB/1b2Pq1r/Q4n1P/P1P2P2/2B1N1RK w kq - 1 7")]
    fn test_weak_lever() {
        assert_eval!(+ - weak_lever, 1, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/1Qp2PPB/1bP2q1r/p4n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_blocked() {
        assert_eval!(* - [0, 1] blocked, (1, 0), (0, 1), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_doubled_isolated() {
        assert_eval!(+ - doubled_isolated, 1, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_pawns() {
        assert_eval!(- pawns, (113, -74), (-42, -172), eval);
    }
}

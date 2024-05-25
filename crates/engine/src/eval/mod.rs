use crate::{board::{coord::Coord, piece::Piece, Board}, color::{Black, Color, White}, move_gen::magics::MagicBitBoards, precomp::PrecomputedData, prelude::BitBoard};

pub mod attack;
pub mod utils;
pub mod imbalance;
pub mod king;
pub mod material;
pub mod mobility;
pub mod passed_pawns;
pub mod pawns;
pub mod pieces;
pub mod space;
pub mod threats;
pub mod winnable;
pub mod macros;


pub struct Evaluation<'a> {
    pub board: &'a Board,
    pub precomp: &'a PrecomputedData,
    pub magics: &'a MagicBitBoards,

    pin_rays: [(BitBoard, BitBoard); 2],
    king_ring: [BitBoard; 2],
}

impl<'a> Evaluation<'a> {
    pub fn new(
        board: &'a Board,
        precomp: &'a PrecomputedData,
        magics: &'a MagicBitBoards,
    ) -> Self {
        Self {
            board,
            precomp,
            magics,

            pin_rays: [(BitBoard(0), BitBoard(0)); 2],
            king_ring: [BitBoard(0); 2],
        }
    }

    pub fn init<W: Color, B: Color>(&mut self) {
        self.pin_rays[W::index()] = self.pin_rays::<W, B>();
        self.pin_rays[B::index()] = self.pin_rays::<B, W>();
        self.king_ring[W::index()] = self.king_ring::<W, B>(false);
        self.king_ring[B::index()] = self.king_ring::<B, W>(false);
    }

    /// Evaluation function adapted from the [Stockfish Evaluation Guide](https://hxim.github.io/Stockfish-Evaluation-Guide/).
    ///
    /// Parameterized by <Friendly, Enemy>
    ///
    /// Notes to self: 
    ///
    /// - Capital letters represent white pieces.
    /// - The ranks are inverted, meaning when it says `y + 1`, it should be `Color::down()`.
    /// - Castling rights go in order: [white kingside, white queenside, black kingside, black
    /// queenside]
    pub fn evaluate<W: Color, B: Color>(&mut self) -> i32 {
        self.init::<W, B>();

        let mg = self.middle_game_eval::<W, B>() as f32;
        let mut eg = self.end_game_eval::<W, B>() as f32;
        let p = self.phase::<W, B>() as f32;
        let rule50 = self.rule50() as f32;

        let sf = if eg as i32 > 0 {
            self.scale_factor::<W, B>()
        } else {
            self.scale_factor::<B, W>()
        };

        eg = eg * sf as f32 / 64.0;
        let mut v = (((mg * p + ((eg * (128.0 - p)).trunc())) / 128.0).trunc());
        v = ((v / 16.0).trunc()) * 16.0;
        v += self.tempo::<W>() as f32;
        v = (v * (100.0 - rule50) / 100.0).trunc();

        v as i32
    }
}


impl<'a> Evaluation<'a> {
    fn middle_game_eval<W: Color, B: Color>(&self) -> i32 {
        let mut v = 0;

        v += self.piece_value_mg::<W, B>() - self.piece_value_mg::<B, W>();
        v += self.psqt_mg::<W, B>() - self.psqt_mg::<B, W>();
        v += self.imbalance_total::<W, B>();
        v += self.pawns_mg::<W, B>() - self.pawns_mg::<B, W>();
        v += self.pieces_mg::<W, B>() - self.pieces_mg::<B, W>();
        v += self.mobility_mg::<W, B>() - self.mobility_mg::<B, W>();
        v += self.threats_mg::<W, B>() - self.threats_mg::<B, W>();
        v += self.passed_mg::<W, B>() - self.passed_mg::<B, W>();
        v += self.space::<W, B>() - self.space::<B, W>();
        v += self.king_mg::<W, B>() - self.king_mg::<B, W>();
        v += self.winnable_total_mg::<W, B>(v);

        v
    }

    fn end_game_eval<W: Color, B: Color>(&self) -> i32 {
        let mut v = 0;

        v += self.piece_value_eg::<W, B>() - self.piece_value_eg::<B, W>();
        v += self.psqt_eg::<W, B>() - self.psqt_eg::<B, W>();
        v += self.imbalance_total::<W, B>();
        v += self.pawns_eg::<W, B>() - self.pawns_eg::<B, W>();
        v += self.pieces_eg::<W, B>() - self.pieces_eg::<B, W>();
        v += self.mobility_eg::<W, B>() - self.mobility_eg::<B, W>();
        v += self.threats_eg::<W, B>() - self.threats_eg::<B, W>();
        v += self.passed_eg::<W, B>() - self.passed_eg::<B, W>();
        v += self.king_eg::<W, B>() - self.king_eg::<B, W>();
        v += self.winnable_total_eg::<W, B>(v);

        v
    }

    const PHASE_MG_LIMIT: i32 = 15258;
    const PHASE_EG_LIMIT: i32 = 3915;
    fn phase<W: Color, B: Color>(&self) -> i32 {
        let npm = (self.non_pawn_material::<W, B>() + self.non_pawn_material::<B, W>())
            .clamp(Self::PHASE_EG_LIMIT, Self::PHASE_MG_LIMIT);
        ((npm - Self::PHASE_EG_LIMIT) * 128) / (Self::PHASE_MG_LIMIT - Self::PHASE_EG_LIMIT)
    }

    fn rule50(&self) -> i32 {
        self.board.current_state.fifty_move_counter as i32
    }

    const BISHOP_VALUE_MG: i32 = 825;
    const BISHOP_VALUE_EG: i32 = 915;
    const ROOK_VALUE_MG: i32 = 1276;
    fn scale_factor<W: Color, B: Color>(&self) -> i32 {
        let mut sf = 64;

        let (pc_w, pc_b) = (self.pawn_count::<W, B>(), self.pawn_count::<B, W>());
        let (qc_w, qc_b) = (self.queen_count::<W, B>(), self.queen_count::<B, W>());
        let (bc_w, bc_b) = (self.bishop_count::<W, B>(), self.bishop_count::<B, W>());
        let (nc_w, nc_b) = (self.knight_count::<W, B>(), self.knight_count::<B, W>());
        let (npm_w, npm_b) = (self.non_pawn_material::<W, B>(), self.non_pawn_material::<B, W>());

        if pc_w == 0 && npm_w - npm_b <= Self::BISHOP_VALUE_MG {
            sf = if npm_w < Self::ROOK_VALUE_MG { 0 } else if npm_b <= Self::BISHOP_VALUE_MG { 4 } else { 14 };
        }

        if sf == 64 {
            let ob = self.opposite_bishops();
            if ob && npm_w == Self::BISHOP_VALUE_MG && npm_b == Self::BISHOP_VALUE_MG {
                sf = 22 + 4 * self.candidate_passed::<W, B>().count() as i32;
            } else if ob {
                sf = 22 + 3 * self.piece_count::<W, B>();
            } else {
                if npm_w == Self::ROOK_VALUE_MG && npm_b == Self::ROOK_VALUE_MG && pc_w - pc_b <= 1 {
                    let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
                    let left_flank = (BitBoard::from_ranks(0..=3) & pawns).0 > 0;
                    let right_flank = (BitBoard::from_ranks(4..=7) & pawns).0 > 0;
                    let pawnking_b = (self.precomp.king_moves[self.king_square::<B, W>()] 
                        & self.board.piece_bitboards[B::piece(Piece::PAWN)]).0 > 0;
                    if left_flank != right_flank && pawnking_b { return 36 };
                }

                if qc_w + qc_b == 1 {
                    sf = 37 + 3 * (if qc_w == 1 { bc_b + nc_b } else { bc_w + nc_w });
                } else {
                    sf = sf.min(36 + 7 * pc_w);
                }
            }
        }

        sf
    }

    fn tempo<W: Color>(&self) -> i32 {
        if W::is_white() == self.board.white_to_move {
            28
        } else {
            -28
        }
    }
}


// TODO: Add further testing
// http://bernd.bplaced.net/fengenerator/fengenerator.html
// for random chess positions.
#[cfg(test)]
pub mod tests {
    use super::test_prelude::*;
    use proc_macro_utils::evaluation_test;

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_eval_0() {
        assert_eq!(eval.evaluate::<White, Black>(), -1081)
    }

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK b kq - 9 6")]
    fn test_eval_1() {
        assert_eq!(eval.evaluate::<White, Black>(), 3439);
    }

    #[test]
    #[evaluation_test("1K6/6R1/1P1kPp2/4q1P1/p1r2Np1/4P2r/1Qn5/8 w - - 0 1")]
    fn test_eval_2() {
        assert_eq!(eval.evaluate::<White, Black>(), -324);
    }

    #[test]
    #[evaluation_test("5nBN/1bKP1P2/2Q3pp/1P1qR1P1/3p4/3P4/k4r2/N3b3 w - - 0 1")]
    fn test_eval_3() {
        assert_eq!(eval.evaluate::<White, Black>(), -724);
    }

    #[test]
    #[evaluation_test("3bR3/1KNp1pPp/p4Q1P/4P1P1/3B1r1P/1n4rP/n1q1p1pN/2k5 w - - 0 1")]
    fn test_eval_4() {
        assert_eq!(eval.evaluate::<White, Black>(), -3012);
    }

    #[test]
    #[evaluation_test("k7/1Rn1brB1/b2P1PPp/4nr2/N4p2/3PN3/2Q1p1K1/4q2R w - - 0 1")]
    fn test_eval_5() {
        assert_eq!(eval.evaluate::<White, Black>(), -2308)
    }

    #[test]
    #[evaluation_test("8/2p5/1R5K/4b2p/3Q3p/7B/2k5/8 w - - 0 1")]
    fn test_eval_6() {
        assert_eq!(eval.evaluate::<White, Black>(), 3020);
    }

    #[test]
    #[evaluation_test("2Q5/P2P1pP1/2qnRN2/bpPpKPpP/PBr4n/r1pRp2B/ppb5/k2N4 w - - 0 1")]
    fn test_eval_7() {
        assert_eq!(eval.evaluate::<White, Black>(), -2788);
    }

    #[test]
    #[evaluation_test("N1B1QR2/2nP2Kb/3p1BpP/b1NpPpp1/1k1p2P1/1pp2q1P/1P4Pr/n3r2R w - - 0 1")]
    fn test_eval_8() {
        assert_eq!(eval.evaluate::<White, Black>(), -420);
    }

    #[test]
    #[evaluation_test("1n1QB1N1/pppR2b1/1r3B2/pP1bp1np/1r4PN/pRPq2PP/1p1P1K1P/3k4 w - - 0 1")]
    fn test_eval_9() {
        assert_eq!(eval.evaluate::<White, Black>(), -5204);
    }

    #[test]
    #[evaluation_test("nb3n2/QP3prp/r3bpN1/1q1N2Pp/pRP3Bp/P2RpP1P/1pk1K2P/B7 w - - 0 1")]
    fn test_eval_10() {
        assert_eq!(eval.evaluate::<White, Black>(), 6604);
    }

    #[test]
    #[evaluation_test("1q1B1r2/1pp1b1pN/3k2P1/1B2N1Rr/P1P3bQ/p1KpPnpR/PPpn2Pp/8 w - - 0 1")]
    fn test_eval_11() {
        assert_eq!(eval.evaluate::<White, Black>(), -2420);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_mg_0() {
        eval.init::<White, Black>();
        assert_eq!(eval.middle_game_eval::<White, Black>(), -1225);
    }

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK w kq - 9 6")]
    fn test_mg_1() {
        eval.init::<White, Black>();
        assert_eq!(eval.middle_game_eval::<White, Black>(), 4132);
    }

    #[test]
    #[evaluation_test("1K6/6R1/1P1kPp2/4q1P1/p1r2Np1/4P2r/1Qn5/8 w - - 0 1")]
    fn test_mg_2() {
        eval.init::<White, Black>();
        assert_eq!(eval.middle_game_eval::<White, Black>(), 113);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_eg_0() {
        eval.init::<White, Black>();
        assert_eq!(eval.end_game_eval::<White, Black>(), -344);
    }

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK w kq - 9 6")]
    fn test_eg_1() {
        eval.init::<White, Black>();
        assert_eq!(eval.end_game_eval::<White, Black>(), 2011);
    }

    #[test]
    #[evaluation_test("1K6/6R1/1P1kPp2/4q1P1/p1r2Np1/4P2r/1Qn5/8 w - - 0 1")]
    fn test_eg_2() {
        eval.init::<White, Black>();
        assert_eq!(eval.end_game_eval::<White, Black>(), -1096);
    }

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK w kq - 9 6")]
    fn test_scale_factor() {
        eval.init::<White, Black>();
        let eg = eval.end_game_eval::<White, Black>();
        assert_eq!(if eg > 0 { eval.scale_factor::<White, Black>() } else { eval.scale_factor::<Black, White>() }, 50);
    }

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK w kq - 9 6")]
    fn test_phase() {
        eval.init::<White, Black>();
        assert_eq!(eval.phase::<White, Black>(), 112);
    }

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK b kq - 9 6")]
    fn test_tempo() {
        eval.init::<White, Black>();
        assert_eq!(eval.tempo::<White>(), -28);
    }
}


#[cfg(test)]
pub(super) mod test_prelude {
    pub use crate::precomp::PrecomputedData;
    pub use crate::board::Board;
    pub use crate::board::zobrist::Zobrist;
    pub use crate::color::{Color, White, Black};
    pub use crate::move_gen::magics::MagicBitBoards;
    pub use crate::eval::Evaluation;
    pub use crate::assert_eval;
    pub use crate::sum_sqrs;
    pub use proc_macro_utils::evaluation_test;
}

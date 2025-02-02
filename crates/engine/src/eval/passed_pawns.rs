use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, precomp::Precomputed, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    // TODO: Improve this majorly.
    pub fn candidate_passed<W: Color, B: Color>(&self) -> BitBoard {
        let mut passed = BitBoard(0);
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let friendly_pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let supported = self.supported[W::index()].0;

        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            if (Precomputed::forward_files(W::index(), sqr) 
                & self.board.piece_bitboards[W::piece(Piece::PAWN)]).0 != 0 { continue };

            let mut forward_enemies = Precomputed::forward_files(W::index(), sqr) 
                & self.board.piece_bitboards[B::piece(Piece::PAWN)];
            let ty1 = if W::is_white() && forward_enemies.0 != 0 {
                Coord::from_idx(forward_enemies.msb() as i8).rank()
            } else if forward_enemies.0 != 0 {
                Coord::from_idx(forward_enemies.lsb() as i8).rank()
            } else {
                W::max_back_rank()
            };

            let mut span = Precomputed::pawn_attack_span(W::index(), sqr)
                & self.board.piece_bitboards[B::piece(Piece::PAWN)];
            let ty2 = if W::is_white() && span.0 != 0 {
                Coord::from_idx(span.msb() as i8).rank()
            } else if span.0 != 0 {
                Coord::from_idx(span.lsb() as i8).rank()
            } else {
                W::max_back_rank()
            };

            if ty1 == W::max_back_rank() && W::below_eq(ty2, sqr.rank() + W::up_dir()) {
                passed |= sqr.to_bitboard();
                continue;
            }
            if W::above(ty2, sqr.rank() + 2 * W::up_dir()) || W::above(ty1, sqr.rank() + W::up_dir()) { continue };

            if (W::below_eq(ty2, sqr.rank()) && ty1 == sqr.rank() + W::up_dir() && W::above(sqr.rank(), W::rank(3)))
            && ((friendly_pawns.contains_checked(Coord::new_unchecked(sqr.file() - 1, sqr.rank() + W::down_dir()))
            && !enemy_pawns.contains_checked(Coord::new_unchecked(sqr.file() - 1, sqr.rank()))
            && !enemy_pawns.contains_checked(Coord::new_unchecked(sqr.file() - 2, sqr.rank() + W::up_dir())))
            || (friendly_pawns.contains_checked(Coord::new_unchecked(sqr.file() + 1, sqr.rank() + W::down_dir()))
            && !enemy_pawns.contains_checked(Coord::new_unchecked(sqr.file() + 1, sqr.rank()))
            && !enemy_pawns.contains_checked(Coord::new_unchecked(sqr.file() + 2, sqr.rank() + W::up_dir())))) {
                passed |= sqr.to_bitboard();
                continue;
            }

            if enemy_pawns.contains_checked(Coord::new(sqr.file(), sqr.rank() + W::up_dir())) { continue };
            
            let lever = (Precomputed::pawn_attacks(sqr.to_bitboard(), W::is_white()) & enemy_pawns).count() as i32;
            let lever_push = (Precomputed::pawn_attacks(sqr.to_bitboard().shifted_2d(W::up()), W::is_white()) & enemy_pawns).count() as i32;
            let phalanx = (Precomputed::pawn_attacks(sqr.to_bitboard().shifted_2d(W::down()), W::is_white()) & friendly_pawns).count() as i32;

            if lever - if supported.contains_square(sqr.square()) { 1 } else { 0 } > 1 { continue };
            if lever_push - phalanx > 0 { continue };
            if lever > 0 && lever_push > 0 { continue };

            passed |= sqr.to_bitboard();
        }

        passed
    }

    pub fn king_proximity<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.passed_leverable[W::index()];

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);

            let r = W::rank(sqr.rank());
            let w = if r > 2 { 5 * r as i32 - 13 } else { 0 };
            let mut v = 0;
            if w <= 0 { continue };

            let kw = self.king_square::<W, B>();
            let kb = self.king_square::<B, W>();
            let offset = if W::is_white() { 1 } else { -1 };

            v += ((kb.rank() - sqr.rank() - offset) as i32).abs().max(((kb.file() - sqr.file()) as i32).abs()).min(5) * 19 / 4 * w;
            v -= ((kw.rank() - sqr.rank() - offset) as i32).abs().max(((kw.file() - sqr.file()) as i32).abs()).min(5) * 2 * w;
            if r < 6 {
                v -= ((kw.rank() - sqr.rank() - 2 * offset) as i32).abs().max(((kw.file() - sqr.file()) as i32).abs()).min(5) * w;
            }

            eval[sqr] = v;
        }

        eval
    }

    pub fn passed_block<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();

        let mut sqrs = BitBoard::from_ranks(W::ranks(3..=7));
        sqrs &= self.passed_leverable[W::index()];

        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        pawns &= !self.board.all_pieces_bitboard.shifted_2d(W::down());
        
        sqrs &= pawns;

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);

            let r = W::rank(sqr.rank());
            let w = 5 * r as i32 - 13;

            let forward_file = Precomputed::forward_files(W::index(), sqr);
            let span = Precomputed::pawn_attack_span(W::index(), sqr);
            let push_sqr = sqr.to_bitboard().shifted_2d(W::up());
            let attacks = self.all_attacks[W::index()];
            let enemy_attacks = self.all_attacks[B::index()];

            let mut defended = (forward_file & attacks).count() as i32;
            let mut not_safe = (forward_file & enemy_attacks).count() as i32;
            let w_not_safe = (span & self.all_attacks[B::index()]).count() as i32;

            let mut defended_1 = (push_sqr & attacks).count() as i32;
            let mut not_safe_1 = (push_sqr & enemy_attacks).count() as i32;

            let backward_file = Precomputed::forward_files(B::index(), sqr);
            let defenders = self.board.piece_bitboards[W::piece(Piece::ROOK)] | self.board.piece_bitboards[W::piece(Piece::QUEEN)];
            let attackers = self.board.piece_bitboards[B::piece(Piece::ROOK)] | self.board.piece_bitboards[B::piece(Piece::QUEEN)];

            if (backward_file & defenders).0 != 0 {
                defended = 1;
                defended_1 = 1;
            }

            if (backward_file & attackers).0 != 0 {
                not_safe = 1;
                not_safe_1 = 1;
            }

            let k = if not_safe == 0 && w_not_safe == 0 { 35 } 
                else if not_safe == 0 { 20 } 
                else if not_safe_1 == 0 { 9 }
                else { 0 }
                + if defended_1 != 0 { 5 } else { 0 };

            eval[sqr] = k * w;
        }

        eval
    }

    pub fn passed_file<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.passed_leverable[W::index()];

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            eval[sqr] = sqr.file().min(7 - sqr.file()) as i32;
        }

        eval
    }

    pub fn passed_rank<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.passed_leverable[W::index()];

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            eval[sqr] = W::rank(sqr.rank()) as i32;
        }

        eval
    }

    pub fn passed_leverable<W: Color, B: Color>(&self) -> BitBoard {
        let passed = self.candidate_passed[W::index()];
        (passed & !self.board.piece_bitboards[B::piece(Piece::PAWN)].shifted_2d(W::down()))
            | (passed & self.board.piece_bitboards[W::piece(Piece::PAWN)].shifted_2d(W::offset(1, 1))
               & !self.board.color_bitboards[B::index()].shifted_2d(Coord::new(1, 0))
               & (self.all_attacks[W::index()].shifted_2d(W::offset(1, 0)) | !self.all_doubled_attacks[B::index()].shifted_2d(W::offset(1, 0))))
            | (passed & self.board.piece_bitboards[W::piece(Piece::PAWN)].shifted_2d(W::offset(-1, 1))
               & !self.board.color_bitboards[B::index()].shifted_2d(Coord::new(-1, 0))
               & (self.all_attacks[W::index()].shifted_2d(W::offset(-1, 0)) | !self.all_doubled_attacks[B::index()].shifted_2d(W::offset(-1, 0))))
    }

    const PASSED_RANK_VAL_MG: [i32; 7] = [0, 10, 17, 15, 62, 168, 276];
    const PASSED_RANK_VAL_EG: [i32; 7] = [0, 28, 33, 41, 72, 177, 260];
    /// Returns `(mg, eg)`
    pub fn passed<W: Color, B: Color>(&self) -> (i32, i32) {
        let mut mg = 0;
        let mut eg = 0;

        let passed_rank = self.passed_rank::<W, B>();
        let passed_block = self.passed_block::<W, B>();
        let passed_file = self.passed_file::<W, B>();

        mg += passed_rank.map(|i| Self::PASSED_RANK_VAL_MG[i as usize]).count();
        mg += passed_block.count();
        mg -= 11 * passed_file.count();

        eg += self.king_proximity::<W, B>().count();
        eg += passed_rank.map(|i| Self::PASSED_RANK_VAL_EG[i as usize]).count();
        eg += passed_block.count();
        eg -= 8 * passed_file.count();

        (mg, eg)
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_candidate_passed() {
        assert_eval!(+ - candidate_passed, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king_proximity() {
        assert_eval!(+ - king_proximity, -18, -7, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_block() {
        assert_eval!(+ - passed_block, 10, 35, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_file() {
        assert_eval!(+ - passed_file, 3, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_rank() {
        assert_eval!(+ - passed_rank, 5, 4, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed_leverable() {
        assert_eval!(+ - passed_leverable, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p3n2n/np1k1pR1/pQ3P1B/1b1P1qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_passed() {
        assert_eval!(- passed, (9, 42), (86, 92), eval);
    }
}

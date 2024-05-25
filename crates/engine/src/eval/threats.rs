use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, precomp::Precomputed, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn safe_pawn<W: Color, B: Color>(&self) -> BitBoard {
        self.board.piece_bitboards[W::piece(Piece::PAWN)] 
            & (self.all_attacks::<W, B>() | !self.all_attacks::<B, W>())
    }

    pub fn threat_safe_pawn<W: Color, B: Color>(&self) -> BitBoard {
        let non_pawn_enemies = self.board.piece_bitboards[B::piece(Piece::KNIGHT)] 
            | self.board.piece_bitboards[B::piece(Piece::BISHOP)]
            | self.board.piece_bitboards[B::piece(Piece::ROOK)]
            | self.board.piece_bitboards[B::piece(Piece::QUEEN)];
        let safe_pawn_attacks = Precomputed::pawn_attacks(self.safe_pawn::<W, B>(), W::is_white());

        non_pawn_enemies & safe_pawn_attacks
    }

    pub fn weak_enemies<W: Color, B: Color>(&self) -> BitBoard {
        let enemies = self.board.color_bitboards[B::index()];
        let pawn_defended = self.all_pawn_attacks::<B, W>().0;

        (enemies & !pawn_defended) 
            & (self.all_doubled_attacks::<W, B>() | (self.all_attacks::<W, B>() & !self.all_doubled_attacks::<B, W>())) }

    pub fn minor_threat<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let mut pieces = self.board.color_bitboards[B::index()];
        pieces &= self.all_knight_attacks::<W, B>().0 | self.all_bishop_xray_attacks::<W, B>().0;
        pieces &= !(
            (enemy_pawns | !(
                Precomputed::pawn_attacks(enemy_pawns, B::is_white())
                | (self.all_attacks::<W, B>() & !self.all_doubled_attacks::<W, B>() & self.all_doubled_attacks::<B, W>())))
            & !self.weak_enemies::<W, B>());

        while pieces.0 != 0 {
            let sqr = Coord::from_idx(pieces.pop_lsb() as i8);
            eval[sqr] = self.board.square[sqr].piece_type() as i32;
        }

        eval

    }

    pub fn rook_threat<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();

        let mut pieces = self.board.color_bitboards[B::index()];
        pieces &= self.weak_enemies::<W, B>();
        pieces &= self.all_rook_xray_attacks::<W, B>().0;

        while pieces.0 != 0 {
            let sqr = Coord::from_idx(pieces.pop_lsb() as i8);
            eval[sqr] = self.board.square[sqr].piece_type() as i32;
        }

        eval
    }

    pub fn hanging<W: Color, B: Color>(&self) -> BitBoard {
        self.weak_enemies::<W, B>() 
            & ((!self.board.piece_bitboards[B::piece(Piece::PAWN)] & self.all_doubled_attacks::<W, B>()) | !self.all_attacks::<B, W>())
    }

    pub fn king_threat<W: Color, B: Color>(&self) -> BitBoard {
        (self.board.color_bitboards[B::index()] & !self.board.piece_bitboards[B::piece(Piece::KING)])
            & self.weak_enemies::<W, B>() & self.all_king_attacks::<W, B>()
    }

    pub fn pawn_push_threat<W: Color, B: Color>(&self) -> BitBoard {
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];
        let enemies = self.board.color_bitboards[B::index()] & !enemy_pawns;
        let pieces = self.board.all_pieces_bitboard;
        let attacks = self.all_attacks::<W, B>();
        let enemy_attacks = self.all_attacks::<B, W>();

        enemies & (
            (pawns.shifted_2d(W::offset(1, 2)) & !pieces.shifted_2d(W::offset(1, 1))
                & !enemy_pawns.shifted_2d(W::offset(2, 0)) & (attacks.shifted_2d(W::offset(1, 1)) | !enemy_attacks.shifted_2d(B::offset(1, -1))))
            | (BitBoard::from_rank(W::rank(4)) & pawns.shifted_2d(W::offset(1, 3)) & !pieces.shifted_2d(W::offset(1, 2))
                & !pieces.shifted_2d(W::offset(1, 1)) & !enemy_pawns.shifted_2d(W::offset(2, 0))
                & (attacks.shifted_2d(W::offset(1, 1)) | !enemy_attacks.shifted_2d(B::offset(1, -1))))

            | (pawns.shifted_2d(W::offset(-1, 2)) & !pieces.shifted_2d(W::offset(-1, 1))
                & !enemy_pawns.shifted_2d(W::offset(-2, 0)) & (attacks.shifted_2d(W::offset(-1, 1)) | !enemy_attacks.shifted_2d(B::offset(-1, -1))))
            | (BitBoard::from_rank(W::rank(4)) & pawns.shifted_2d(W::offset(-1, 3)) & !pieces.shifted_2d(W::offset(-1, 2))
                & !pieces.shifted_2d(W::offset(-1, 1)) & !enemy_pawns.shifted_2d(W::offset(-2, 0))
                & (attacks.shifted_2d(W::offset(-1, 1)) | !enemy_attacks.shifted_2d(B::offset(-1, -1))))
        )
    }

    /// Remember to multiply by two when the friendly side has one or more queens.
    pub fn slider_on_queen<W: Color, B: Color>(&self) -> BitBoard {
        if self.queen_count::<B, W>() != 1 { return BitBoard(0) };
        let mut on_queen = BitBoard(0);

        let mut threats = !self.board.piece_bitboards[W::piece(Piece::PAWN)];
        threats &= !self.all_pawn_attacks::<B, W>().0;
        threats &= self.all_doubled_attacks::<W, B>();
        threats &= self.mobility_area::<W, B>();

        while threats.0 != 0 {
            let sqr = Coord::from_idx(threats.pop_lsb() as i8);
            let diagonal = self.queen_attack_diagonal::<B, W>(None, sqr).count() > 0;
            if (diagonal && self.bishop_xray_attack::<W, B>(None, sqr).count() > 0)
            || (!diagonal && self.rook_xray_attack::<W, B>(None, sqr).count() > 0
            && self.queen_attack::<B, W>(None, sqr).count() > 0) {
                on_queen |= sqr.to_bitboard();
            }
        }

        on_queen
    }

    /// Remember to multiply by two when the friendly side has one or more queens.
    pub fn knight_on_queen<W: Color, B: Color>(&self) -> BitBoard {
        if self.queen_count::<B, W>() != 1 { return BitBoard(0) };
        let queen = Coord::from_idx(self.board.piece_bitboards[B::piece(Piece::QUEEN)].clone().pop_lsb() as i8);
        let mut on_queen = BitBoard(0);

        let mut threats = !self.board.piece_bitboards[W::piece(Piece::PAWN)];
        threats &= !self.all_pawn_attacks::<B, W>().0;
        threats &= !(!self.all_doubled_attacks::<W, B>() & self.all_doubled_attacks::<B, W>());
        threats &= self.mobility_area::<W, B>();
        threats &= self.all_knight_attacks::<W, B>().0;

        while threats.0 != 0 {
            let sqr = Coord::from_idx(threats.pop_lsb() as i8);
            if ((queen.file() - sqr.file()).abs() == 2 && (queen.rank() - sqr.rank()).abs() == 1)
            || ((queen.file() - sqr.file()).abs() == 1 && (queen.rank() - sqr.rank()).abs() == 2) {
                on_queen |= sqr.to_bitboard();
            }
        }

        on_queen
    }

    pub fn restricted<W: Color, B: Color>(&self) -> BitBoard {
        self.all_attacks::<W, B>() & self.all_attacks::<B, W>() & !self.all_pawn_attacks::<B, W>().0
            & !(self.all_doubled_attacks::<B, W>() & (self.all_attacks::<W, B>() & !self.all_doubled_attacks::<W, B>()))
    }

    pub fn weak_queen_protection<W: Color, B: Color>(&self) -> BitBoard {
        self.weak_enemies::<W, B>() & self.all_queen_attacks::<B, W>().0
    }

    const MINOR_THREAT_MG_VALS: [i32; 7] = [0, 5, 57, 77, 88, 79, 0];
    const ROOK_THREAT_MG_VALS: [i32; 7] = [0, 3, 37, 42, 0, 58, 0];
    pub fn threats_mg<W: Color, B: Color>(&self) -> i32 {
        let mut v = 0;

        v += 69 * self.hanging::<W, B>().count() as i32;
        v += if self.king_threat::<W, B>().0 > 0 { 24 } else { 0 };
        v += 48 * self.pawn_push_threat::<W, B>().count() as i32;
        v += 173 * self.threat_safe_pawn::<W, B>().count() as i32;

        let slider_on_queen = self.slider_on_queen::<W, B>().count() as i32;
        v += 60 * if self.queen_count::<W, B>() == 0 { 2 * slider_on_queen } else { slider_on_queen };

        let knight_on_queen = self.knight_on_queen::<W, B>().count() as i32;
        v += 16 * if self.queen_count::<W, B>() == 0 { 2 * knight_on_queen } else { knight_on_queen };

        v += 7 * self.restricted::<W, B>().count() as i32;
        v += 14 * self.weak_queen_protection::<W, B>().count() as i32;

        v += self.minor_threat::<W, B>().map(|i| Self::MINOR_THREAT_MG_VALS[i as usize]).count();
        v += self.rook_threat::<W, B>().map(|i| Self::ROOK_THREAT_MG_VALS[i as usize]).count();

        v
    }

    const MINOR_THREAT_EG_VALS: [i32; 7] = [0, 32, 41, 56, 119, 161, 0];
    const ROOK_THREAT_EG_VALS: [i32; 7] = [0, 46, 68, 60, 38, 41, 0];
    pub fn threats_eg<W: Color, B: Color>(&self) -> i32 {
        let mut v = 0;

        v += 36 * self.hanging::<W, B>().count() as i32;
        v += if self.king_threat::<W, B>().0 > 0 { 89 } else { 0 };
        v += 39 * self.pawn_push_threat::<W, B>().count() as i32;
        v += 94 * self.threat_safe_pawn::<W, B>().count() as i32;

        let slider_on_queen = self.slider_on_queen::<W, B>().count() as i32;
        v += 18 * if self.queen_count::<W, B>() == 0 { 2 * slider_on_queen } else { slider_on_queen };

        let knight_on_queen = self.knight_on_queen::<W, B>().count() as i32;
        v += 11 * if self.queen_count::<W, B>() == 0 { 2 * knight_on_queen } else { knight_on_queen };

        v += 7 * self.restricted::<W, B>().count() as i32;

        v += self.minor_threat::<W, B>().map(|i| Self::MINOR_THREAT_EG_VALS[i as usize]).count();
        v += self.rook_threat::<W, B>().map(|i| Self::ROOK_THREAT_EG_VALS[i as usize]).count();

        v
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/p1p1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 w kq - 3 9")]
    fn test_safe_pawn() {
        assert_eval!(+ - safe_pawn, 4, 2, eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_threat_safe_pawn() {
        assert_eval!(+ - threat_safe_pawn, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_weak_enemies() {
        assert_eval!(+ - weak_enemies, 5, 7, eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_minor_threat() {
        assert_eval!(+ - minor_threat, 18, 11, eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_rook_threat() {
        assert_eval!(+ - rook_threat, 3, 6, eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/1k2p2p/p2n2R1/1pp1bP1q/R1P1qB1r/1NP3nP/P4PBR/6K1 b kq - 0 9")]
    fn test_hanging() {
        assert_eval!(+ - hanging, 4, 5, eval);
    }

    #[test]
    #[evaluation_test("nr1B3Q/4p2p/p2n2R1/kPp1bP1q/R3qB1r/1NP4P/P4PBR/5nK1 b kq - 0 9")]
    fn test_king_threat() {
        assert_eval!(+ - king_threat, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_pawn_push_threat() {
        assert_eval!(+ - pawn_push_threat, 3, 1, eval);
    }

    #[test]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_slider_on_queen() {
        assert_eval!(+ - slider_on_queen, 4, 3, eval);
    }

    #[test]
    #[evaluation_test("n2Br3/2p1p1Q1/p2n4/kRp1bP1r/P1P4r/3BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_knight_on_queen() {
        assert_eval!(+ - knight_on_queen, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_restricted() {
        assert_eval!(+ - restricted, 20, 16, eval);
    }

    #[test]
    #[evaluation_test("n1n1r3/4p1Q1/1q2pP2/kpp1bB1r/P1PB3r/R3N2P/P4P1R/1B2RnK1 b kq - 2 11")]
    fn test_weak_queen_protection() {
        assert_eval!(+ - weak_queen_protection, 3, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_threats_mg() {
        assert_eval!(- threats_mg, 1004, 827, eval);
    }

    #[test]
    #[evaluation_test("n3r3/2p1p1Q1/p2n4/k1p1bP1r/P1PB3r/R2BN2P/Pq3P1R/1B2RnK1 b kq - 0 9")]
    fn test_threats_eg() {
        assert_eval!(- threats_eg, 814, 982, eval);
    }
}

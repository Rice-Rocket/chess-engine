use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, prelude::BitBoard};
use super::Evaluation;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckType {
    All,      // 'null'
    Knight,   // 0
    Bishop,   // 1
    Rook,     // 2
    Queen,    // 3
    NotQueen, // 4
}


impl<'a> Evaluation<'a> {
    /// Whether or not the enemy king is on a pawnless flank. 
    ///
    /// A pawnless flank is defined as a flank with no enemy pawns. 
    // TODO: Cache this
    #[evaluation_fn]
    pub fn pawnless_flank(&self) -> bool {
        let kx = self.enemy_king_square().file();
        let pawns = self.board.piece_bitboards[self.color.piece(Piece::PAWN)];
        let mut counts = [0u8; 8];

        for (file, count) in counts.iter_mut().enumerate() {
            *count = (pawns & BitBoard::FILES[file]).count() as u8;
        }

        let sum = match kx {
            0 => counts[0] + counts[1] + counts[2],
            1..=2 => counts[0] + counts[1] + counts[2] + counts[3],
            3..=4 => counts[2] + counts[3] + counts[4] + counts[5],
            5..=6 => counts[4] + counts[5] + counts[6] + counts[7],
            _ => counts[5] + counts[6] + counts[7],
        };

        sum == 0
    }

    const WEAKNESS: [[i32; 7]; 4] = [
        [ -6,  81,  93,  58,  39,  18,   25],
        [-43,  61,  35, -49, -29, -11,  -63],
        [-10,  75,  23,  -2,  32,   3,  -45],
        [-39, -13, -29, -52, -48, -67, -166]
    ];
    /// King shelter strength for each square on the board.
    #[evaluation_fn]
    pub fn strength_square(&self, sqr: Coord) -> i32 {
        let mut v = 5;
        let kx = sqr.file().max(1).min(6);
        
        // TODO: Improve this making use of bitboards
        let friendly_pawns = self.board.piece_bitboards[self.color.piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[self.color.flip().piece(Piece::PAWN)];
        for file in kx - 1..=kx + 1 {
            let mut us = 0;
            for rank in self.color.ranks_up_till_incl(sqr.rank()) {
                let s = Coord::to_index(file, rank);
                let s1 = Coord::new_unchecked(file - 1, rank + self.color.down_dir());
                let s2 = Coord::new_unchecked(file + 1, rank + self.color.down_dir());
                if enemy_pawns.contains_square(s) 
                && !friendly_pawns.get_square(s1) 
                && !friendly_pawns.get_square(s2) {
                    // NOTE: With arrays like this, it's from the white players perspective. (but
                    // also in this case ranks are flipped). Remember to adjust the index
                    // accordingly. 
                    us = if self.color.is_white() { 7 - rank } else { rank };
                }
            }
            let f = file.min(7 - file);
            if us < 7 {
                v += Self::WEAKNESS[f as usize][us as usize];
            }
        }

        v
    }

    const UNBLOCKED_STORM: [[i32; 7]; 4] = [
        [ 85, -289, -166, 97, 50,  45,  50],
        [ 46,  -25,  122, 45, 37, -10,  20],
        [ -6,   51,  168, 34, -2, -22, -14],
        [-15,  -11,  101,  4, 11, -15, -29]
    ];
    const BLOCKED_STORM: [[i32; 7]; 2] = [
        [0, 0, 76, -10, -7, -4, -1],
        [0, 0, 78,  15, 10,  6,  2]
    ];
    /// Enemy pawns storm for each square on the board. 
    #[evaluation_fn]
    pub fn storm_square(&self, eg: bool, sqr: Coord) -> i32 {
        let mut blocked_idx = if eg { 1 } else { 0 };
        let mut v = 0;
        let kx = sqr.file().max(1).min(6);

        // TODO: Improve this making use of bitboards. Very similar to above function.
        let friendly_pawns = self.board.piece_bitboards[self.color.piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[self.color.flip().piece(Piece::PAWN)];
        for file in kx - 1..=kx + 1 {
            let (mut us, mut them) = (0, 0);

            for rank in self.color.ranks_up_till_incl(sqr.rank()) {
                let s = Coord::to_index(file, rank);
                let s1 = Coord::new_unchecked(file - 1, rank + self.color.down_dir());
                let s2 = Coord::new_unchecked(file + 1, rank + self.color.down_dir());
                if enemy_pawns.contains_square(s) 
                && !friendly_pawns.get_square(s1) 
                && !friendly_pawns.get_square(s2) {
                    us = if self.color.is_white() { 7 - rank } else { rank };
                }
                if friendly_pawns.contains_square(s) {
                    them = if self.color.is_white() { 7 - rank } else { rank };
                }
            }

            let f = file.min(7 - file);
            if us > 0 && them == us + 1 {
                v += Self::BLOCKED_STORM[blocked_idx][them as usize];
            } else if !eg {
                v += Self::UNBLOCKED_STORM[f as usize][them as usize];
            }
        }

        if eg { v + 5 } else { v }
    }

    /// Returns `(shelter_strength, shelter_storm)`
    // TODO: Cache the result of this function.
    #[evaluation_fn]
    pub fn shelter_strength_storm_eg(&self) -> (i32, i32, i32) {
        let mut w = 0;
        let mut s = 1024;
        let mut e = 0;

        // TODO: Improve this with bitboards. This loop is disgusting.
        // And more easily: cache the strength_square and storm_square values as well.
        for sqr in Coord::iter_squares() {
            if sqr == self.enemy_king_square() 
            || (self.board.current_state.has_kingside_castle_right(self.color.flip().is_white()) 
                && sqr.file() == 6 && sqr.rank() == self.color.flip().back_rank())
            || (self.board.current_state.has_queenside_castle_right(self.color.flip().is_white())
                && sqr.file() == 2 && sqr.rank() == self.color.flip().back_rank()) {
                let w1 = self.friendly_strength_square(sqr);
                let s1 = self.friendly_storm_square(false, sqr);
                let e1 = self.friendly_storm_square(true, sqr);
                if (s1 - w1 < s - w) {
                    w = w1;
                    s = s1;
                    e = e1;
                }
            }
        }

        (w, s, e)
    }

    /// The minimum distance from the friendly king to a friendly pawn
    // TODO: Cache this
    #[evaluation_fn]
    pub fn king_pawn_distance(&self) -> i32 {
        let k = self.friendly_king_square();
        let mut pawns = self.board.piece_bitboards[self.color.piece(Piece::PAWN)];

        let mut closest = 6;
        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let d = self.precomp.king_distance[sqr][k];
            closest = closest.min(d);
        }

        closest as i32
    }

    /// The positions to which friendly pieces could move to deliver check.
    // TODO: Cache this
    #[evaluation_fn]
    pub fn check(&self, ty: CheckType) -> BitBoard {
        let king = self.enemy_king_square();
        let blockers = self.board.all_pieces_bitboard & !self.board.piece_bitboards[self.color.flip().piece(Piece::QUEEN)];
        let mut checks = BitBoard(0);

        if ty == CheckType::Rook || ty == CheckType::All || ty == CheckType::NotQueen {
            let mut moves = self.friendly_all_rook_xray_attacks().0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = self.magics.get_rook_attacks(sqr, blockers);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        if ty == CheckType::Bishop || ty == CheckType::All || ty == CheckType::NotQueen {
            let mut moves = self.friendly_all_bishop_xray_attacks().0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = self.magics.get_bishop_attacks(sqr, blockers);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        if ty == CheckType::Knight || ty == CheckType::All || ty == CheckType::NotQueen {
            let mut moves = self.friendly_all_knight_attacks().0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = self.precomp.knight_moves[sqr];
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        if ty == CheckType::All {
            let mut moves = self.friendly_all_queen_attacks().0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = self.magics.get_bishop_attacks(sqr, blockers) | self.magics.get_rook_attacks(sqr, blockers);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        checks
    }

    /// The positions to which friendly pieces could move to deliver check without being captured. 
    // TODO: Cache this
    #[evaluation_fn]
    pub fn safe_check(&self, ty: CheckType) -> BitBoard {
        let mut checks = self.friendly_check(ty) & !self.board.color_bitboards[self.color];

        if ty == CheckType::Queen {
            checks &= !self.friendly_safe_check(CheckType::Rook);
        } else if ty == CheckType::Bishop {
            checks &= !self.friendly_safe_check(CheckType::Queen);
        }
        
        checks &= !self.enemy_all_attacks() | (self.friendly_weak_squares() & self.friendly_all_doubled_attacks());

        if ty == CheckType::Queen {
            checks &= !self.enemy_all_queen_attacks().0;
        }

        checks
    }

    /// The friendly pieces that attack squares in the king ring. Pawns which attack two squares in
    /// the king ring are part of a separate bitboard. 
    ///
    /// Returns: `(attackers (+ double pawns attacks), double pawn attacks)`
    // TODO: Cache this
    #[evaluation_fn]
    pub fn king_attackers_origin(&self) -> (BitBoard, BitBoard) {
        let mut attacked = BitBoard(0);
        let mut pawn_attacked = BitBoard(0);
        let mut double_pawn_attacked = BitBoard(0);
        let mut attacked_king_ring = self.precomp.king_ring[self.enemy_king_square()] & self.friendly_all_attacks();

        while attacked_king_ring.0 != 0 {
            let sqr = Coord::from_idx(attacked_king_ring.pop_lsb() as i8);
            let pawn_attacks = self.friendly_pawn_attack(None, sqr);
            attacked |= pawn_attacks
                | self.friendly_knight_attack(None, sqr)
                | self.friendly_bishop_xray_attack(None, sqr)
                | self.friendly_rook_xray_attack(None, sqr)
                | self.friendly_queen_attack(None, sqr);

            double_pawn_attacked |= pawn_attacked & pawn_attacks;
            pawn_attacked |= pawn_attacks;
        }

        (attacked | double_pawn_attacked, double_pawn_attacked)
    }

    const KING_ATTACK_WEIGHTS: [i32; 4] = [81, 52, 44, 10]; // Knight, bishop, rook, queen
    #[evaluation_fn]
    pub fn king_attackers_weight(&self) -> i32 {
        let attacks = self.friendly_king_attackers_origin().0;

        (attacks & self.board.piece_bitboards[self.color.piece(Piece::KNIGHT)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[0]
        + (attacks & self.board.piece_bitboards[self.color.piece(Piece::BISHOP)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[1]
        + (attacks & self.board.piece_bitboards[self.color.piece(Piece::ROOK)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[2]
        + (attacks & self.board.piece_bitboards[self.color.piece(Piece::QUEEN)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[3]
    }

    // TODO: Switch to using `SquareEvaluations`
    #[evaluation_fn]
    pub fn king_attacks(&self, sqr: Coord) -> i32 {
        let mut adjacent = self.precomp.diagonal_directions[self.enemy_king_square()] 
            | self.precomp.orthogonal_directions[self.enemy_king_square()];
        
        let mut v = 0;
        while adjacent.0 != 0 {
            let s = Coord::from_idx(adjacent.pop_lsb() as i8);
            v += self.friendly_knight_attack(Some(sqr), s).count() as i32
                + self.friendly_bishop_xray_attack(Some(sqr), s).count() as i32
                + self.friendly_rook_xray_attack(Some(sqr), s).count() as i32
                + self.friendly_queen_attack(Some(sqr), s).count() as i32;
        }
        v
    }

    #[evaluation_fn]
    pub fn weak_bonus(&self) -> BitBoard {
        self.friendly_weak_squares() & self.friendly_king_ring(false)
    }

    #[evaluation_fn]
    pub fn weak_squares(&self) -> BitBoard {
        self.friendly_all_attacks() 
            & !self.enemy_all_doubled_attacks()
            & (!self.enemy_all_attacks() | self.enemy_all_king_attacks() | self.enemy_all_queen_attacks().0)
    }

    #[evaluation_fn]
    pub fn unsafe_checks(&self) -> BitBoard {
        (self.friendly_check(CheckType::Knight) 
            & (if self.friendly_safe_check(CheckType::Knight).count() == 0 { BitBoard::ALL } else { BitBoard(0) }))
        | (self.friendly_check(CheckType::Bishop) 
            & (if self.friendly_safe_check(CheckType::Bishop).count() == 0 { BitBoard::ALL } else { BitBoard(0) }))
        | (self.friendly_check(CheckType::Rook) 
            & (if self.friendly_safe_check(CheckType::Rook).count() == 0 { BitBoard::ALL } else { BitBoard(0) }))
    }

    #[evaluation_fn]
    pub fn knight_defender(&self) -> BitBoard {
        self.friendly_all_knight_attacks().0 & self.friendly_all_king_attacks()
    }

    #[evaluation_fn]
    pub fn blockers_for_king(&self) -> BitBoard {
        self.pin_rays[self.color.flip()] & self.board.color_bitboards[self.color.flip()]
    }

    /// Returns `(attacked exactly once, attacked twice)`
    #[evaluation_fn]
    pub fn flank_attack(&self) -> (BitBoard, BitBoard) {
        todo!();
    }

    #[evaluation_fn]
    pub fn flank_defense(&self) -> BitBoard {
        todo!();
    }

    #[evaluation_fn]
    pub fn king_danger(&self) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn king_mg(&self) -> i32 {
        todo!();
    }

    #[evaluation_fn]
    pub fn king_eg(&self) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("3q4/4p1p1/bn1rpPp1/kr3bNp/2NPnP1P/3P2P1/3PPR2/1RBQKB2 w KQkq - 2 3")]
    fn test_pawnless_flank() {
        assert_eval!(- friendly_pawnless_flank, true, false, eval);
    }

    #[test]
    #[evaluation_test("2b1k3/1ppp1ppr/r1nb4/pB1Np1qp/3n1P2/4PQ1N/PPPP2PP/R1B2RK1 w Q - 8 8")]
    fn test_strength_square() {
        assert_eval!(friendly_strength_square, -660, -1578, eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_storm_square() {
        assert_eval!(friendly_storm_square, 672, 2579, eval; false);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_shelter_strength_storm() {
        assert_eval!(- friendly_shelter_strength_storm_eg, (-2, -27, 5), (76, 17, 5), eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_king_pawn_distance() {
        assert_eval!(- friendly_king_pawn_distance, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_check() {
        assert_eval!(+ - friendly_check, 11, 2, eval; CheckType::All);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_safe_check() {
        assert_eval!(+ - friendly_safe_check, 2, 1, eval; CheckType::All);
    }

    #[test]
    #[evaluation_test("1nb2rk1/p2rb3/p5P1/p1K1q1N1/pP1P1BQ1/p1Np4/p1P1P1PR/1R3B2 w q - 4 10")]
    fn test_king_attackers_origin() {
        assert_eval!(* - [0, 1] friendly_king_attackers_origin, (3, 1), (7, 0), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3PnB/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_king_attackers_weight() {
        assert_eval!(- friendly_king_attackers_weight, 54, 135, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_king_attacks() {
        assert_eval!(friendly_king_attacks, 6, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_weak_bonus() {
        assert_eval!(+ - friendly_weak_bonus, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_weak_squares() {
        assert_eval!(+ - friendly_weak_squares, 22, 20, eval);
    }

    #[test]
    #[evaluation_test("3q4/4p1p1/bn1rpPp1/kr1n1bNp/P1N2P1P/3P2R1/3PP1P1/1RBQKB2 b KQkq - 3 3")]
    fn test_unsafe_checks() {
        assert_eval!(+ - friendly_unsafe_checks, 2, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK w Qkq - 5 4")]
    fn test_knight_defender() {
        assert_eval!(+ - friendly_knight_defender, 1, 6, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_blockers_for_king() {
        assert_eval!(+ - friendly_blockers_for_king, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1nb2rk1/p2rb3/p5P1/2K3N1/pP1P1BQ1/2Npq3/2P1P1PR/1R3B2 w q - 4 10")]
    fn test_flank_attack() {
        assert_eval!(* - [0, 1] friendly_flank_attack, (4, 8), (8, 1), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_flank_defense() {
        assert_eval!(+ - friendly_flank_defense, 19, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king_danger() {
        assert_eval!(- friendly_king_danger, 2640, 3448, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king_mg() {
        assert_eval!(- friendly_king_mg, 1812, 3168, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king_eg() {
        assert_eval!(- friendly_king_eg, 138, 210, eval);
    }
}

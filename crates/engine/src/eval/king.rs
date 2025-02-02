use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, move_gen::magics::Magics, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, precomp::Precomputed, prelude::BitBoard, sum_sqrs};
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
    pub fn pawnless_flank<W: Color, B: Color>(&self) -> bool {
        let kx = self.king_square::<B, W>().file();
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)] | self.board.piece_bitboards[B::piece(Piece::PAWN)];
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
    pub fn strength_square<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();

        let friendly_pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];

        for sqr in Coord::iter_squares() {
            let mut v = 5;
            let kx = sqr.file().max(1).min(6);

            // TODO: Improve this making use of bitboards
            for file in kx - 1..=kx + 1 {
                let mut us = 0;
                for rank in W::ranks_up_till_incl(sqr.rank()) {
                    let s = Coord::to_index(file, rank);
                    let s1 = Coord::new_unchecked(file - 1, rank + W::down_dir());
                    let s2 = Coord::new_unchecked(file + 1, rank + W::down_dir());
                    if enemy_pawns.contains_square(s) 
                        && !friendly_pawns.contains_checked(s1) 
                            && !friendly_pawns.contains_checked(s2) {
                                us = if W::is_white() { 7 - rank } else { rank };
                            }
                }
                let f = file.min(7 - file);
                if us < 7 {
                    v += Self::WEAKNESS[f as usize][us as usize];
                }
            }

            eval[sqr] = v;
        }

        eval
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
    pub fn storm_square<W: Color, B: Color>(&self, eg: bool) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        
        let friendly_pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];

        for sqr in Coord::iter_squares() {
            let mut blocked_idx = if eg { 1 } else { 0 };
            let mut v = 0;
            let kx = sqr.file().max(1).min(6);

            // TODO: Improve this making use of bitboards. Very similar to above function.
            for file in kx - 1..=kx + 1 {
                let (mut us, mut them) = (0, 0);

                for rank in W::ranks_up_till_incl(sqr.rank()) {
                    let s = Coord::to_index(file, rank);
                    let s1 = Coord::new_unchecked(file - 1, rank + W::down_dir());
                    let s2 = Coord::new_unchecked(file + 1, rank + W::down_dir());
                    if enemy_pawns.contains_square(s) 
                        && !friendly_pawns.contains_checked(s1) 
                            && !friendly_pawns.contains_checked(s2) {
                                us = if W::is_white() { 7 - rank } else { rank };
                            }
                    if friendly_pawns.contains_square(s) {
                        them = if W::is_white() { 7 - rank } else { rank };
                    }
                }

                let f = file.min(7 - file);
                if us > 0 && them == us + 1 {
                    v += Self::BLOCKED_STORM[blocked_idx][them as usize];
                } else if !eg {
                    v += Self::UNBLOCKED_STORM[f as usize][them as usize];
                }
            }

            eval[sqr] = if eg { v + 5 } else { v };
        }

        eval
    }

    /// Returns `(shelter_strength, shelter_storm, endgame_shelter)`
    // TODO: Cache the result of this function.
    pub fn shelter_strength_storm_eg<W: Color, B: Color>(&self) -> (i32, i32, i32) {
        let mut w = 0;
        let mut s = 1024;
        let mut e = 0;

        // TODO: Improve this with bitboards. This loop is disgusting.
        // And more easily: cache the strength_square and storm_square values as well.
        for sqr in Coord::iter_squares() {
            if sqr == self.king_square::<B, W>() 
            || (self.board.current_state.has_kingside_castle_right(B::is_white()) 
                && sqr.file() == 6 && sqr.rank() == B::back_rank())
            || (self.board.current_state.has_queenside_castle_right(B::is_white())
                && sqr.file() == 2 && sqr.rank() == B::back_rank()) {
                let w1 = self.strength_square::<W, B>()[sqr];
                let s1 = self.storm_square::<W, B>(false)[sqr];
                let e1 = self.storm_square::<W, B>(true)[sqr];
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
    pub fn king_pawn_distance<W: Color, B: Color>(&self) -> i32 {
        let k = self.king_square::<W, B>();
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];

        let mut closest = 6;
        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let d = Precomputed::king_distance(sqr, k);
            closest = closest.min(d);
        }

        closest as i32
    }

    /// The positions to which friendly pieces could move to deliver check.
    // TODO: Cache this
    pub fn check<W: Color, B: Color>(&self, ty: CheckType) -> BitBoard {
        let king = self.king_square::<B, W>();
        let blockers = self.board.all_pieces_bitboard & !self.board.piece_bitboards[B::piece(Piece::QUEEN)];
        let mut checks = BitBoard(0);

        if ty == CheckType::Rook || ty == CheckType::All || ty == CheckType::NotQueen {
            let mut moves = self.all_rook_attacks[W::index()].0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = Magics::rook_attacks(sqr, blockers);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        if ty == CheckType::Bishop || ty == CheckType::All || ty == CheckType::NotQueen {
            let mut moves = self.all_bishop_attacks[W::index()].0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = Magics::bishop_attacks(sqr, blockers);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        if ty == CheckType::Knight || ty == CheckType::All || ty == CheckType::NotQueen {
            let mut moves = self.all_knight_attacks[W::index()].0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = Precomputed::knight_moves(sqr);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        if ty == CheckType::All || ty == CheckType::Queen {
            let mut moves = self.all_queen_attacks[W::index()].0;

            while moves.0 != 0 {
                let sqr = Coord::from_idx(moves.pop_lsb() as i8);
                let attacks = Magics::bishop_attacks(sqr, blockers) | Magics::rook_attacks(sqr, blockers);
                if attacks.contains_square(king.square()) {
                    checks.set_square(sqr.square());
                }
            }
        }

        checks
    }

    /// The positions to which friendly pieces could move to deliver check without being captured. 
    // TODO: Cache this
    pub fn safe_check<W: Color, B: Color>(&self, ty: CheckType) -> BitBoard {
        let mut checks = self.check::<W, B>(ty) & !self.board.color_bitboards[W::index()];

        if ty == CheckType::Queen {
            checks &= !self.safe_check::<W, B>(CheckType::Rook);
        } else if ty == CheckType::Bishop {
            checks &= !self.safe_check::<W, B>(CheckType::Queen);
        }
        
        checks &= !self.all_attacks[B::index()] | (self.weak_squares::<W, B>() & self.all_doubled_attacks[W::index()]);

        if ty == CheckType::Queen {
            checks &= !self.all_queen_attacks[B::index()].0;
        }

        checks
    }

    /// The friendly pieces that attack squares in the king ring. Pawns which attack two squares in
    /// the king ring are part of a separate bitboard. 
    ///
    /// Returns: `(attackers (+ double pawns attacks), double pawn attacks)`
    // TODO: Cache this
    pub fn king_attackers_origin<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut attacked = BitBoard(0);
        let mut pawn_attacked = BitBoard(0);
        let mut double_pawn_attacked = BitBoard(0);
        let king_ring = self.king_ring::<W, B>(false);
        let mut attacked_king_ring = king_ring & self.all_attacks[W::index()];

        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let pawns_on_king_ring = Precomputed::pawn_attacks(king_ring, B::is_white()) & pawns;
        let shared_pawn_attack = (pawns_on_king_ring & pawns_on_king_ring.shifted_2d(W::offset(2, 0)));

        while attacked_king_ring.0 != 0 {
            let sqr = Coord::from_idx(attacked_king_ring.pop_lsb() as i8);
            let pawn_attacks = self.pawn_attack::<W, B>(sqr);
            attacked |= self.knight_attack::<W, B>(sqr)
                | self.bishop_xray_attack::<W, B>(sqr)
                | self.rook_xray_attack::<W, B>(sqr)
                | self.queen_attack::<W, B>(sqr);

            double_pawn_attacked |= pawn_attacked & pawn_attacks;
            pawn_attacked |= pawn_attacks;
        }

        pawn_attacked &= !shared_pawn_attack;

        (attacked | pawn_attacked, double_pawn_attacked)
    }

    const KING_ATTACK_WEIGHTS: [i32; 4] = [81, 52, 44, 10]; // Knight, bishop, rook, queen
    pub fn king_attackers_weight<W: Color, B: Color>(&self) -> i32 {
        let attacks = self.king_attackers_origin::<W, B>().0;

        (attacks & self.board.piece_bitboards[W::piece(Piece::KNIGHT)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[0]
        + (attacks & self.board.piece_bitboards[W::piece(Piece::BISHOP)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[1]
        + (attacks & self.board.piece_bitboards[W::piece(Piece::ROOK)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[2]
        + (attacks & self.board.piece_bitboards[W::piece(Piece::QUEEN)]).count() as i32 * Self::KING_ATTACK_WEIGHTS[3]
    }

    pub fn king_attacks<W: Color, B: Color>(&self, king_attackers_origin: BitBoard) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = king_attackers_origin & (self.board.color_bitboards[W::index()] 
            & !(self.board.piece_bitboards[W::piece(Piece::PAWN)] | self.board.piece_bitboards[W::piece(Piece::KING)]));

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            let mut adjacent = Precomputed::diagonal_directions(self.king_square::<B, W>()) 
                | Precomputed::orthogonal_directions(self.king_square::<B, W>());

            let mut v = 0;
            while adjacent.0 != 0 {
                let s = Coord::from_idx(adjacent.pop_lsb() as i8);
                v += (self.knight_attack::<W, B>(s) & sqr.to_bitboard()).count() as i32
                    + (self.bishop_xray_attack::<W, B>(s) & sqr.to_bitboard()).count() as i32
                    + (self.rook_xray_attack::<W, B>(s) & sqr.to_bitboard()).count() as i32
                    + (self.queen_attack::<W, B>(s) & sqr.to_bitboard()).count() as i32;
            }

            eval[sqr] = v;
        }

        eval
    }

    pub fn weak_bonus<W: Color, B: Color>(&self) -> BitBoard {
        self.weak_squares::<W, B>() & self.king_ring::<W, B>(false)
    }

    pub fn weak_squares<W: Color, B: Color>(&self) -> BitBoard {
        self.all_attacks[W::index()] 
            & !self.all_doubled_attacks[B::index()]
            & (!self.all_attacks[B::index()] | self.all_king_attacks[B::index()] | self.all_queen_attacks[B::index()].0)
    }

    pub fn unsafe_checks<W: Color, B: Color>(&self) -> BitBoard {
        (self.check::<W, B>(CheckType::Knight) 
            & (if self.safe_check::<W, B>(CheckType::Knight).count() == 0 { BitBoard::ALL } else { BitBoard(0) }))
        | (self.check::<W, B>(CheckType::Bishop) 
            & (if self.safe_check::<W, B>(CheckType::Bishop).count() == 0 { BitBoard::ALL } else { BitBoard(0) }))
        | (self.check::<W, B>(CheckType::Rook) 
            & (if self.safe_check::<W, B>(CheckType::Rook).count() == 0 { BitBoard::ALL } else { BitBoard(0) }))
    }

    pub fn knight_defender<W: Color, B: Color>(&self) -> BitBoard {
        self.all_knight_attacks[W::index()].0 & self.all_king_attacks[W::index()]
    }

    pub fn blockers_for_king<W: Color, B: Color>(&self) -> BitBoard {
        (self.pin_rays[B::index()].0 & self.board.color_bitboards[B::index()])
            | (self.pin_rays[B::index()].1 & self.board.color_bitboards[W::index()])
    }

    // TODO: Cache this
    pub fn king_flank<W: Color, B: Color>(&self) -> BitBoard {
        let mut flank = !BitBoard::from_ranks(W::ranks(0..=2));
        let kx = self.king_square::<B, W>().file();

        if kx == 0 { flank &= !BitBoard::from_files(3..=7) };
        if kx < 3 { flank &= !BitBoard::from_files(4..=7) };
        if (3..5).contains(&kx) { flank &= !(BitBoard::from_files(0..=1) | BitBoard::from_files(6..=7)) };
        if kx >= 5 { flank &= !BitBoard::from_files(0..=3) };
        if kx == 7 { flank &= !BitBoard::from_files(0..=4) };

        flank
        
    }

    /// Returns `(attacked once or more, attacked twice)`
    pub fn flank_attack<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut attacked = self.king_flank::<W, B>();
        let a = self.all_doubled_attacks[W::index()];
        attacked &= self.all_attacks[W::index()];
        (attacked, attacked & a)
    }

    pub fn flank_defense<W: Color, B: Color>(&self) -> BitBoard {
        let mut flank = self.king_flank::<W, B>();
        flank & self.all_attacks[B::index()]
    }

    pub fn king_danger<W: Color, B: Color>(&self, shelter_strength: (i32, i32, i32)) -> i32 {
        let king_attackers_origin = self.king_attackers_origin::<W, B>();
        let count = (king_attackers_origin.0.count() + king_attackers_origin.1.count()) as i32;
        let weight = self.king_attackers_weight::<W, B>();
        let king_attacks = self.king_attacks::<W, B>(king_attackers_origin.0).count();
        let weak = self.weak_bonus::<W, B>().count() as i32;
        let unsafe_checks = self.unsafe_checks::<W, B>().count() as i32;
        let blockers_for_king = self.blockers_for_king::<W, B>().count() as i32;
        let flank_attack_total = self.flank_attack::<W, B>();
        let king_flank_attack = (flank_attack_total.0.count() + flank_attack_total.1.count()) as i32;
        let king_flank_defense = self.flank_defense::<W, B>().count() as i32;
        let no_queen = if self.queen_count::<W, B>() > 0 { 0 } else { 1 };

        let v = count * weight
            + 69 * king_attacks
            + 185 * weak
            - 100 * (if self.knight_defender::<B, W>().count() > 0 { 1 } else { 0 })
            + 148 * unsafe_checks
            + 98 * blockers_for_king
            - 4 * king_flank_defense
            + (3 * king_flank_attack * king_flank_attack / 8)
            - 873 * no_queen
            - (6 * (shelter_strength.0 - shelter_strength.1) / 8)
            + self.mobility_bonus[W::index()].0 - self.mobility_bonus[B::index()].0
            + 37
            + (772.0 * (self.safe_check::<W, B>(CheckType::Queen).count() as f32).min(1.45)) as i32
            + (1084.0 * (self.safe_check::<W, B>(CheckType::Rook).count() as f32).min(1.75)) as i32
            + (645.0 * (self.safe_check::<W, B>(CheckType::Bishop).count() as f32).min(1.50)) as i32
            + (792.0 * (self.safe_check::<W, B>(CheckType::Knight).count() as f32).min(1.62)) as i32;
        ;

        if v > 100 { v } else { 0 }
    }

    // Returns `(mg, eg)`
    pub fn king<W: Color, B: Color>(&self) -> (i32, i32) {
        let mut mg = 0;
        let shelter_strength = self.shelter_strength_storm_eg::<W, B>();
        let kd = self.king_danger::<W, B>(shelter_strength);
        let flank = self.flank_attack::<W, B>();
        let pawnless_flank = self.pawnless_flank::<W, B>();
        mg -= shelter_strength.0;
        mg += shelter_strength.1;
        mg += kd * kd / 4096;
        mg += 8 * (flank.0.count() + flank.1.count()) as i32;
        mg += 17 * if pawnless_flank { 1 } else { 0 };

        let mut eg = 0;
        eg -= 16 * self.king_pawn_distance::<W, B>();
        eg += shelter_strength.2;
        eg += 95 * if pawnless_flank { 1 } else { 0 };
        eg += kd / 16;

        (mg, eg)
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("1r3q1R/4n3/3k1pR1/p7/3B2pr/Q6P/P7/4N1RK b kq - 9 6")]
    fn test_pawnless_flank() {
        assert_eval!(- pawnless_flank, false, false, eval);
    }

    #[test]
    #[evaluation_test("2b1k3/1ppp1ppr/r1nb4/pB1Np1qp/3n1P2/4PQ1N/PPPP2PP/R1B2RK1 w Q - 8 8")]
    fn test_strength_square() {
        assert_eval!(+ - strength_square, -660, -1578, eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_storm_square() {
        assert_eval!(+ - storm_square, 672, 2579, eval; false);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_shelter_strength_storm() {
        assert_eval!(- shelter_strength_storm_eg, (-2, -27, 5), (76, 17, 5), eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_king_pawn_distance() {
        assert_eval!(- king_pawn_distance, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_check() {
        assert_eval!(+ - check, 11, 2, eval; CheckType::All);
    }

    #[test]
    #[evaluation_test("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_safe_check() {
        assert_eval!(+ - safe_check, 2, 1, eval; CheckType::All);
    }

    #[test]
    #[evaluation_test("2q3BB/N3n3/2Qr2pk/Pp1NKp1P/pPR1PrPb/4Pp2/p3b3/4Rn2 w - - 0 1")]
    fn test_king_attackers_origin() {
        assert_eval!(* - [0, 1] king_attackers_origin, (5, 1), (7, 0), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3PnB/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_king_attackers_weight() {
        assert_eval!(- king_attackers_weight, 54, 135, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_weak_bonus() {
        assert_eval!(+ - weak_bonus, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")]
    fn test_weak_squares() {
        assert_eval!(+ - weak_squares, 22, 20, eval);
    }

    #[test]
    #[evaluation_test("3q4/4p1p1/bn1rpPp1/kr1n1bNp/P1N2P1P/3P2R1/3PP1P1/1RBQKB2 b KQkq - 3 3")]
    fn test_unsafe_checks() {
        assert_eval!(+ - unsafe_checks, 2, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK w Qkq - 5 4")]
    fn test_knight_defender() {
        assert_eval!(+ - knight_defender, 1, 6, eval);
    }

    #[test]
    #[evaluation_test("1K6/6R1/1P1kPp2/4q1P1/p1r2Np1/4P2r/1Qn5/8 w - - 0 1")]
    fn test_blockers_for_king() {
        assert_eval!(+ - blockers_for_king, 0, 1, eval);
    }

    #[test]
    #[evaluation_test("1nb2rk1/p2rb3/p5P1/2K3N1/pP1P1BQ1/2Npq3/2P1P1PR/1R3B2 w q - 4 10")]
    fn test_flank_attack() {
        assert_eval!(* - [0, 1] flank_attack, (12, 8), (9, 1), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_flank_defense() {
        assert_eval!(+ - flank_defense, 19, 11, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_king() {
        assert_eval!(- king, (1812, 138), (3168, 210), eval);
    }
}

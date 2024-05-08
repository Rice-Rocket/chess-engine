use crate::board::coord::Coord;
use super::state::State;

pub fn pawnless_flank(state: &State) -> i32 {
    todo!();
}

pub fn strength_square(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn storm_square(state: &State, eg: bool, sqr: Coord) -> i32 {
    todo!();
}

pub fn shelter_strength(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn shelter_storm(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_pawn_distance(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn check(state: &State, ty: u8, sqr: Coord) -> i32 {
    todo!();
}

pub fn safe_check(state: &State, ty: u8, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_attackers_count(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_attackers_weight(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_attacks(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn weak_bonus(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn weak_squares(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn unsafe_checks(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn knight_defender(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn endgame_shelter(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn blockers_for_king(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn flank_attack(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn flank_defense(state: &State, sqr: Coord) -> i32 {
    todo!();
}

pub fn king_danger(state: &State) -> i32 {
    todo!();
}

pub fn king_mg(state: &State) -> i32 {
    todo!();
}

pub fn king_eg(state: &State) -> i32 {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{sum_sqrs, assert_eval, eval::state::Color, Board, Zobrist};
    use super::*;

    #[test]
    fn test_pawnless_flank() {
        assert_eval!(- pawnless_flank, 1, 0, "3q4/4p1p1/bn1rpPp1/kr3bNp/2NPnP1P/3P2P1/3PPR2/1RBQKB2 w KQkq - 2 3");
    }

    #[test]
    fn test_strength_square() {
        assert_eval!(strength_square, -2768, -956, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_storm_square() {
        assert_eval!(storm_square, 672, 2579, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3"; false);
    }

    #[test]
    fn test_shelter_strength() {
        assert_eval!(shelter_strength, -2, 76, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_shelter_storm() {
        assert_eval!(shelter_storm, -27, 17, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_king_pawn_distance() {
        assert_eval!(king_pawn_distance, 2, 1, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_check() {
        assert_eval!(check, 11, 2, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3"; 0);
    }

    #[test]
    fn test_safe_check() {
        assert_eval!(safe_check, 2, 1, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3"; 0);
    }

    #[test]
    fn test_king_attackers_count() {
        assert_eval!(king_attackers_count, 4, 4, "nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_king_attackers_weight() {
        assert_eval!(king_attackers_weight, 54, 135, "1r3q1R/p1p1nR2/n2k1pn1/pQ3PnB/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_king_attacks() {
        assert_eval!(king_attacks, 6, 1, "1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_weak_bonus() {
        assert_eval!(weak_bonus, 1, 2, "1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_weak_squares() {
        assert_eval!(weak_squares, 22, 20, "1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3");
    }

    #[test]
    fn test_unsafe_checks() {
        assert_eval!(unsafe_checks, 2, 0, "3q4/4p1p1/bn1rpPp1/kr1n1bNp/P1N2P1P/3P2R1/3PP1P1/1RBQKB2 b KQkq - 3 3");
    }

    #[test]
    fn test_knight_defender() {
        assert_eval!(knight_defender, 1, 6, "1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK w Qkq - 5 4");
    }

    #[test]
    fn test_endgame_shelter() {
        assert_eval!(endgame_shelter, 5, 11, "1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK b kq - 8 5");
    }

    #[test]
    fn test_blockers_for_king() {
        assert_eval!(blockers_for_king, 2, 1, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    fn test_flank_attack() {
        assert_eval!(flank_attack, 17, 16, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    fn test_flank_defense() {
        assert_eval!(flank_defense, 19, 11, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    fn test_king_danger() {
        assert_eval!(- king_danger, 2640, 3448, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    fn test_king_mg() {
        assert_eval!(- king_mg, 1812, 3168, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }

    #[test]
    fn test_king_eg() {
        assert_eval!(- king_eg, 138, 210, "1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6");
    }
}

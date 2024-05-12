use proc_macro_utils::flipped_eval;

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
    #[flipped_eval]
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
    #[flipped_eval]
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
    #[flipped_eval]
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

        v
    }

    /// Returns `(shelter_strength, shelter_storm)`
    // TODO: Cache the result of this function.
    #[flipped_eval]
    pub fn shelter_strength_storm(&self) -> (i32, i32) {
        let mut w = 0;
        let mut s = 1024;

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
                if (s1 - w1 < s - w) {
                    w = w1;
                    s = s1;
                }
            }
        }

        (w, s)
    }

    /// The minimum distance from the friendly king to a friendly pawn
    #[flipped_eval]
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
    #[flipped_eval]
    pub fn check(&self, ty: CheckType, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !self.board.piece_bitboards[self.color.flip().piece(Piece::QUEEN)];
        let mut checks = BitBoard(0);

        // if ty == CheckType::Rook || ty == CheckType::All || ty == CheckType::NotQueen {
        //     
        // }

        checks
    }

    /// The positions to which friendly pieces could move to deliver check without being captured. 
    #[flipped_eval]
    pub fn safe_check(&self, ty: CheckType, sqr: Coord) -> BitBoard {
        todo!();
    }

    #[flipped_eval]
    pub fn king_attackers_count(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn king_attackers_weight(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn king_attacks(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn weak_bonus(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn weak_squares(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn unsafe_checks(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn knight_defender(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn endgame_shelter(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn blockers_for_king(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn flank_attack(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn flank_defense(&self, sqr: Coord) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn king_danger(&self) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn king_mg(&self) -> i32 {
        todo!();
    }

    #[flipped_eval]
    pub fn king_eg(&self) -> i32 {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    fn test_pawnless_flank() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("3q4/4p1p1/bn1rpPp1/kr3bNp/2NPnP1P/3P2P1/3PPR2/1RBQKB2 w KQkq - 2 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- friendly_pawnless_flank, true, false, eval);
    }

    #[test]
    fn test_strength_square() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("2b1k3/1ppp1ppr/r1nb4/pB1Np1qp/3n1P2/4PQ1N/PPPP2PP/R1B2RK1 w Q - 8 8")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_strength_square, -660, -1578, eval);
    }

    #[test]
    fn test_storm_square() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_storm_square, 672, 2579, eval; false);
    }

    #[test]
    fn test_shelter_strength_storm() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- friendly_shelter_strength_storm, (-2, -27), (76, 17), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_pawn_distance() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- friendly_king_pawn_distance, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_check() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_check, 11, 2, eval; CheckType::All);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_safe_check() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_safe_check, 2, 1, eval; CheckType::All);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attackers_count() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nr3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_king_attackers_count, 4, 4, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attackers_weight() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3PnB/1bP2qp1/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_king_attackers_weight, 54, 135, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_attacks() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r1PP/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_king_attacks, 6, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_bonus() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_weak_bonus, 1, 2, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_weak_squares() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2BN2RK b Qkq - 4 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_weak_squares, 22, 20, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_unsafe_checks() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("3q4/4p1p1/bn1rpPp1/kr1n1bNp/P1N2P1P/3P2R1/3PP1P1/1RBQKB2 b KQkq - 3 3")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_unsafe_checks, 2, 0, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_knight_defender() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK w Qkq - 5 4")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_knight_defender, 1, 6, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_endgame_shelter() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1nR2/n2k1pn1/pQ3P1B/1bP2qpn/QP2r2P/P1P1P3/2B1N1RK b kq - 8 5")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_endgame_shelter, 5, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_blockers_for_king() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_blockers_for_king, 2, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_flank_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_flank_attack, 17, 16, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_flank_defense() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(friendly_flank_defense, 19, 11, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_danger() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- friendly_king_danger, 2640, 3448, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_mg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- friendly_king_mg, 1812, 3168, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    fn test_king_eg() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(- friendly_king_eg, 138, 210, eval);
    }
}

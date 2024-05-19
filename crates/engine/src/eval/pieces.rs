use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, color::{Color, White, Black}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn outpost<W: Color, B: Color>(&self) -> BitBoard {
        self.outpost_square::<W, B>() 
            & (self.board.piece_bitboards[W::piece(Piece::KNIGHT)] 
            | self.board.piece_bitboards[W::piece(Piece::BISHOP)])
    }

    pub fn outpost_square<W: Color, B: Color>(&self) -> BitBoard {
        BitBoard::from_ranks(W::ranks(4..=5))
            & self.all_pawn_attacks::<W, B>().0
            & !self.pawn_attacks_span::<W, B>()
    }

    /// Returns `(not supported by pawn, supported by pawn)`
    pub fn reachable_outpost<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut reachable = (self.all_knight_attacks::<W, B>().0 | self.all_bishop_xray_attacks::<W, B>().0)
            & self.outpost_square::<W, B>() & !self.board.color_bitboards[W::index()];
        let supported = reachable & self.all_pawn_attacks::<W, B>().0;
        let mut supported_origins = BitBoard(0);
        let mut origins = BitBoard(0);

        while reachable.0 != 0 {
            let sqr = Coord::from_idx(reachable.pop_lsb() as i8);
            let has_support = supported.contains_square(sqr.square());
            let attacks = self.bishop_xray_attack::<W, B>(None, sqr) | self.knight_attack::<W, B>(None, sqr);
            if has_support {
                supported_origins |= attacks;
            } else {
                origins |= attacks;
            }
        }

        (origins, supported_origins)
    }

    pub fn minor_behind_pawn<W: Color, B: Color>(&self) -> BitBoard {
        (self.board.piece_bitboards[W::piece(Piece::KNIGHT)] | self.board.piece_bitboards[W::piece(Piece::BISHOP)])
            & (self.board.piece_bitboards[Piece::new(Piece::WHITE_PAWN)] 
               | self.board.piece_bitboards[Piece::new(Piece::BLACK_PAWN)]).shifted_2d(W::down())
    }

    pub fn bishop_pawns<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        if !self.board.piece_bitboards[W::piece(Piece::BISHOP)].contains_square(sqr.square()) {
            return 0;
        }
        let is_light_sqr = (sqr.file() + sqr.rank()) % 2 != 0;
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let on_sqr_col = pawns
            & (if is_light_sqr { BitBoard::LIGHT_SQUARES } else { BitBoard::DARK_SQUARES });
        let blocked = pawns & BitBoard::from_files(2..=5) & self.board.all_pieces_bitboard.shifted_2d(W::down());

        let attacked = if self.pawn_attack::<W, B>(None, sqr).count() > 0 { 0 } else { 1 };
        on_sqr_col.count() as i32 * (blocked.count() as i32 + attacked)
    }

    /// Returns `(on open file, on semi-open file)`
    ///
    /// A semi-open file is defined as having only enemy pawns.
    pub fn rook_on_file<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
        let mut open = BitBoard(0);
        let mut semiopen = BitBoard(0);

        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            let file = BitBoard::FILES[sqr.file() as usize];
            if (self.board.piece_bitboards[W::piece(Piece::PAWN)] & file).0 == 0 {
                if (self.board.piece_bitboards[B::piece(Piece::PAWN)] & file).0 == 0 {
                    open |= sqr.to_bitboard();
                } else {
                    semiopen |= sqr.to_bitboard();
                }
            }
        }

        (open, semiopen)
    }

    pub fn trapped_rook<W: Color, B: Color>(&self) -> BitBoard {
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
    
        let on_file = self.rook_on_file::<W, B>();
        rooks &= !(on_file.0 | on_file.1);
        
        let kx = self.king_square::<W, B>().file();
        rooks &= if kx < 4 {
            BitBoard::from_cond(|s| s.file() < kx)
        } else {
            BitBoard::from_cond(|s| s.file() >= kx)
        };

        rooks & BitBoard::from_cond(|s| self.mobility::<W, B>(s) <= 3)
    }

    pub fn weak_queen<W: Color, B: Color>(&self) -> BitBoard {
        let mut queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)];
        let mut weak = BitBoard(0);

        'queens: while queens.0 != 0 {
            let queen_sqr = Coord::from_idx(queens.pop_lsb() as i8);
            for dir in 0..8 {
                let is_diagonal = dir > 3;
                let slider = if is_diagonal { self.diagonal_sliders::<B, W>() } else { self.orthogonal_sliders::<B, W>() };
                if (self.precomp.dir_ray_mask[queen_sqr][dir] & slider).0 == 0 { continue; }

                let n = self.precomp.num_sqrs_to_edge[queen_sqr][dir];
                let dir_offset = self.precomp.direction_offsets[dir];
                let mut is_piece_along_ray = false;

                for i in 0..n {
                    let sqr = queen_sqr + dir_offset * (i + 1);
                    let piece = self.board.square[sqr];

                    if piece != Piece::NULL {
                        if is_piece_along_ray {
                            if is_diagonal && piece.piece_type() == Piece::BISHOP
                            || !is_diagonal && piece.piece_type() == Piece::ROOK {
                                weak |= queen_sqr.to_bitboard();
                                continue 'queens;
                            }
                        } else {
                            is_piece_along_ray = true;
                        }
                    }
                }
            }
        }

        weak
    }

    pub fn king_protector<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        if self.board.piece_bitboards[W::piece(Piece::KNIGHT)].contains_square(sqr.square())
        || self.board.piece_bitboards[W::piece(Piece::BISHOP)].contains_square(sqr.square()) {
            self.king_distance::<W, B>(sqr)
        } else {
            0
        }
    }

    const LONG_DIAGONALS: BitBoard = BitBoard(0b10000001_01000010_00100100_00011000_00011000_00100100_01000010_10000001);
    const CENTER_SQUARES: BitBoard = BitBoard(0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000);
    pub fn long_diagonal_bishop<W: Color, B: Color>(&self) -> BitBoard {
        let mut bishops = self.board.piece_bitboards[W::piece(Piece::BISHOP)] & Self::LONG_DIAGONALS;
        let mut attacks_center = BitBoard(0);
        let blockers = self.board.piece_bitboards[W::piece(Piece::PAWN)] | self.board.piece_bitboards[B::piece(Piece::PAWN)];

        while bishops.0 != 0 {
            let sqr = Coord::from_idx(bishops.pop_lsb() as i8);
            let attacks = self.magics.get_bishop_attacks(sqr, blockers) & !blockers;
            if (attacks & Self::CENTER_SQUARES).0 != 0 {
                attacks_center |= sqr.to_bitboard();
            }
        }

        attacks_center
    }

    pub fn outpost_total<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        let knight = self.board.piece_bitboards[W::piece(Piece::KNIGHT)].contains_square(sqr.square());
        if !knight && !self.board.piece_bitboards[W::piece(Piece::BISHOP)].contains_square(sqr.square()) {
            return 0;
        }

        let mut reachable = 0;
        if !self.outpost::<W, B>().contains_square(sqr.square()) {
            if !knight { return 0 };
            let reachable_outpost = self.reachable_outpost::<W, B>();
            reachable = if reachable_outpost.0.contains_square(sqr.square()) { 1 } else { 0 }
                + if reachable_outpost.1.contains_square(sqr.square()) { 2 } else { 0 };
            if reachable == 0 { return 0 };
            return 1;
        }

        if knight && (sqr.file() < 2 || sqr.file() > 5) {
            let mut ea = false;
            let mut count = 0;
            for s in Coord::iter_squares() {
                if ((sqr.file() - s.file()).abs() == 2 && (sqr.rank() - s.rank()).abs() == 1
                || (sqr.file() - s.file()).abs() == 1 && (sqr.rank() - s.rank()).abs() == 2)
                && self.board.color_bitboards[B::index()].contains_square(s.square()) {
                    ea = true;
                }
                if ((s.file() < 4 && sqr.file() < 4) || (s.file() >= 4 && sqr.file() >= 4))
                && self.board.color_bitboards[B::index()].contains_square(s.square()) {
                    count += 1;
                }
            }
            if !ea && count <= 1 {
                return 2;
            }
        }

        if knight { 4 } else { 3 }
    }

    pub fn rook_on_queen_file<W: Color, B: Color>(&self) -> BitBoard {
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
        let queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)] | self.board.piece_bitboards[B::piece(Piece::QUEEN)];
        let mut on_queen_file = BitBoard(0);

        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            if (BitBoard::FILES[sqr.file() as usize] & queens).0 != 0 {
                on_queen_file |= sqr.to_bitboard();
            }
        }

        on_queen_file
    }

    pub fn bishop_xray_pawns<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        if !self.board.piece_bitboards[W::piece(Piece::BISHOP)].contains_square(sqr.square()) { return 0 };
        (self.board.piece_bitboards[B::piece(Piece::PAWN)] & self.magics.get_bishop_attacks(sqr, BitBoard(0))).count() as i32
    }

    pub fn rook_on_king_ring<W: Color, B: Color>(&self) -> BitBoard {
        let mut on_king_ring = BitBoard(0);
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
        rooks &= !self.king_attackers_origin::<W, B>().0;

        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            if (BitBoard::FILES[sqr.file() as usize] & self.king_ring[W::index()]).0 != 0 {
                on_king_ring |= sqr.to_bitboard();
            }
        }

        on_king_ring
    }

    pub fn bishop_on_king_ring<W: Color, B: Color>(&self) -> BitBoard {
        let mut on_king_ring = BitBoard(0);
        let mut bishops = self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        bishops &= !self.king_attackers_origin::<W, B>().0;
        let blockers = self.board.piece_bitboards[W::piece(Piece::PAWN)] 
            | self.board.piece_bitboards[B::piece(Piece::PAWN)];

        while bishops.0 != 0 {
            let sqr = Coord::from_idx(bishops.pop_lsb() as i8);
            let ray = self.magics.get_bishop_attacks(sqr, blockers);
            if (ray & self.king_ring[W::index()]).0 != 0 {
                on_king_ring |= sqr.to_bitboard();
            }
        }

        on_king_ring
    }

    pub fn queen_infiltration<W: Color, B: Color>(&self) -> BitBoard {
        let mut queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)];
        queens &= B::home_side();
        queens &= !self.all_pawn_attacks::<B, W>().0;
        queens &= !self.pawn_attacks_span::<W, B>();
        queens
    }

    const OUTPOST_TOTAL_VALS_MG: [i32; 5] = [0, 31, -7, 30, 56];
    pub fn pieces_mg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        if !(self.board.color_bitboards[W::index()] 
        & !self.board.piece_bitboards[W::piece(Piece::PAWN)]
        & !self.board.piece_bitboards[W::piece(Piece::KING)]).contains_square(sqr.square()) {
            return 0;
        }

        let mut v = 0;
        v += Self::OUTPOST_TOTAL_VALS_MG[self.outpost_total::<W, B>(sqr) as usize];
        v += if self.minor_behind_pawn::<W, B>().contains_square(sqr.square()) { 18 } else { 0 };
        v -= 3 * self.bishop_pawns::<W, B>(sqr);
        v -= 4 * self.bishop_xray_pawns::<W, B>(sqr);
        v += if self.rook_on_queen_file::<W, B>().contains_square(sqr.square()) { 6 } else { 0 };
        v += if self.rook_on_king_ring::<W, B>().contains_square(sqr.square()) { 16 } else { 0 };
        v += if self.bishop_on_king_ring::<W, B>().contains_square(sqr.square()) { 24 } else { 0 };

        let rook_on_file = self.rook_on_file::<W, B>();
        v += if rook_on_file.1.contains_square(sqr.square()) { 19 } else { 0 };
        v += if rook_on_file.0.contains_square(sqr.square()) { 48 } else { 0 };
        
        v -= if self.trapped_rook::<W, B>().contains_square(sqr.square()) { 
            55 * (if self.board.current_state.has_kingside_castle_right(W::is_white()) 
                  || self.board.current_state.has_queenside_castle_right(W::is_white()) { 1 } else { 2 })
        } else { 0 };
        
        v -= if self.weak_queen::<W, B>().contains_square(sqr.square()) { 56 } else { 0 };
        v -= if self.queen_infiltration::<W, B>().contains_square(sqr.square()) { 2 } else { 0 };
        v -= self.king_protector::<W, B>(sqr) 
            * if self.board.piece_bitboards[W::piece(Piece::KNIGHT)].contains_square(sqr.square()) { 8 } else { 6 };
        v += if self.long_diagonal_bishop::<W, B>().contains_square(sqr.square()) { 45 } else { 0 };

        v
    }

    const OUTPOST_TOTAL_VALS_EG: [i32; 5] = [0, 22, 36, 23, 36];
    pub fn pieces_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        if !(self.board.color_bitboards[W::index()] 
        & !self.board.piece_bitboards[W::piece(Piece::PAWN)]
        & !self.board.piece_bitboards[W::piece(Piece::KING)]).contains_square(sqr.square()) {
            return 0;
        }
        
        let mut v = 0;
        v += Self::OUTPOST_TOTAL_VALS_EG[self.outpost_total::<W, B>(sqr) as usize];
        v += if self.minor_behind_pawn::<W, B>().contains_square(sqr.square()) { 3 } else { 0 };
        v -= 7 * self.bishop_pawns::<W, B>(sqr);
        v -= 5 * self.bishop_xray_pawns::<W, B>(sqr);
        v += if self.rook_on_queen_file::<W, B>().contains_square(sqr.square()) { 11 } else { 0 };

        let rook_on_file = self.rook_on_file::<W, B>();
        v += if rook_on_file.1.contains_square(sqr.square()) { 7 } else { 0 };
        v += if rook_on_file.0.contains_square(sqr.square()) { 29 } else { 0 };

        v -= if self.trapped_rook::<W, B>().contains_square(sqr.square()) {
            13 * (if self.board.current_state.has_kingside_castle_right(W::is_white())
                  || self.board.current_state.has_queenside_castle_right(W::is_white()) { 1 } else { 2 })
        } else { 0 };

        v -= if self.weak_queen::<W, B>().contains_square(sqr.square()) { 15 } else { 0 };
        v += if self.queen_infiltration::<W, B>().contains_square(sqr.square()) { 14 } else { 0 };
        v -= 9 * self.king_protector::<W, B>(sqr);

        v
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("r2qk2r/6p1/1ppNp3/p1Pn1pNp/Pb1PnPbP/6P1/1P2P3/R1BQKB1R b KQkq - 1 2")]
    fn test_outpost() {
        assert_eval!(+ - outpost, 2, 3, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n4n/p2knpRp/pQp2PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_outpost_square() {
        assert_eval!(+ - outpost_square, 5, 0, eval);
    }

    #[test]
    #[evaluation_test("r2qk2r/6p1/1pp1p3/p1Pn1b1p/PbNPnP1P/5NP1/1P2P3/R1BQKB1R w KQkq - 2 3")]
    fn test_reachable_outpost() {
        assert_eval!(* - [0, 1] reachable_outpost, (0, 2), (0, 1), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n5/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_minor_behind_pawn() {
        assert_eval!(+ - minor_behind_pawn, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/3b4/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK w kq - 1 8")]
    fn test_bishop_pawns() {
        assert_eval!(bishop_pawns, 28, 11, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/3b4/pP1knpRp/pQpn1P1B/1b3q1r/5n1P/P1P2P1P/2B1N1RK w kq - 1 8")]
    fn test_rook_on_file() {
        assert_eval!(* - [0, 1] rook_on_file, (2, 0), (0, 1), eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP2q1r/5n1P/P1P2P2/2B1N1KR w kq - 1 8")]
    fn test_trapped_rook() {
        assert_eval!(+ - trapped_rook, 1, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP4r/5n1P/P1P1qP2/2B1N1KR w kq - 1 8")]
    fn test_weak_queen() {
        assert_eval!(+ - weak_queen, 1, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/1k6/p1b1npRp/pQpn1P2/1bP4r/2B2n1P/P1P1qPB1/4N1KR w kq - 1 8")]
    fn test_long_diagonal_bishop() {
        assert_eval!(+ - long_diagonal_bishop, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("r2qk2r/6p1/1pp1p3/p1Pn1bNp/PbNPnP1P/6P1/1P2P3/R1BQKB1R w KQkq - 2 3")]
    fn test_outpost_total() {
        assert_eval!(outpost_total, 5, 3, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/1k2R3/p1b1np1p/pQpnRP2/1bP4r/2B2n1P/P1P1qPB1/4N1K1 w kq - 1 8")]
    fn test_rook_on_queen_file() {
        assert_eval!(+ - rook_on_queen_file, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("1r1B1q1R/1k2R3/p3np1p/pQpnRP2/2P4r/1bB2n1P/P1P1qPB1/4N1K1 w kq - 1 8")]
    fn test_bishop_xray_pawns() {
        assert_eval!(bishop_xray_pawns, 4, 3, eval);
    }

    #[test]
    #[evaluation_test("k2q4/3r2p1/1pprp3/p1Pn1bNp/PbNPnP1P/6P1/1P2PR2/1RBQKB2 w KQkq - 2 3")]
    fn test_rook_on_king_ring() {
        assert_eval!(+ - rook_on_king_ring, 1, 2, eval);
    }

    #[test]
    #[evaluation_test("1r1B1q1R/1k6/p2Rnp1p/pQpnbP2/2P2B1r/5n1P/P1P1qPBR/4N1K1 w kq - 1 8")]
    fn test_bishop_on_king_ring() {
        assert_eval!(+ - bishop_on_king_ring, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("1r1B1q1R/1k1np1Q1/p5Rp/p1pnbP2/2P2B1r/5n1P/P1P1qPBR/4N1K1 w kq - 1 8")]
    fn test_queen_infiltration() {
        assert_eval!(+ - queen_infiltration, 1, 1, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_pieces_mg() {
        assert_eval!(pieces_mg, -121, -14, eval);
    }

    #[test]
    #[ignore = "unimplemented evaluation function"]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_pieces_eg() {
        assert_eval!(pieces_eg, -325, -105, eval);
    }
}

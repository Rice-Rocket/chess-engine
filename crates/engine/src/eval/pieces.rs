use proc_macro_utils::evaluation_fn;

use crate::{bitboard::square_values::SquareEvaluations, board::{coord::Coord, piece::Piece}, color::{Black, Color, White}, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn outpost<W: Color, B: Color>(&self) -> BitBoard {
        self.outpost_square::<W, B>() 
            & (self.board.piece_bitboards[W::piece(Piece::KNIGHT)] 
            | self.board.piece_bitboards[W::piece(Piece::BISHOP)])
    }

    pub fn outpost_square<W: Color, B: Color>(&self) -> BitBoard {
        BitBoard::from_ranks(W::ranks(3..=5))
            & self.all_pawn_attacks[W::index()].0
            & !self.pawn_attacks_span::<W, B>()
    }

    /// Returns `(not supported by pawn, supported by pawn)`
    pub fn reachable_outpost<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut reachable = (self.all_knight_attacks[W::index()].0 | self.all_bishop_attacks[W::index()].0)
            & self.outpost_square::<W, B>() & !self.board.color_bitboards[W::index()];
        let supported = reachable & self.all_pawn_attacks[W::index()].0;
        let mut supported_origins = BitBoard(0);
        let mut origins = BitBoard(0);
        let knights = self.board.piece_bitboards[W::piece(Piece::KNIGHT)];

        while reachable.0 != 0 {
            let sqr = Coord::from_idx(reachable.pop_lsb() as i8);
            let has_support = supported.contains_square(sqr.square());
            let attacks = self.knight_attack::<W, B>(None, sqr) | self.bishop_xray_attack::<W, B>(None, sqr);
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

    pub fn bishop_pawns<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        let pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);

            let is_light_sqr = (sqr.file() + sqr.rank()) % 2 != 0;
            let on_sqr_col = pawns
                & (if is_light_sqr { BitBoard::LIGHT_SQUARES } else { BitBoard::DARK_SQUARES });
            let blocked = pawns & BitBoard::from_files(2..=5) & self.board.all_pieces_bitboard.shifted_2d(W::down());

            let attacked = if self.pawn_attack::<W, B>(None, sqr).count() > 0 { 0 } else { 1 };
            eval[sqr] = on_sqr_col.count() as i32 * (blocked.count() as i32 + attacked);
        }

        eval
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

        rooks & BitBoard::from_cond(|s| self.mobility::<W, B>()[s] <= 3)
    }

    pub fn weak_queen<W: Color, B: Color>(&self) -> BitBoard {
        let mut queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)];
        let mut weak = BitBoard(0);

        'queens: while queens.0 != 0 {
            let queen_sqr = Coord::from_idx(queens.pop_lsb() as i8);
            'dirs: for dir in 0..8 {
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
                            } else {
                                continue 'dirs;
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

    pub fn king_protector<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.board.piece_bitboards[W::piece(Piece::KNIGHT)]
            | self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        let king_distance = self.king_distance::<W, B>();

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            eval[sqr] = king_distance[sqr];
        }
        
        eval
    }

    const LONG_DIAGONALS: BitBoard = BitBoard(0b10000001_01000010_00100100_00011000_00011000_00100100_01000010_10000001);
    const CENTER_SQUARES: BitBoard = BitBoard(0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000);
    pub fn long_diagonal_bishop<W: Color, B: Color>(&self) -> BitBoard {
        let mut bishops = self.board.piece_bitboards[W::piece(Piece::BISHOP)] & Self::LONG_DIAGONALS & !Self::CENTER_SQUARES;
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

    pub fn outpost_total<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let knights = self.board.piece_bitboards[W::piece(Piece::KNIGHT)];
        let mut sqrs = knights | self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        let outposts = self.outpost::<W, B>();

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            let knight = knights.contains_square(sqr.square());

            let mut reachable = 0;
            if !outposts.contains_square(sqr.square()) {
                if !knight { continue };
                let reachable_outpost = self.reachable_outpost::<W, B>();
                reachable = if reachable_outpost.0.contains_square(sqr.square()) { 1 } else { 0 }
                + if reachable_outpost.1.contains_square(sqr.square()) { 2 } else { 0 };
                if reachable == 0 { continue };

                eval[sqr] = 1;
                continue;
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
                    eval[sqr] = 2;
                    continue;
                }
            }

            eval[sqr] = if knight { 4 } else { 3 };
        }

        eval
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

    pub fn bishop_xray_pawns<W: Color, B: Color>(&self) -> SquareEvaluations {
        let mut eval = SquareEvaluations::new();
        let mut sqrs = self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        let enemy_pawns = self.board.piece_bitboards[B::piece(Piece::PAWN)];

        while sqrs.0 != 0 {
            let sqr = Coord::from_idx(sqrs.pop_lsb() as i8);
            eval[sqr] = (enemy_pawns & self.magics.get_bishop_attacks(sqr, BitBoard(0))).count() as i32
        }

        eval
    }

    pub fn rook_on_king_ring<W: Color, B: Color>(&self) -> BitBoard {
        let mut on_king_ring = BitBoard(0);
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
        rooks &= !self.king_attackers_origin::<W, B>().0;

        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            if (BitBoard::FILES[sqr.file() as usize] & self.king_ring::<W, B>(false)).0 != 0 {
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
            if (ray & self.king_ring::<W, B>(false)).0 != 0 {
                on_king_ring |= sqr.to_bitboard();
            }
        }

        on_king_ring
    }

    pub fn queen_infiltration<W: Color, B: Color>(&self) -> BitBoard {
        let mut queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)];
        queens &= B::home_side();
        queens &= !self.all_pawn_attacks[B::index()].0;
        queens &= !self.pawn_attacks_span::<W, B>();
        queens
    }

    const OUTPOST_TOTAL_VALS_MG: [i32; 5] = [0, 31, -7, 30, 56];
    const OUTPOST_TOTAL_VALS_EG: [i32; 5] = [0, 22, 36, 23, 36];
    /// Returns `(mg, eg)`
    pub fn pieces<W: Color, B: Color>(&self) -> (i32, i32) {
        let mut mg = 0;
        let mut eg = 0;

        let outpost_total = self.outpost_total::<W, B>();
        let minor_behind_pawn = self.minor_behind_pawn::<W, B>();
        let bishop_pawns = self.bishop_pawns::<W, B>();
        let bishop_xray_pawns = self.bishop_xray_pawns::<W, B>();
        let rook_on_queen_file = self.rook_on_queen_file::<W, B>();
        mg += outpost_total.map(|i| Self::OUTPOST_TOTAL_VALS_MG[i as usize]).count();
        mg += 18 * minor_behind_pawn.count() as i32;
        mg -= 3 * bishop_pawns.count();
        mg -= 4 * bishop_xray_pawns.count();
        mg += 6 * rook_on_queen_file.count() as i32;
        mg += 16 * self.rook_on_king_ring::<W, B>().count() as i32;
        mg += 24 * self.bishop_on_king_ring::<W, B>().count() as i32;
        eg += outpost_total.map(|i| Self::OUTPOST_TOTAL_VALS_EG[i as usize]).count();
        eg += 3 * minor_behind_pawn.count() as i32;
        eg -= 7 * bishop_pawns.count();
        eg -= 5 * bishop_xray_pawns.count();
        eg += 11 * rook_on_queen_file.count() as i32;

        let rook_on_file = self.rook_on_file::<W, B>();
        mg += 19 * rook_on_file.1.count() as i32;
        mg += 48 * rook_on_file.0.count() as i32;
        eg += 7 * rook_on_file.1.count() as i32;
        eg += 29 * rook_on_file.0.count() as i32;

        let trapped_rook = self.trapped_rook::<W, B>();
        let castle_mult = if self.board.current_state.has_kingside_castle_right(W::is_white()) 
            || self.board.current_state.has_queenside_castle_right(W::is_white()) { 1 } else { 2 };
        mg -= trapped_rook.count() as i32
            * 55 * castle_mult;
        eg -= trapped_rook.count() as i32
            * 13 * castle_mult;

        let weak_queen = self.weak_queen::<W, B>().count() as i32;
        let queen_infiltration = self.queen_infiltration::<W, B>().count() as i32;
        mg -= 56 * weak_queen;
        mg -= 2 * queen_infiltration;
        eg -= 15 * weak_queen;
        eg += 14 * queen_infiltration;

        let king_protector = self.king_protector::<W, B>();
        let knights = self.board.piece_bitboards[W::piece(Piece::KNIGHT)];
        mg -= 8 * (king_protector & knights).count();
        mg -= 6 * (king_protector & !knights).count();
        mg += 45 * self.long_diagonal_bishop::<W, B>().count() as i32;
        eg -= 9 * king_protector.count();

        (mg, eg)
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
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_outpost_square() {
        assert_eval!(+ - outpost_square, 5, 2, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_reachable_outpost() {
        assert_eval!(* - [0, 1] reachable_outpost, (0, 1), (0, 1), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/2n5/p2knpRp/pQpn1PPB/1bP2q1r/5n1P/P1P2P2/2B1N1RK b kq - 0 7")]
    fn test_minor_behind_pawn() {
        assert_eval!(+ - minor_behind_pawn, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_bishop_pawns() {
        assert_eval!(+ - bishop_pawns, 32, 8, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/3b4/pP1knpRp/pQpn1P1B/1b3q1r/5n1P/P1P2P1P/2B1N1RK w kq - 1 8")]
    fn test_rook_on_file() {
        assert_eval!(* - [0, 1] rook_on_file, (2, 0), (0, 1), eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/1k1b4/p3npRp/pQpn1P1B/1bP2q1r/5n1P/P1P2P2/2B1N1KR w kq - 1 8")]
    fn test_trapped_rook() {
        assert_eval!(+ - trapped_rook, 1, 0, eval);
    }

    #[test]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_weak_queen() {
        assert_eval!(+ - weak_queen, 0, 0, eval);
    }

    #[test]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_long_diagonal_bishop() {
        assert_eval!(+ - long_diagonal_bishop, 1, 0, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_outpost_total() {
        assert_eval!(+ - outpost_total, 0, 1, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/1k2R3/p1b1np1p/pQpnRP2/1bP4r/2B2n1P/P1P1qPB1/4N1K1 w kq - 1 8")]
    fn test_rook_on_queen_file() {
        assert_eval!(+ - rook_on_queen_file, 2, 1, eval);
    }

    #[test]
    #[evaluation_test("nr1B1q2/1k2p1Q1/p5Rp/p1pnbP2/R1P2B1r/2P2n1P/P3qPBR/4N1K1 w kq - 1 8")]
    fn test_bishop_xray_pawns() {
        assert_eval!(+ - bishop_xray_pawns, 3, 1, eval);
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
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_pieces() {
        assert_eval!(- pieces, (-50, -191), (-1, -92), eval);
    }
}

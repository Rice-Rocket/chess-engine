use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, move_gen::magics::Magics, color::{Black, Color, White}, precomp::Precomputed, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn king_square<W: Color, B: Color>(&self) -> Coord {
        self.board.king_square[W::index()]
    }

    pub fn diagonal_sliders<W: Color, B: Color>(&self) -> BitBoard {
        if W::is_white() == self.board.white_to_move {
            self.board.friendly_diagonal_sliders
        } else {
            self.board.enemy_diagonal_sliders
        }
    }

    pub fn orthogonal_sliders<W: Color, B: Color>(&self) -> BitBoard {
        if W::is_white() == self.board.white_to_move {
            self.board.friendly_orthogonal_sliders
        } else {
            self.board.enemy_orthogonal_sliders
        }
    }

    /// Requires: `king_square`
    pub fn pin_rays<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut pin_rays = BitBoard(0);
        let mut blocker_rays = BitBoard(0);
        let mut start_dir_idx = 0;
        let mut end_dir_idx = 8;
        let mut in_double_check = false;
        let mut in_check = false;

        // Don't calculate unecessary directions
        if self.board.piece_bitboards[B::piece(Piece::QUEEN)].count() == 0 {
            start_dir_idx = if self.board.piece_bitboards[B::piece(Piece::ROOK)].count() > 0 { 0 } else { 4 };
            end_dir_idx = if self.board.piece_bitboards[B::piece(Piece::BISHOP)].count() > 0 { 8 } else { 4 };
        }

        for dir in start_dir_idx..end_dir_idx {
            let is_diagonal = dir > 3;
            let slider = if is_diagonal { self.diagonal_sliders::<B, W>() } else { self.orthogonal_sliders::<B, W>() };
            if (Precomputed::dir_ray_mask(self.king_square::<W, B>(), dir) & slider).0 == 0 { continue; }

            let n = Precomputed::num_sqrs_to_edge(self.king_square::<W, B>(), dir);
            let dir_offset = Precomputed::direction_offsets(dir);
            let mut is_piece_along_ray = false;
            let mut is_friendly_piece = false;
            let mut ray_mask = BitBoard(0);

            for i in 0..n {
                let sqr = self.king_square::<W, B>() + dir_offset * (i + 1);
                ray_mask |= sqr.to_bitboard();
                let piece = self.board.square[sqr];

                if piece != Piece::NULL {
                    if piece.is_color(B::piece_color()) && ((is_diagonal && piece.is_bishop_or_queen()) || (!is_diagonal && piece.is_rook_or_queen())) {
                        if is_piece_along_ray {
                            if is_friendly_piece {
                                pin_rays |= ray_mask;
                            } else {
                                ray_mask &= !sqr.to_bitboard();
                                blocker_rays |= ray_mask;
                            }
                            break;
                        } else {
                            is_piece_along_ray = true;
                            is_friendly_piece = false;
                        }
                    } else if !is_piece_along_ray {
                        if piece.is_color(W::piece_color()) {
                            is_friendly_piece = true;
                        }
                        is_piece_along_ray = true
                    } else { break }
                }
            }
        };

        (pin_rays, blocker_rays)
    }

    /// Checks if a piece is pinned. 
    ///
    /// Requires: `pin_rays`
    pub fn pinned<W: Color, B: Color>(&self, sqr: Coord) -> bool {
        ((self.pin_rays[W::index()].0 >> sqr.index()) & 1).0 != 0
    }

    /// Returns `(all_attacks, double_attacks)`
    // TODO: Use to eliminate squares early on for `knight_attack`.
    // Same with all other all attacks functions.
    pub fn all_knight_attacks<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut knights = self.board.piece_bitboards[W::piece(Piece::KNIGHT)];
        knights &= !self.pin_rays[W::index()].0;
        let mut attacks = BitBoard(0);
        let mut doubled = BitBoard(0);

        while knights.0 != 0 {
            let sqr = Coord::from_idx(knights.pop_lsb() as i8);
            let moves = Precomputed::knight_moves(sqr);
            doubled |= attacks & moves;
            attacks |= moves;
        }

        (attacks, doubled)
    }

    /// Calculates the friendly knights attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    /// 
    /// Requires: `pin_rays`
    pub fn knight_attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let mut attacks = Precomputed::knight_moves(sqr) & self.board.piece_bitboards[W::piece(Piece::KNIGHT)];
        attacks &= !self.pin_rays[W::index()].0;
        attacks
    }

    pub fn knight_attack_from<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        if self.pinned::<W, B>(sqr) {
            BitBoard(0)
        } else {
            Precomputed::knight_moves(sqr)
        }
    }

    pub fn all_bishop_xray_attacks<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut bishops = self.board.piece_bitboards[W::piece(Piece::BISHOP)];
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = BitBoard(0);
        let mut doubled = BitBoard(0);
        
        while bishops.0 != 0 {
            let sqr = Coord::from_idx(bishops.pop_lsb() as i8);
            let moves = Magics::bishop_attacks(sqr, blockers);
            let valid = if self.pinned::<W, B>(sqr) {
                moves & Precomputed::align_mask(sqr, self.king_square::<W, B>())
                // moves & (self.pin_rays[W::index()].0 | self.king_square::<W, B>().to_bitboard())
            } else {
                moves
            };
            doubled |= attacks & valid;
            attacks |= valid;
        }
        (attacks, doubled)
    }

    /// Calculates the friendly bishops attacking `sqr`, including xray attacks through queens. 
    /// If s2 specified, only counts attacks coming from that square.  
    ///
    /// Requires: `pin_rays`
    pub fn bishop_xray_attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = Magics::bishop_attacks(sqr, blockers) 
            & self.board.piece_bitboards[W::piece(Piece::BISHOP)];

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned::<W, B>(start) && !Precomputed::align_mask(start, self.king_square::<W, B>()).contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    pub fn bishop_xray_attack_from<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = Magics::bishop_attacks(sqr, blockers);

        if self.pinned::<W, B>(sqr) {
            attacks &= Precomputed::align_mask(sqr, self.king_square::<W, B>());
        }

        attacks
    }

    pub fn all_rook_xray_attacks<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut rooks = self.board.piece_bitboards[W::piece(Piece::ROOK)];
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]
            | self.board.piece_bitboards[W::piece(Piece::ROOK)]);
        let mut attacks = BitBoard(0);
        let mut doubled = BitBoard(0);
        
        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            let moves = Magics::rook_attacks(sqr, blockers);
            let valid = if self.pinned::<W, B>(sqr) {
                moves & Precomputed::align_mask(sqr, self.king_square::<W, B>())
                // moves & (self.pin_rays[W::index()].0 | self.king_square::<W, B>().to_bitboard())
            } else {
                moves
            };
            doubled |= attacks & valid;
            attacks |= valid;
        }
        (attacks, doubled)
    }

    /// Calculates the friendly rooks attacking `sqr`, including xray attacks through queens. 
    /// If s2 specified, only counts attacks coming from that square.  
    ///
    /// Requires: `pin_rays`
    pub fn rook_xray_attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]
            | self.board.piece_bitboards[W::piece(Piece::ROOK)]);
        let mut attacks = Magics::rook_attacks(sqr, blockers) 
            & self.board.piece_bitboards[W::piece(Piece::ROOK)];

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned::<W, B>(start) && !Precomputed::align_mask(start, self.king_square::<W, B>()).contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    pub fn rook_xray_attack_from<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]
            | self.board.piece_bitboards[W::piece(Piece::ROOK)]);
        let mut attacks = Magics::rook_attacks(sqr, blockers);

        if self.pinned::<W, B>(sqr) {
            attacks &= Precomputed::align_mask(sqr, self.king_square::<W, B>());
        }

        attacks
    }

    pub fn all_queen_attacks<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut queens = self.board.piece_bitboards[W::piece(Piece::QUEEN)];
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = BitBoard(0);
        let mut doubled = BitBoard(0);
        
        while queens.0 != 0 {
            let sqr = Coord::from_idx(queens.pop_lsb() as i8);
            let moves = Magics::bishop_attacks(sqr, blockers) | Magics::rook_attacks(sqr, blockers);
            let valid = if self.pinned::<W, B>(sqr) {
                moves & Precomputed::align_mask(sqr, self.king_square::<W, B>())
                // moves & (self.pin_rays[W::index()].0 | self.king_square::<W, B>().to_bitboard())
            } else {
                moves
            };
            doubled |= attacks & valid;
            attacks |= valid;
        }
        (attacks, doubled)
    }

    /// Calculates the friendly queens attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    ///
    /// Requires: `pin_rays`
    pub fn queen_attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = (Magics::bishop_attacks(sqr, blockers) | Magics::rook_attacks(sqr, blockers))
            & self.board.piece_bitboards[W::piece(Piece::QUEEN)];

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned::<W, B>(start) && !Precomputed::align_mask(start, self.king_square::<W, B>()).contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    pub fn queen_attack_from<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = (Magics::bishop_attacks(sqr, blockers) | Magics::rook_attacks(sqr, blockers));

        if self.pinned::<W, B>(sqr) {
            attacks &= Precomputed::align_mask(sqr, self.king_square::<W, B>());
        }

        attacks
    }

    pub fn all_pawn_attacks<W: Color, B: Color>(&self) -> (BitBoard, BitBoard) {
        let mut pawns = self.board.piece_bitboards[W::piece(Piece::PAWN)];
        let mut attacks = BitBoard(0);
        let mut doubled = BitBoard(0);
        
        while pawns.0 != 0 {
            let sqr = Coord::from_idx(pawns.pop_lsb() as i8);
            let moves = if W::is_white() { Precomputed::white_pawn_attacks(sqr) } else { Precomputed::black_pawn_attacks(sqr) };
            doubled |= attacks & moves;
            attacks |= moves;
        }
        (attacks, doubled)
    }

    /// Calculates the friendly pawns attacking `sqr`, excluding pins and en-passant. 
    /// If s2 specified, only counts attacks coming from that square. 
    ///
    /// Requires: `pin_rays`
    pub fn pawn_attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        let map = if W::is_white() { Precomputed::black_pawn_attacks(sqr) } else { Precomputed::white_pawn_attacks(sqr) };
        map & self.board.piece_bitboards[W::piece(Piece::PAWN)]
    }

    /// Does not return the doubled attacks, as it is impossible.
    pub fn all_king_attacks<W: Color, B: Color>(&self) -> BitBoard {
        let mut kings = self.board.piece_bitboards[W::piece(Piece::KING)];
        let mut attacks = BitBoard(0);
        
        while kings.0 != 0 {
            let sqr = Coord::from_idx(kings.pop_lsb() as i8);
            attacks |= Precomputed::king_moves(sqr);
        }
        attacks
    }

    /// Calculates the friendly kings attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    pub fn king_attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        Precomputed::king_moves(sqr) & self.board.piece_bitboards[W::piece(Piece::KING)]
    }

    pub fn all_attacks<W: Color, B: Color>(&self) -> BitBoard {
        self.all_pawn_attacks[W::index()].0
            | self.all_king_attacks[W::index()]
            | self.all_knight_attacks[W::index()].0
            | self.all_bishop_attacks[W::index()].0
            | self.all_rook_attacks[W::index()].0
            | self.all_queen_attacks[W::index()].0
    }

    pub fn all_doubled_attacks<W: Color, B: Color>(&self) -> BitBoard {
        let pawns = self.all_pawn_attacks[W::index()];
        let knights = self.all_knight_attacks[W::index()];
        let bishops = self.all_bishop_attacks[W::index()];
        let rooks = self.all_rook_attacks[W::index()];
        let queens = self.all_queen_attacks[W::index()];
        let kings = self.all_king_attacks[W::index()];

        let mut doubled = pawns.1 | knights.1 | bishops.1 | rooks.1 | queens.1;

        doubled | ((pawns.0 & (knights.0 | bishops.0 | rooks.0 | queens.0 | kings))
            | (knights.0 & (pawns.0 | bishops.0 | rooks.0 | queens.0 | kings))
            | (bishops.0 & (pawns.0 | knights.0 | rooks.0 | queens.0 | kings))
            | (rooks.0 & (pawns.0 | knights.0 | bishops.0 | queens.0 | kings))
            | (queens.0 & (pawns.0 | knights.0 | bishops.0 | rooks.0 | kings)))
            | (kings & (pawns.0 | knights.0 | bishops.0 | rooks.0 | queens.0))
    }

    /// Calculates the friendly attacks on `sqr` by all pieces.
    ///
    /// Requires: `pin_rays`
    pub fn attack<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard {
        self.pawn_attack::<W, B>(sqr)
            | self.king_attack::<W, B>(sqr)
            | self.knight_attack::<W, B>(sqr)
            | self.bishop_xray_attack::<W, B>(sqr)
            | self.rook_xray_attack::<W, B>(sqr)
            | self.queen_attack::<W, B>(sqr)
    }

    /// Calculates the friendly queens attacking `sqr` diagonally. If s2 specified, only counts
    /// attacks coming from that square. 
    ///
    /// Requires: `pin_rays`
    pub fn queen_attack_diagonal<W: Color, B: Color>(&self, sqr: Coord) -> BitBoard{
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = Magics::bishop_attacks(sqr, blockers)
            & self.board.piece_bitboards[W::piece(Piece::QUEEN)];

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned::<W, B>(start) && !Precomputed::align_mask(start, self.king_square::<W, B>()).contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("nb2kb1R/pppppppp/6n1/4R2B/Qb2P3/4r3/PPPP1PPP/2BNK1Rq b KQkq e3 0 1")]
    fn test_knight_attack() {
        assert_eval!(+ knight_attack, 4, 8, eval);
    }

    #[test]
    #[evaluation_test("rnb1k1nr/pppp1ppp/8/4p2B/1qb1PPQ1/8/PPPB1PPP/RN2K1NR b KQkq - 1 2")]
    fn test_bishop_xray_attack() {
        assert_eval!(+ bishop_xray_attack, 10, 12, eval);
    }

    #[test]
    #[evaluation_test("2p1kbn1/pp1bpppr/r7/3p1q1B/Q7/P2R4/PPP1PPPP/1N2KBNR w KQkq d6 0 2")]
    fn test_rook_xray_attack() {
        assert_eval!(+ rook_xray_attack, 13, 15, eval);
    }

    #[test]
    #[evaluation_test("nb2kb1R/pppppppp/6n1/4R2B/1bPP1q2/Q3r3/PPP2PPP/2BNK1R1 b KQkq e3 0 1")]
    fn test_queen_attack() {
        assert_eval!(+ queen_attack, 11, 15, eval);
    }

    #[test]
    #[evaluation_test("nb2kb1R/p1p1n2p/1p3pn1/n3R2B/1bPP1qpP/QP2r1P1/P1P2P2/2BNK1R1 w KQkq - 0 2")]
    fn test_pawn_attack() {
        assert_eval!(+ pawn_attack, 14, 10, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_king_attack() {
        assert_eval!(+ king_attack, 3, 8, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_attack() {
        assert_eval!(+ attack, 51, 73, eval);
    }

    #[test]
    #[evaluation_test("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")]
    fn test_queen_attack_diagonal() {
        assert_eval!(+ queen_attack_diagonal, 3, 7, eval);
    }
}

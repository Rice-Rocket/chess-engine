use proc_macro_utils::flipped_eval;

use crate::{board::{coord::Coord, piece::Piece}, color::Color, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    #[flipped_eval]
    pub fn king_square(&self) -> Coord {
        self.board.king_square[self.color]
    }

    /// Requires: `king_square`
    // TODO: Save this data from move generation
    #[flipped_eval]
    pub fn pin_rays(&self) -> BitBoard {
        let mut pin_rays = BitBoard(0);
        let mut start_dir_idx = 0;
        let mut end_dir_idx = 8;
        let mut in_double_check = false;
        let mut in_check = false;

        // Don't calculate unecessary directions
        if self.board.piece_bitboards[self.color.flip().piece(Piece::QUEEN)].count() == 0 {
            start_dir_idx = if self.board.piece_bitboards[self.color.flip().piece(Piece::ROOK)].count() > 0 { 0 } else { 4 };
            end_dir_idx = if self.board.piece_bitboards[self.color.flip().piece(Piece::BISHOP)].count() > 0 { 8 } else { 4 };
        }

        for dir in start_dir_idx..end_dir_idx {
            let is_diagonal = dir > 3;
            let slider = if is_diagonal { self.board.enemy_diagonal_sliders } else { self.board.enemy_orthogonal_sliders };
            if (self.precomp.dir_ray_mask[self.friendly_king_square()][dir] & slider).0 == 0 { continue; }

            let n = self.precomp.num_sqrs_to_edge[self.friendly_king_square()][dir];
            let dir_offset = self.precomp.direction_offsets[dir];
            let mut is_friendly_piece_along_ray = false;
            let mut ray_mask = BitBoard(0);

            for i in 0..n {
                let sqr = self.friendly_king_square() + dir_offset * (i + 1);
                ray_mask |= sqr.to_bitboard();
                let piece = self.board.square[sqr];

                if piece != Piece::NULL {
                    if piece.is_color(self.color.piece_color()) {
                        if !is_friendly_piece_along_ray {
                            is_friendly_piece_along_ray = true
                        } else { break };
                    } else if (is_diagonal && piece.is_bishop_or_queen()) || (!is_diagonal && piece.is_rook_or_queen()) {
                        if is_friendly_piece_along_ray {
                            pin_rays |= ray_mask;
                        } else {
                            in_double_check = in_check;
                            in_check = true;
                        }
                        break;
                    } else { break; }
                }
            }
            if in_double_check { break; }
        };

        pin_rays
    }

    /// Checks if a piece is pinned. 
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn pinned(&self, sqr: Coord) -> bool {
        ((self.pin_rays[self.color] >> sqr.index()) & 1).0 != 0
    }

    // TODO: Cache this and use if to elimate squares early on for `knight_attack`.
    // Same with all other attack functions.
    #[flipped_eval]
    pub fn all_knight_attacks(&self) -> BitBoard {
        let mut knights = self.board.piece_bitboards[self.color.piece(Piece::KNIGHT)];
        knights &= !self.pin_rays[self.color];
        let mut attacks = BitBoard(0);

        while knights.0 != 0 {
            let sqr = Coord::from_idx(knights.pop_lsb() as i8);
            attacks |= self.precomp.knight_moves[sqr];
        }

        attacks
    }

    /// Calculates the friendly knights attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    /// 
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn knight_attack(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard {
        let mut attacks = self.precomp.knight_moves[sqr] & self.board.piece_bitboards[self.color.piece(Piece::KNIGHT)];
        attacks &= !self.pin_rays[self.color];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }
        attacks
    }

    #[flipped_eval]
    pub fn all_bishop_xray_attacks(&self) -> BitBoard {
        let mut bishops = self.board.piece_bitboards[self.color.piece(Piece::BISHOP)];
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = BitBoard(0);
        
        while bishops.0 != 0 {
            let sqr = Coord::from_idx(bishops.pop_lsb() as i8);
            let moves = self.magics.get_bishop_attacks(sqr, blockers);
            if self.friendly_pinned(sqr) {
                attacks |= moves & self.pin_rays[self.color];
            } else {
                attacks |= moves;
            }
        }
        attacks
    }

    /// Calculates the friendly bishops attacking `sqr`, including xray attacks through queens. 
    /// If s2 specified, only counts attacks coming from that square.  
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn bishop_xray_attack(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = self.magics.get_bishop_attacks(sqr, blockers) 
            & self.board.piece_bitboards[self.color.piece(Piece::BISHOP)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.friendly_pinned(start) && !self.precomp.align_mask[start][self.friendly_king_square()].contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    #[flipped_eval]
    pub fn all_rook_xray_attacks(&self) -> BitBoard {
        let mut rooks = self.board.piece_bitboards[self.color.piece(Piece::ROOK)];
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = BitBoard(0);
        
        while rooks.0 != 0 {
            let sqr = Coord::from_idx(rooks.pop_lsb() as i8);
            let moves = self.magics.get_rook_attacks(sqr, blockers);
            if self.friendly_pinned(sqr) {
                attacks |= moves & self.pin_rays[self.color];
            } else {
                attacks |= moves;
            }
        }
        attacks
    }

    /// Calculates the friendly rooks attacking `sqr`, including xray attacks through queens. 
    /// If s2 specified, only counts attacks coming from that square.  
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn rook_xray_attack(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = self.magics.get_rook_attacks(sqr, blockers) 
            & self.board.piece_bitboards[self.color.piece(Piece::ROOK)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.friendly_pinned(start) && !self.precomp.align_mask[start][self.friendly_king_square()].contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    #[flipped_eval]
    pub fn all_queen_xray_attacks(&self) -> BitBoard {
        let mut queens = self.board.piece_bitboards[self.color.piece(Piece::QUEEN)];
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = BitBoard(0);
        
        while queens.0 != 0 {
            let sqr = Coord::from_idx(queens.pop_lsb() as i8);
            let moves = self.magics.get_bishop_attacks(sqr, blockers) | self.magics.get_rook_attacks(sqr, blockers);
            if self.friendly_pinned(sqr) {
                attacks |= moves & self.pin_rays[self.color];
            } else {
                attacks |= moves;
            }
        }
        attacks
    }

    /// Calculates the friendly queens attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn queen_attack(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard {
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = (self.magics.get_bishop_attacks(sqr, blockers) | self.magics.get_rook_attacks(sqr, blockers))
            & self.board.piece_bitboards[self.color.piece(Piece::QUEEN)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.friendly_pinned(start) && !self.precomp.align_mask[start][self.friendly_king_square()].contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    /// Calculates the friendly pawns attacking `sqr`, excluding pins and en-passant. 
    /// If s2 specified, only counts attacks coming from that square. 
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn pawn_attack(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard {
        let map = if self.color == Color::White { self.precomp.black_pawn_attacks[sqr] } else { self.precomp.white_pawn_attacks[sqr] };
        let mut attacks = map & self.board.piece_bitboards[self.color.piece(Piece::PAWN)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.friendly_pinned(start) && !self.precomp.align_mask[start][self.friendly_king_square()].contains_square(sqr.square()) {
                res.clear_square(start.square());
            };
        }
        res
    }

    /// Calculates the friendly kings attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    #[flipped_eval]
    pub fn king_attack(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard {
        let mut attacks = self.precomp.king_moves[sqr] & self.board.piece_bitboards[self.color.piece(Piece::KING)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }
        attacks
    }

    /// Calculates the friendly attacks on `sqr` by all pieces.
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn attack(&self, sqr: Coord) -> BitBoard {
        self.friendly_pawn_attack(None, sqr)
            | self.friendly_king_attack(None, sqr)
            | self.friendly_knight_attack(None, sqr)
            | self.friendly_bishop_xray_attack(None, sqr)
            | self.friendly_rook_xray_attack(None, sqr)
            | self.friendly_queen_attack(None, sqr)
    }

    /// Calculates the friendly queens attacking `sqr` diagonally. If s2 specified, only counts
    /// attacks coming from that square. 
    ///
    /// Requires: `pin_rays`
    #[flipped_eval]
    pub fn queen_attack_diagonal(&self, s2: Option<Coord>, sqr: Coord) -> BitBoard{
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = self.magics.get_bishop_attacks(sqr, blockers)
            & self.board.piece_bitboards[self.color.piece(Piece::QUEEN)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut res = attacks;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.friendly_pinned(start) && !self.precomp.align_mask[start][self.friendly_king_square()].contains_square(sqr.square()) {
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
    fn test_knight_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/Qb2P3/4r3/PPPP1PPP/2BNK1Rq b KQkq e3 0 1")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_knight_attack, 4, 8, eval; None);
    }

    #[test]
    fn test_bishop_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("rnb1k1nr/pppp1ppp/8/4p2B/1qb1PPQ1/8/PPPB1PPP/RN2K1NR b KQkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ enemy_bishop_xray_attack, 12, 10, eval; None);
    }

    #[test]
    fn test_rook_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("2p1kbn1/pp1bpppr/r7/3p1q1B/Q7/P2R4/PPP1PPPP/1N2KBNR w KQkq d6 0 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_rook_xray_attack, 13, 15, eval; None);
    }

    #[test]
    fn test_queen_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/1bPP1q2/Q3r3/PPP2PPP/2BNK1R1 b KQkq e3 0 1")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_queen_attack, 11, 15, eval; None);
    }

    #[test]
    fn test_pawn_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/p1p1n2p/1p3pn1/n3R2B/1bPP1qpP/QP2r1P1/P1P2P2/2BNK1R1 w KQkq - 0 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_pawn_attack, 14, 10, eval; None);
    }

    #[test]
    fn test_king_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_king_attack, 3, 8, eval; None);
    }

    #[test]
    fn test_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_attack, 51, 73, eval);
    }

    #[test]
    fn test_queen_attack_diagonal() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(+ friendly_queen_attack_diagonal, 3, 7, eval; None);
    }
}

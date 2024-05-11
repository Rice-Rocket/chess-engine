use crate::{board::{coord::Coord, piece::Piece}, color::Color, prelude::BitBoard};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn king_square(&self) -> Coord {
        self.board.king_square[self.color]
    }

    /// Requires: `king_square`
    // TODO: Save this data from move generation
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
            if (self.precomp.dir_ray_mask[self.king_square][dir] & slider).0 == 0 { continue; }

            let n = self.precomp.num_sqrs_to_edge[self.king_square][dir];
            let dir_offset = self.precomp.direction_offsets[dir];
            let mut is_friendly_piece_along_ray = false;
            let mut ray_mask = BitBoard(0);

            for i in 0..n {
                let sqr = self.king_square + dir_offset * (i + 1);
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
    /// Requires: `pin_rays`, `king_square`
    pub fn pinned(&self, sqr: Coord) -> bool {
        ((self.pin_rays >> sqr.index()) & 1).0 != 0
    }

    /// Calculates the number of friendly knights attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    /// 
    /// Requires: `pin_rays`, `king_square`
    pub fn knight_attack(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let mut attacks = self.precomp.knight_moves[sqr] & self.board.piece_bitboards[self.color.piece(Piece::KNIGHT)];
        attacks &= !self.pin_rays;
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }
        attacks.count() as i32
    }

    /// Calculates the number of friendly bishops attacking `sqr`, including xray attacks through queens. 
    /// If s2 specified, only counts attacks coming from that square.  
    ///
    /// Requires: `pin_rays`, `king_square`
    pub fn bishop_xray_attack(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = self.magics.get_bishop_attacks(sqr, blockers) 
            & self.board.piece_bitboards[self.color.piece(Piece::BISHOP)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut count = 0;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned(start) {
                if self.precomp.align_mask[start][self.king_square].contains_square(sqr.square()) {
                    count += 1;
                }
            } else {
                count += 1;
            };
        }

        count
    }

    /// Calculates the number of friendly rooks attacking `sqr`, including xray attacks through queens. 
    /// If s2 specified, only counts attacks coming from that square.  
    ///
    /// Requires: `pin_rays`, `king_square`
    pub fn rook_xray_attack(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let blockers = self.board.all_pieces_bitboard & !(
            self.board.piece_bitboards[Piece::new(Piece::WHITE_QUEEN)] 
            | self.board.piece_bitboards[Piece::new(Piece::BLACK_QUEEN)]);
        let mut attacks = self.magics.get_rook_attacks(sqr, blockers) 
            & self.board.piece_bitboards[self.color.piece(Piece::ROOK)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut count = 0;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned(start) {
                if self.precomp.align_mask[start][self.king_square].contains_square(sqr.square()) {
                    count += 1;
                }
            } else {
                count += 1;
            };
        }
        count
    }

    /// Calculates the number of friendly queens attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    ///
    /// Requires: `pin_rays`, `king_square`
    pub fn queen_attack(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = (self.magics.get_bishop_attacks(sqr, blockers) | self.magics.get_rook_attacks(sqr, blockers))
            & self.board.piece_bitboards[self.color.piece(Piece::QUEEN)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut count = 0;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned(start) {
                if self.precomp.align_mask[start][self.king_square].contains_square(sqr.square()) {
                    count += 1;
                }
            } else {
                count += 1;
            };
        }
        count
    }

    /// Calculates the number of friendly pawns attacking `sqr`, excluding pins and en-passant. 
    /// If s2 specified, only counts attacks coming from that square. 
    ///
    /// Requires: `pin_rays`, `king_square`
    pub fn pawn_attack(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let map = if self.color == Color::White { self.precomp.black_pawn_attacks[sqr] } else { self.precomp.white_pawn_attacks[sqr] };
        let mut attacks = map & self.board.piece_bitboards[self.color.piece(Piece::PAWN)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut count = 0;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned(start) {
                if self.precomp.align_mask[start][self.king_square].contains_square(sqr.square()) {
                    count += 1;
                }
            } else {
                count += 1;
            };
        }
        count
    }

    /// Calculates the number of friendly kings attacking `sqr`. If s2 specified, only counts attacks coming
    /// from that square. 
    pub fn king_attack(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let mut attacks = self.precomp.king_moves[sqr] & self.board.piece_bitboards[self.color.piece(Piece::KING)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }
        attacks.count() as i32
    }

    /// Calculates the number of friendly attacks on `sqr` by all pieces.
    ///
    /// Requires: `pin_rays`, `king_square`
    pub fn attack(&self, sqr: Coord) -> i32 {
        self.pawn_attack(None, sqr)
            + self.king_attack(None, sqr)
            + self.knight_attack(None, sqr)
            + self.bishop_xray_attack(None, sqr)
            + self.rook_xray_attack(None, sqr)
            + self.queen_attack(None, sqr)
    }

    /// Calculates the number of friendly queens attacking `sqr` diagonally. If s2 specified, only counts
    /// attacks coming from that square. 
    ///
    /// Requires: `pin_rays`, `king_square`
    pub fn queen_attack_diagonal(&self, s2: Option<Coord>, sqr: Coord) -> i32 {
        let blockers = self.board.all_pieces_bitboard;
        let mut attacks = self.magics.get_bishop_attacks(sqr, blockers)
            & self.board.piece_bitboards[self.color.piece(Piece::QUEEN)];
        if let Some(s) = s2 {
            attacks &= s.to_bitboard();
        }

        let mut count = 0;
        while attacks.0 != 0 {
            let start = Coord::from_idx(attacks.pop_lsb() as i8);
            if self.pinned(start) {
                if self.precomp.align_mask[start][self.king_square].contains_square(sqr.square()) {
                    count += 1;
                }
            } else {
                count += 1;
            };
        }
        count
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

        assert_eval!(knight_attack, 4, 8, eval; None);
    }

    #[test]
    fn test_bishop_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("rnb1k1nr/pppp1ppp/8/4p2B/1qb1PPQ1/8/PPPB1PPP/RN2K1NR b KQkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(bishop_xray_attack, 10, 12, eval; None);
    }

    #[test]
    fn test_rook_xray_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("2p1kbn1/pp1bpppr/r7/3p1q1B/Q7/P2R4/PPP1PPPP/1N2KBNR w KQkq d6 0 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(rook_xray_attack, 13, 15, eval; None);
    }

    #[test]
    fn test_queen_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/pppppppp/6n1/4R2B/1bPP1q2/Q3r3/PPP2PPP/2BNK1R1 b KQkq e3 0 1")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(queen_attack, 11, 15, eval; None);
    }

    #[test]
    fn test_pawn_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb2kb1R/p1p1n2p/1p3pn1/n3R2B/1bPP1qpP/QP2r1P1/P1P2P2/2BNK1R1 w KQkq - 0 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(pawn_attack, 14, 10, eval; None);
    }

    #[test]
    fn test_king_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(king_attack, 3, 8, eval; None);
    }

    #[test]
    fn test_attack() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(attack, 51, 73, eval);
    }

    #[test]
    fn test_queen_attack_diagonal() {
        let precomp = PrecomputedData::new();
        let magics = MagicBitBoards::default();
        let board = Board::load_position(Some(String::from("nb3b1R/p1pkn2p/1p2Rpn1/n6B/1bPP1qpP/QP2r1P1/P1P2P2/2BN2RK b Qkq - 1 2")), &mut Zobrist::new());
        let mut eval = Evaluation::new(&board, &precomp, &magics, Color::White);

        assert_eval!(queen_attack_diagonal, 3, 7, eval; None);
    }
}

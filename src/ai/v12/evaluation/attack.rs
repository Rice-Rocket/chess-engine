use crate::board::{coord::Coord, piece::Piece};
use crate::move_gen::bitboard::bb::BitBoard;
use super::perspective::Perspective;
use super::pos::{sum_sqrs, PositionEvaluation};

pub fn pinned_direction(pos: &PositionEvaluation, per: Perspective) -> i32 {
    sum_sqrs(pinned_direction_sqr, pos, per)
}

pub fn pinned_direction_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> i32 {
    let piece = pos.square(sqr);
    if piece == Piece::NULL { return 0; }

    let mut color = 1;
    if !per.is_color(piece.color()) { color = -1; }
    for i in 0..8 {
        let dir = pos.move_data.dir_offsets_2d[i];
        let mut king = false;
        for d in 1..8 {
            let piece = pos.square(Coord::new(sqr.file() + d * dir.file(), sqr.rank() + d * dir.rank()));
            if piece == per.friendly_piece(Piece::KING) { king = true; }
            if piece != Piece::NULL { break; }
        };

        if king {
            for d in 1..8 {
                let piece = pos.square(Coord::new(sqr.file() - d * dir.file(), sqr.rank() - d * dir.rank()));
                if piece == per.enemy_piece(Piece::QUEEN) 
                || (piece == per.enemy_piece(Piece::BISHOP) && dir.file() * dir.rank() != 0)
                || (piece == per.enemy_piece(Piece::ROOK) && dir.file() * dir.rank() == 0) {
                    return (dir.file() + dir.rank() * 3).abs() as i32 * color;
                }
                if piece != Piece::NULL { break; }
            }
        }
    };
    return 0;
}


pub fn pinned_sqr(pos: &PositionEvaluation, per: Option<Perspective>, sqr: Coord) -> i32 {
    if let Some(perspective) = per {
        return if pos.friendly_pin_rays(perspective).contains_square(sqr.square()) { 1 } else { 0 };
    }
    return if pos.all_pin_rays().contains_square(sqr.square()) { 1 } else { 0 };
}


/// Counts the number of friendly knights attacking a square. 
/// If `sqr2` is `Some`, the function will determine if the given square is attacked by
/// a knight on that square. 
pub fn knight_attacks_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    // Possible attackers are knights on knight accessible squares (that is they can be reached by a knight jump)
    let mut attackers = pos.move_data.knight_attack_bitboards[sqr.index()] & pos.friendly_piece_bb(per, Piece::KNIGHT);
    if let Some(s2) = sqr2 {
        // Only allow knights on square 2 if applicable
        attackers &= s2.to_bitboard();
    }
    let mut v = 0;

    // Loop through knights with line of sight
    while attackers.0 != 0 {
        let sq = attackers.pop_lsb();
        let coord = Coord::from_idx(sq as i8);
        // If the attacking knight is not pinned, include it
        if pinned_sqr(pos, Some(per), coord) == 0 {
            v += 1;
        }
    }
    v
}

/// Counts the number of friendly bishops attacking a square. 
/// If `sqr2` is `Some`, the function will determine if the given square is attacked by
/// a bishop on that square. 
pub fn bishop_xray_attacks_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    // Allow bishops to see through queens
    let blockers = pos.all_pieces_bb() & !pos.piece_bb(Piece::QUEEN);
    // Possible attackers are bishops that can be seen on diagonals from the given square
    let mut attackers = pos.magic.get_bishop_attacks(sqr, blockers) & pos.friendly_piece_bb(per, Piece::BISHOP);
    if let Some(s2) = sqr2 {
        // Only allow bishops on square 2 if applicable
        attackers &= s2.to_bitboard();
    }
    let mut v = 0;

    // Loop through bishops with line of sight
    while attackers.0 != 0 {
        let sq = attackers.pop_lsb();
        let coord = Coord::from_idx(sq as i8);
        // If it's not pinned, include the square
        if pinned_sqr(pos, Some(per), coord) == 0 {
            v += 1;
        // If it is pinned, only include the square if it is in the direction of the pin
        } else if (pos.move_data.align_mask[coord.index()][pos.friendly_king_sqr(per).index()] & sqr.to_bitboard()).0 > 0 {
            v += 1;
        }
    }
    v
}


/// Counts the number of friendly rooks attacking a square. 
/// If `sqr2` is `Some`, the function will determine if the given square is attacked by
/// a rook on that square. 
pub fn rook_xray_attacks_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    // Allow rooks to see through queens and friendly rooks
    let blockers = pos.all_pieces_bb() & !pos.piece_bb(Piece::QUEEN) & !pos.friendly_piece_bb(per, Piece::ROOK);
    // Possible attackers are rooks that can be seen on orthogonals from the given square
    let mut attackers = pos.magic.get_rook_attacks(sqr, blockers) & pos.friendly_piece_bb(per, Piece::ROOK);
    if let Some(s2) = sqr2 {
        // Only allow rooks on square 2 if applicable
        attackers &= s2.to_bitboard();
    }
    let mut v = 0;

    // Loop through rooks with line of sight
    while attackers.0 != 0 {
        let sq = attackers.pop_lsb();
        let coord = Coord::from_idx(sq as i8);
        // If it's not pinned, include the square
        if pinned_sqr(pos, Some(per), coord) == 0 {
            v += 1;
        // If it is pinned, only include the square if it is in the direction of the pin
        } else if (pos.move_data.align_mask[coord.index()][pos.friendly_king_sqr(per).index()] & sqr.to_bitboard()).0 > 0 {
            v += 1;
        }
    }
    v
}


/// Counts the number of friendly queens attacking a square. 
/// If `sqr2` is `Some`, the function will determine if the given square is attacked by
/// a queen on that square. 
pub fn queen_attacks_sqr(pos: &PositionEvaluation, per: Perspective, sqr: Coord, sqr2: Option<Coord>) -> i32 {
    // Queen line of sight is blocked by everything
    let blockers = pos.all_pieces_bb();
    // Possible attackers are queens that can be seen on diagonals or orthogonals from the given square
    let mut attackers = (pos.magic.get_bishop_attacks(sqr, blockers) | pos.magic.get_rook_attacks(sqr, blockers)) & pos.friendly_piece_bb(per, Piece::QUEEN);
    if let Some(s2) = sqr2 {
        // Only allow queens on square 2 if applicable
        attackers &= s2.to_bitboard();
    }
    let mut v = 0;

    // Loop through queens with line of sight
    while attackers.0 != 0 {
        let sq = attackers.pop_lsb();
        let coord = Coord::from_idx(sq as i8);
        // If it's not pinned, include the square
        if pinned_sqr(pos, Some(per), coord) == 0 {
            v += 1;
        // If it is pinned, only include the square if it is in the direction of the pin
        } else if (pos.move_data.align_mask[coord.index()][pos.friendly_king_sqr(per).index()] & sqr.to_bitboard()).0 > 0 {
            v += 1;
        }
    }
    v
}




pub struct AttackEvaluationData {
    pub pinned_pieces: BitBoard,
    pub knight_attacks: [BitBoard; 64],
    pub bishop_attacks: [BitBoard; 64],
    pub rook_attacks: [BitBoard; 64],
    pub queen_attacks: [BitBoard; 64],
}

impl AttackEvaluationData {
    pub fn new(pos: &PositionEvaluation) -> Self {
        let mut eval_data = Self {
            pinned_pieces: BitBoard(0),
            knight_attacks: [BitBoard(0); 64],
            bishop_attacks: [BitBoard(0); 64],
            rook_attacks: [BitBoard(0); 64],
            queen_attacks: [BitBoard(0); 64],
        };
        eval_data.initialize(pos);
        eval_data
    }

    pub fn initialize(&mut self, pos: &PositionEvaluation) {
        self.pinned_pieces = (pos.friendly_pin_rays(Perspective::White) | pos.enemy_pin_rays(Perspective::Black)) & pos.all_pieces_bb();

        let mut w_knights = pos.friendly_piece_bb(Perspective::White, Piece::KNIGHT);
        let mut w_bishops = pos.friendly_piece_bb(Perspective::White, Piece::BISHOP);
        let mut w_rooks = pos.friendly_piece_bb(Perspective::White, Piece::ROOK);
        let mut w_queens = pos.friendly_piece_bb(Perspective::White, Piece::QUEEN);

        let mut b_knights = pos.friendly_piece_bb(Perspective::Black, Piece::KNIGHT);
        let mut b_bishops = pos.friendly_piece_bb(Perspective::Black, Piece::BISHOP);
        let mut b_rooks = pos.friendly_piece_bb(Perspective::Black, Piece::ROOK);
        let mut b_queens = pos.friendly_piece_bb(Perspective::Black, Piece::QUEEN);

        while w_knights.0 != 0 {
            let sq_idx = w_knights.pop_lsb() as i8;
            self.knight_attacks[sq_idx as usize] = self.get_knight_attacks(pos, Perspective::White, Coord::from_idx(sq_idx));
        }
        while w_bishops.0 != 0 {
            let sq_idx = w_bishops.pop_lsb() as i8;
            self.bishop_attacks[sq_idx as usize] = self.get_bishop_xray_attacks(pos, Perspective::White, Coord::from_idx(sq_idx));
        }
        while w_rooks.0 != 0 {
            let sq_idx = w_rooks.pop_lsb() as i8;
            self.rook_attacks[sq_idx as usize] = self.get_rook_xray_attacks(pos, Perspective::White, Coord::from_idx(sq_idx));
        }
        while w_queens.0 != 0 {
            let sq_idx = w_queens.pop_lsb() as i8;
            self.queen_attacks[sq_idx as usize] = self.get_queen_attacks(pos, Perspective::White, Coord::from_idx(sq_idx));
        }

        while b_knights.0 != 0 {
            let sq_idx = b_knights.pop_lsb() as i8;
            self.knight_attacks[sq_idx as usize] = self.get_knight_attacks(pos, Perspective::Black, Coord::from_idx(sq_idx));
        }
        while b_bishops.0 != 0 {
            let sq_idx = b_bishops.pop_lsb() as i8;
            self.bishop_attacks[sq_idx as usize] = self.get_bishop_xray_attacks(pos, Perspective::Black, Coord::from_idx(sq_idx));
        }
        while b_rooks.0 != 0 {
            let sq_idx = b_rooks.pop_lsb() as i8;
            self.rook_attacks[sq_idx as usize] = self.get_rook_xray_attacks(pos, Perspective::Black, Coord::from_idx(sq_idx));
        }
        while b_queens.0 != 0 {
            let sq_idx = b_queens.pop_lsb() as i8;
            self.queen_attacks[sq_idx as usize] = self.get_queen_attacks(pos, Perspective::Black, Coord::from_idx(sq_idx));
        }
    }

    fn is_pinned(&self, pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> bool {
        pos.friendly_pin_rays(per).contains_square(sqr.square())
    }


    /// Returns a bitboard of all the squares a knight at the given square can attack
    fn get_knight_attacks(&self, pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> BitBoard {
        // If the knight is pinned, it can't move
        if self.is_pinned(pos, per, sqr) { return BitBoard(0); }
        // Possible targets are knight accessible squares
        let targets = pos.move_data.knight_attack_bitboards[sqr.index()];
        targets
    }

    /// Returns a bitboard of all the squares a bishop at the given square can attack
    fn get_bishop_xray_attacks(&self, pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> BitBoard {
        // Allow bishops to see through queens
        let blockers = pos.all_pieces_bb() & !pos.piece_bb(Piece::QUEEN);
        let mut targets = pos.magic.get_bishop_attacks(sqr, blockers);

        // If the piece is pinned
        if self.is_pinned(pos, per, sqr) {
            // Mask the possible target squares with the pin direction
            targets &= pos.move_data.align_mask[sqr.index()][pos.friendly_king_sqr(per).index()];
        }

        targets
    }


    /// Returns a bitboard of all the squares a rook at the given square can attack
    fn get_rook_xray_attacks(&self, pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> BitBoard {
        // Allow rooks to see through queens and friendly rooks
        let blockers = pos.all_pieces_bb() & !pos.piece_bb(Piece::QUEEN) & !pos.friendly_piece_bb(per, Piece::ROOK);
        let mut targets = pos.magic.get_rook_attacks(sqr, blockers);

        // If the piece is pinned
        if self.is_pinned(pos, per, sqr) {
            // Mask the possible target squares with the pin direction
            targets &= pos.move_data.align_mask[sqr.index()][pos.friendly_king_sqr(per).index()];
        }

        targets
    }


    /// Returns a bitboard of all the squares a queen at the given square can attack
    fn get_queen_attacks(&self, pos: &PositionEvaluation, per: Perspective, sqr: Coord) -> BitBoard {
        // Queens are blocked by all pieces
        let blockers = pos.all_pieces_bb();
        let mut targets = pos.magic.get_bishop_attacks(sqr, blockers) | pos.magic.get_rook_attacks(sqr, blockers);

        // If the piece is pinned
        if self.is_pinned(pos, per, sqr) {
            // Mask the possible target squares with the pin direction
            targets &= pos.move_data.align_mask[sqr.index()][pos.friendly_king_sqr(per).index()];
        }

        targets
    }
}
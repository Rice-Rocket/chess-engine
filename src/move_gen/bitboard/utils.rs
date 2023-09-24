use bevy::prelude::*;

use crate::board::coord::Coord;

use super::bb::BitBoard;


#[derive(Resource)]
pub struct BitBoardUtils {
    pub knight_attacks: [BitBoard; 64],
    pub king_moves: [BitBoard; 64],
    pub white_pawn_attacks: [BitBoard; 64],
    pub black_pawn_attacks: [BitBoard; 64],
}

impl BitBoardUtils {
    const FILE_A: u64 = 0x101010101010101;

    pub const RANK_1: u64 = 0b11111111;
    pub const RANK_2: u64 = Self::RANK_1 << 8;
    pub const RANK_3: u64 = Self::RANK_2 << 8;
    pub const RANK_4: u64 = Self::RANK_3 << 8;
    pub const RANK_5: u64 = Self::RANK_4 << 8;
    pub const RANK_6: u64 = Self::RANK_5 << 8;
    pub const RANK_7: u64 = Self::RANK_6 << 8;
    pub const RANK_8: u64 = Self::RANK_7 << 8;

    pub const NOT_FILE_A: u64 = !Self::FILE_A;
    pub const NOT_FILE_H: u64 = !(Self::FILE_A << 7);

    
    pub fn pawn_attacks(b: BitBoard, is_white: bool) -> BitBoard {
        if is_white {
            return ((b << 9) & !BitBoard::FILE_A) | ((b << 7) & Self::NOT_FILE_H);
        }
        return ((b >> 7) & Self::NOT_FILE_A) | ((b >> 9) & Self::NOT_FILE_H);
    }
}


impl Default for BitBoardUtils {
    fn default() -> Self {
        let mut knight_attacks: [BitBoard; 64] = [BitBoard(0); 64];
        let mut king_moves: [BitBoard; 64] = [BitBoard(0); 64];
        let mut white_pawn_attacks: [BitBoard; 64] = [BitBoard(0); 64];
        let mut black_pawn_attacks: [BitBoard; 64] = [BitBoard(0); 64];

        let ortho_dir: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let diag_dir: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
        let knight_jumps: [(i8, i8); 8] = [(-2, -1), (-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2)];

        for y in 0..8 {
            for x in 0..8 {
                let sqr_idx = y * 8 + x;
                let sqr = Coord::from_idx(sqr_idx);
                for dir_idx in 0..4 {
                    let ortho_x = x + ortho_dir[dir_idx].0;
                    let ortho_y = y + ortho_dir[dir_idx].1;
                    let diag_x = x + diag_dir[dir_idx].0;
                    let diag_y = y + diag_dir[dir_idx].1;

                    if let Some(ortho_target_idx) = valid_index(ortho_x, ortho_y) {
                        king_moves[sqr.index()] |= 1 << ortho_target_idx;
                    }
                    if let Some(diag_target_idx) = valid_index(diag_x, diag_y) {
                        king_moves[sqr.index()] |= 1 << diag_target_idx;
                    }

                    for i in 0..knight_jumps.len() {
                        let knight_x = x + knight_jumps[i].0;
                        let knight_y = y + knight_jumps[i].1;
                        if let Some(knight_target_idx) = valid_index(knight_x, knight_y) {
                            knight_attacks[sqr.index()] |= 1 << knight_target_idx;
                        }
                    }

                    if let Some(white_pawn_right) = valid_index(x + 1, y + 1) {
                        white_pawn_attacks[sqr.index()] |= 1 << white_pawn_right;
                    }
                    if let Some(white_pawn_left) = valid_index(x - 1, y + 1) {
                        white_pawn_attacks[sqr.index()] |= 1 << white_pawn_left;
                    }
                    if let Some(black_pawn_right) = valid_index(x + 1, y - 1) {
                        black_pawn_attacks[sqr.index()] |= 1 << black_pawn_right;
                    }
                    if let Some(black_pawn_left) = valid_index(x - 1, y - 1) {
                        black_pawn_attacks[sqr.index()] |= 1 << black_pawn_left;
                    }
                }
            };
        };

        fn valid_index(x: i8, y: i8) -> Option<i8> {
            return match x >= 0 && x < 8 && y >= 0 && y < 8 {
                true => Some(y * 8 + x),
                false => None
            };
        }

        BitBoardUtils {
            knight_attacks,
            king_moves,
            white_pawn_attacks,
            black_pawn_attacks,
        }
    }
}

pub fn spawn_bitboard_utils(
    mut commands: Commands,
) {
    commands.insert_resource(BitBoardUtils::default());
}
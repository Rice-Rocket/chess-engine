use crate::board::coord::Coord;

use super::{bbutils::BitBoardUtils, bb::BitBoard};


pub struct PrecomputedBits {
    pub white_passed_pawn_mask: [BitBoard; 64],
    pub black_passed_pawn_mask: [BitBoard; 64],
    pub white_pawn_support_mask: [BitBoard; 64],
    pub black_pawn_support_mask: [BitBoard; 64],
    pub file_mask: [BitBoard; 8],
    pub adjacent_file_mask: [BitBoard; 8],
    pub king_safety_mask: [BitBoard; 64],
    pub white_forward_file_mask: [BitBoard; 64],
    pub black_forward_file_mask: [BitBoard; 64],
    pub triple_file_mask: [BitBoard; 8],
}

impl PrecomputedBits {
    pub const FILE_A: u64 = 0x101010101010101;

    pub const WHITE_KINGSIDE_MASK: BitBoard = BitBoard(1u64 << Coord::F1.const_idx() | 1u64 << Coord::G1.const_idx());
    pub const BLACK_KINGSIDE_MASK: BitBoard = BitBoard(1u64 << Coord::F8.const_idx() | 1u64 << Coord::G8.const_idx());
    pub const WHITE_QUEENSIDE_MASK_2: BitBoard = BitBoard(1u64 << Coord::D1.const_idx() | 1u64 << Coord::C1.const_idx());
    pub const BLACK_QUEENSIDE_MASK_2: BitBoard = BitBoard(1u64 << Coord::D8.const_idx() | 1u64 << Coord::C8.const_idx());
    pub const WHITE_QUEENSIDE_MASK: BitBoard = BitBoard(Self::WHITE_QUEENSIDE_MASK_2.0 | 1u64 << Coord::B1.const_idx());
    pub const BLACK_QUEENSIDE_MASK: BitBoard = BitBoard(Self::BLACK_QUEENSIDE_MASK_2.0 | 1u64 << Coord::B8.const_idx());

    pub fn new(bbutils: &BitBoardUtils) -> Self {
        let mut file_mask: [BitBoard; 8] = [BitBoard(0); 8];
        let mut adjacent_file_mask: [BitBoard; 8] = [BitBoard(0); 8];

        for i in 0..8 {
            file_mask[i] = BitBoard::FILE_A << i;
            let left = if i > 0 { BitBoard::FILE_A << (i - 1) } else { BitBoard(0) };
            let right = if i < 7 { BitBoard::FILE_A << (i + 1) } else { BitBoard(0) };
            adjacent_file_mask[i] = left | right;
        };

        let mut triple_file_mask: [BitBoard; 8] = [BitBoard(0); 8];
        for (i, mask) in triple_file_mask.iter_mut().enumerate() {
            let clamped_file = i.clamp(1, 6);
            *mask = file_mask[clamped_file] | adjacent_file_mask[clamped_file];
        }

        let mut white_passed_pawn_mask: [BitBoard; 64] = [BitBoard(0); 64];
        let mut black_passed_pawn_mask: [BitBoard; 64] = [BitBoard(0); 64];
        let mut white_pawn_support_mask: [BitBoard; 64] = [BitBoard(0); 64];
        let mut black_pawn_support_mask: [BitBoard; 64] = [BitBoard(0); 64];
        let mut white_forward_file_mask: [BitBoard; 64] = [BitBoard(0); 64];
        let mut black_forward_file_mask: [BitBoard; 64] = [BitBoard(0); 64];

        for sqr_idx in 0..64 {
            let sqr = Coord::from_idx(sqr_idx);
            let file = sqr.file();
            let rank = sqr.rank();
            let adjacent_files = PrecomputedBits::FILE_A << (file - 1).max(0) | PrecomputedBits::FILE_A << (file + 1).max(7);

            let white_forward_mask = BitBoard(!(u64::MAX >> (64 - 8 * (rank + 1))));
            let black_forward_mask = BitBoard((1 << (8 * rank)) - 1);

            white_passed_pawn_mask[sqr.index()] = (BitBoard::FILE_A << file as usize | adjacent_files) & white_forward_mask;
            black_passed_pawn_mask[sqr.index()] = (BitBoard::FILE_A << file as usize | adjacent_files) & black_forward_mask;

            let adjacent = ((if sqr_idx == 0 { BitBoard(0) } else { BitBoard(1 << (sqr_idx - 1)) }) | (if sqr_idx == 63 { 0 } else { 1 << (sqr_idx + 1) })) & adjacent_files;
            white_pawn_support_mask[sqr.index()] = adjacent | adjacent.shifted(-8);
            black_pawn_support_mask[sqr.index()] = adjacent | adjacent.shifted(8);

            white_forward_file_mask[sqr.index()] = white_forward_mask & file_mask[file as usize];
            black_forward_file_mask[sqr.index()] = black_forward_mask & file_mask[rank as usize];
        };

        let mut king_safety_mask: [BitBoard; 64] = [BitBoard(0); 64];
        for (i, mask) in king_safety_mask.iter_mut().enumerate() {
            *mask = bbutils.king_moves[i] | (1 << i);
        }

        PrecomputedBits {
            white_passed_pawn_mask,
            black_passed_pawn_mask,
            white_pawn_support_mask,
            black_pawn_support_mask,
            file_mask,
            adjacent_file_mask,
            king_safety_mask,
            white_forward_file_mask,
            black_forward_file_mask,
            triple_file_mask,
        }
    }
}

use crate::board::coord::Coord;

use super::masks::*;
use std::ops::*;

const DE_BRUIJN_64: u128 = 0x37E84A99DAE458F;
const DE_BRUIJN_TABLE: [u32; 64] = [
    0, 1, 17, 2, 18, 50, 3, 57,
    47, 19, 22, 51, 29, 4, 33, 58,
    15, 48, 20, 27, 25, 23, 52, 41,
    54, 30, 38, 5, 43, 34, 59, 8,
    63, 16, 49, 56, 46, 21, 28, 32,
    14, 26, 24, 40, 53, 37, 42, 7,
    62, 55, 45, 31, 13, 39, 36, 6,
    61, 44, 12, 35, 60, 11, 10, 9
];


#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);


impl_bit_ops!(BitBoard, u64);


impl BitBoard {
    pub const FILE_A: BitBoard = BitBoard(FILE_A);
    pub const FILE_B: BitBoard = BitBoard(FILE_B);
    pub const FILE_C: BitBoard = BitBoard(FILE_C);
    pub const FILE_D: BitBoard = BitBoard(FILE_D);
    pub const FILE_E: BitBoard = BitBoard(FILE_E);
    pub const FILE_F: BitBoard = BitBoard(FILE_F);
    pub const FILE_G: BitBoard = BitBoard(FILE_G);
    pub const FILE_H: BitBoard = BitBoard(FILE_H);
    pub const RANK_1: BitBoard = BitBoard(RANK_1);
    pub const RANK_2: BitBoard = BitBoard(RANK_2);
    pub const RANK_3: BitBoard = BitBoard(RANK_3);
    pub const RANK_4: BitBoard = BitBoard(RANK_4);
    pub const RANK_5: BitBoard = BitBoard(RANK_5);
    pub const RANK_6: BitBoard = BitBoard(RANK_6);
    pub const RANK_7: BitBoard = BitBoard(RANK_7);
    pub const RANK_8: BitBoard = BitBoard(RANK_8);

    pub const DARK_SQUARES: BitBoard = BitBoard(DARK_SQUARES);
    pub const LIGHT_SQUARES: BitBoard = BitBoard(LIGHT_SQUARES);
    pub const ALL: BitBoard = BitBoard(!0);


    pub fn from_coord_vec(coords: Vec<Coord>) -> Self {
        let mut bb = BitBoard(0);
        for c in coords.iter() {
            bb |= c.to_bitboard();
        };
        return bb;
    }


    pub fn from_file(file: i8) -> Self {
        Self::FILE_A << (file as usize)
    }

    pub fn from_rank(rank: i8) -> Self {
        Self::RANK_1 << (rank as usize * 8)
    }


    pub fn pop_lsb(&mut self) -> u32 {
        let i = DE_BRUIJN_TABLE[((((self.0 as i128) & -(self.0 as i128)) as u128 * DE_BRUIJN_64) as u64 >> 58) as usize];
        self.0 &= self.0 - 1;
        return i;
    }

    pub fn set_square(&mut self, sqr_idx: i8) {
        self.0 |= 1 << sqr_idx;
    }
    
    pub fn clear_square(&mut self, sqr_idx: i8) {
        self.0 &= !(1 << sqr_idx);
    }
    
    pub fn toggle_square(&mut self, sqr_idx: i8) {
        self.0 ^= 1 << sqr_idx;
    }
    
    pub fn toggle_squares(&mut self, sqr_a: i8, sqr_b: i8) {
        self.0 ^= 1 << sqr_a | 1 << sqr_b;
    }
    
    pub fn contains_square(&self, sqr_idx: i8) -> bool {
        ((self.0 >> sqr_idx) & 1) != 0
    }

    pub fn shifted(self, n: i8) -> BitBoard {
        if n > 0 {
            return self << n as usize;
        } else {
            return self >> -n as usize;
        }
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}


impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("");
        for rank in (0..8).rev() {
            let mut row = String::from("");
            for file in 0..8 {
                let coord = Coord::new(file, rank);
                if self.contains_square(coord.square()) {
                    row += "1 ";
                } else {
                    row += "• ";
                }
            }
            str += &row;
            str += "\n";
        };
        write!(f, "{}", str)
    }
}
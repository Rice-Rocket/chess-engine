use std::ops::{
    Shr, ShrAssign, Shl, ShlAssign,
    Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Add, AddAssign,
    BitXor, BitXorAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign,
    Not, Rem, RemAssign,
};
use crate::board::coord::Coord;
use super::masks::*;

const DE_BRUIJ_M: u64 = 0x03f7_9d71_b4cb_0a89;
const DE_BRUIJ_TABLE: [u32; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];


#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

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

    pub const FILES: [BitBoard; 8] = [
        Self::FILE_A,
        Self::FILE_B,
        Self::FILE_C,
        Self::FILE_D,
        Self::FILE_E,
        Self::FILE_F,
        Self::FILE_G,
        Self::FILE_H,
    ];

    pub const RANKS: [BitBoard; 8] = [
        Self::RANK_1,
        Self::RANK_2,
        Self::RANK_3,
        Self::RANK_4,
        Self::RANK_5,
        Self::RANK_6,
        Self::RANK_7,
        Self::RANK_8,
    ];

    pub const DARK_SQUARES: BitBoard = BitBoard(DARK_SQUARES);
    pub const LIGHT_SQUARES: BitBoard = BitBoard(LIGHT_SQUARES);
    pub const ALL: BitBoard = BitBoard(!0);


    pub fn from_coords(coords: Vec<Coord>) -> Self {
        let mut bb = BitBoard(0);
        for c in coords.iter() {
            bb |= c.to_bitboard();
        };
        bb
    }


    pub fn from_file(file: i8) -> Self {
        Self::FILE_A << (file as usize)
    }

    pub fn from_rank(rank: i8) -> Self {
        Self::RANK_1 << (rank as usize * 8)
    }


    pub fn pop_lsb(&mut self) -> u32 {
        let i = unsafe { *DE_BRUIJ_TABLE.get_unchecked(
            (((self.0 ^ self.0.wrapping_sub(1)).wrapping_mul(DE_BRUIJ_M)).wrapping_shr(58)) as usize,
        ) };
        self.0 &= self.0 - 1;
        i
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
        self.0 ^= (1 << sqr_a) | (1 << sqr_b);
    }
    
    pub fn contains_square(&self, sqr_idx: i8) -> bool {
        ((self.0 >> sqr_idx) & 1) != 0
    }

    pub fn get_square(&self, sqr: Coord) -> bool {
        if sqr.is_valid() {
            ((self.0 >> sqr.square()) & 1) != 0
        } else {
            false
        }
    }

    pub fn shifted(self, n: i8) -> BitBoard {
        if n > 0 {
            self << n as usize
        } else {
            self >> -n as usize
        }
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl From<usize> for BitBoard {
    fn from(value: usize) -> Self {
        BitBoard(unsafe {
            std::mem::transmute_copy(&value)
        })
    }
}


impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("\n\r");
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
            str += "\n\r";
        };
        write!(f, "{}", str)
    }
}


macro_rules! impl_indv_shift_ops {
    ($t:ty, $tname:ident, $fname:ident, $w:ident, $ta_name:ident, $fa_name:ident) => {
        impl $tname<usize> for $t {
            type Output = $t;

            #[inline]
            fn $fname(self, rhs: usize) -> $t {
                Self::from((self.0).$w(rhs as u32))
            }
        }

        impl $ta_name<usize> for $t {
            #[inline]
            fn $fa_name(&mut self, rhs: usize) {
                *self = Self::from((self.0).$w(rhs as u32));
            }
        }
    };
}

macro_rules! impl_indv_bit_ops {
    ($t:ty, $b:ty, $tname:ident, $fname:ident, $w:ident, $ta_name:ident, $fa_name:ident) => {
        impl $tname for $t {
            type Output = $t;

            #[inline]
            fn $fname(self, rhs: $t) -> $t {
                Self::from((self.0).$w(rhs.0))
            }
        }

        impl $ta_name for $t {
            #[inline]
            fn $fa_name(&mut self, rhs: $t) {
                *self = Self::from((self.0).$w(rhs.0));
            }
        }

        impl $tname<$b> for $t {
            type Output = $t;

            #[inline]
            fn $fname(self, rhs: $b) -> $t {
                Self::from((self.0).$w(rhs))
            }
        }

        impl $ta_name<$b> for $t {
            #[inline]
            fn $fa_name(&mut self, rhs: $b) {
                *self = Self::from((self.0).$w(rhs));
            }
        }
    };
}

macro_rules! impl_bit_ops {
    ($t:tt, $b:tt) => {
        impl From<$b> for $t {
            fn from(bit_type: $b) -> Self {
                $t(bit_type)
            }
        }

        impl From<$t> for $b {
            fn from(it: $t) -> Self {
                it.0
            }
        }

        impl_indv_bit_ops!($t, $b, Rem, rem, rem, RemAssign, rem_assign);
        impl_indv_bit_ops!($t, $b, BitOr, bitor, bitor, BitOrAssign, bitor_assign);
        impl_indv_bit_ops!($t, $b, BitAnd, bitand, bitand, BitAndAssign, bitand_assign);
        impl_indv_bit_ops!($t, $b, BitXor, bitxor, bitxor, BitXorAssign, bitxor_assign);

        impl_indv_bit_ops!($t, $b, Add, add, wrapping_add, AddAssign, add_assign);
        impl_indv_bit_ops!($t, $b, Div, div, wrapping_div, DivAssign, div_assign);
        impl_indv_bit_ops!($t, $b, Mul, mul, wrapping_mul, MulAssign, mul_assign);
        impl_indv_bit_ops!($t, $b, Sub, sub, wrapping_sub, SubAssign, sub_assign);

        impl_indv_shift_ops!($t, Shl, shl, wrapping_shl, ShlAssign, shl_assign);
        impl_indv_shift_ops!($t, Shr, shr, wrapping_shr, ShrAssign, shr_assign);

        impl Not for $t {
            type Output = $t;

            #[inline]
            fn not(self) -> $t {
                $t(!self.0)
            }
        }
    };
}

impl_bit_ops!(BitBoard, u64);

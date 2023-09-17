use crate::game::representation::square_name_from_idx;

use super::piece::*;
use super::coord::*;


const START_SQUARE_MASK: u16 = 0b0000000000111111;
const TARGET_SQUARE_MASK: u16 = 0b0000111111000000;
// const FLAG_MASK: u16 = 0b1111000000000000;


#[derive(Clone, Copy)]
pub struct Move {
    val: u16,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", square_name_from_idx(self.start_idx()), square_name_from_idx(self.target_idx()))
    }
}


impl Move {
    pub const NORMAL: u8 = 0;
    pub const EN_PASSANT_CAPTURE: u8 = 1;
    pub const CASTLING: u8 = 2;
    pub const QUEEN_PROMOTION: u8 = 3;
    pub const KNIGHT_PROMOTION: u8 = 4;
    pub const ROOK_PROMOTION: u8 = 5;
    pub const BISHOP_PROMOTION: u8 = 6;
    pub const PAWN_TWO_FORWARD: u8 = 7;

    pub const NULL: Move = Move { val: 0 };

    pub fn from_value(val: u16) -> Self {
        Self {
            val
        }
    }
    pub fn from_start_end(start: i8, target: i8) -> Self {
        Self {
            val: (start as u16) | ((target as u16) << 6),
        }
    }
    pub fn from_start_end_flagged(start: i8, target: i8, flag: u8) -> Self {
        Self {
            val: (start as u16) | ((target as u16) << 6) | ((flag as u16) << 12),
        }
    }
    pub fn same_move(a: Self, b: Self) -> bool {
        a.val == b.val
    }
    pub fn start_idx(&self) -> i8 {
        (self.val & START_SQUARE_MASK) as i8
    }
    pub fn target_idx(&self) -> i8 {
        ((self.val & TARGET_SQUARE_MASK) >> 6) as i8
    }
    pub fn start(&self) -> Coord {
        Coord::from_idx(self.start_idx())
    }
    pub fn target(&self) -> Coord {
        Coord::from_idx(self.target_idx())
    }
    pub fn is_promotion(&self) -> bool {
        let flag = self.move_flag();
        flag == Move::QUEEN_PROMOTION || flag == Move::ROOK_PROMOTION || flag == Move::KNIGHT_PROMOTION || flag == Move::BISHOP_PROMOTION
    }
    pub fn move_flag(&self) -> u8 {
        (self.val >> 12) as u8
    }
    pub fn promotion_ptype(&self) -> u8 {
        match self.move_flag() {
            3 => Piece::QUEEN,
            4 => Piece::KNIGHT,
            5 => Piece::ROOK,
            6 => Piece::BISHOP,
            _ => Piece::NONE
        }
    }
    pub fn value(&self) -> u16 {
        self.val
    }
    pub fn is_invalid(&self) -> bool {
        self.val == 0
    }
    pub fn name(&self) -> String {
        "Not implemented. In: move.rs/Move/name".to_string()
    }
}
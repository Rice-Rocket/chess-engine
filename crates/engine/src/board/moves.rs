use crate::utils::representation;
use crate::utils::representation::square_name_from_coord;
use crate::utils::representation::square_name_from_idx;

use super::piece::*;
use super::coord::*;


const START_SQUARE_MASK: u16 = 0b0000000000111111;
const TARGET_SQUARE_MASK: u16 = 0b0000111111000000;
const FLAG_MASK: u16 = 0b1111000000000000;


#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct Move(u16);

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", square_name_from_coord(self.start().file(), self.start().rank()), square_name_from_coord(self.target().file(), self.target().rank()))
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

    pub const NULL: Move = Move(0);

    pub fn from_value(val: u16) -> Self {
        Self(val)
    }

    pub fn from_start_end(start: i8, target: i8) -> Self {
        Self((start as u16) | ((target as u16) << 6))
    }

    pub fn from_start_end_flagged(start: i8, target: i8, flag: u8) -> Self {
        Self((start as u16) | ((target as u16) << 6) | ((flag as u16) << 12))
    }

    pub fn same_move(a: Self, b: Self) -> bool {
        a.0 == b.0
    }

    pub fn same_move_and_prom(mut self, mut rhs: Self) -> bool {
        if self.0 == rhs.0 {
            true
        } else {
            let flag1 = self.move_flag();
            let flag2 = rhs.move_flag();
            self.0 &= !FLAG_MASK;
            rhs.0 &= !FLAG_MASK;
            self.0 |= if (3..=6).contains(&flag1) {
                (flag1 as u16) << 12
            } else { 0 };
            rhs.0 |= if (3..=6).contains(&flag2) {
                (flag2 as u16) << 12
            } else { 0 };
            self.0 == rhs.0
        }
    }

    pub fn start_idx(&self) -> i8 {
        (self.0 & START_SQUARE_MASK) as i8
    }

    pub fn target_idx(&self) -> i8 {
        ((self.0 & TARGET_SQUARE_MASK) >> 6) as i8
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
        (self.0 >> 12) as u8
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
        self.0
    }

    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }

    pub fn name(&self) -> String {
        match self.move_flag() {
            Move::CASTLING => if self.target().file() == 6 { String::from("O-O") } else { String::from("O-O-O") },
            Move::KNIGHT_PROMOTION => format!("{:?}{:?}n", self.start(), self.target()),
            Move::BISHOP_PROMOTION => format!("{:?}{:?}b", self.start(), self.target()),
            Move::ROOK_PROMOTION => format!("{:?}{:?}r", self.start(), self.target()),
            Move::QUEEN_PROMOTION => format!("{:?}{:?}q", self.start(), self.target()),
            _ => format!("{:?}{:?}", self.start(), self.target()),
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::{Coord, Move};

    #[test]
    fn test_move_start_end() {
        let start = Coord::A8;
        let target = Coord::A1;
        let m = Move::from_start_end(start.square(), target.square());
        assert_eq!(start, m.start());
        assert_eq!(target, m.target());
    }
}

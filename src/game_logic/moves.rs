use super::piece;


pub const NONE: u32 = 0;
pub const EN_PASSANT_CAPTURE: u32 = 1;
pub const CASTLING: u32 = 2;
pub const QUEEN_PROMOTION: u32 = 3;
pub const KNIGHT_PROMOTION: u32 = 4;
pub const ROOK_PROMOTION: u32 = 5;
pub const BISHOP_PROMOTION: u32 = 6;
pub const PAWN_TWO_FORWARD: u32 = 7;


const START_SQUARE_MASK: u16 = 0b0000000000111111;
const TARGET_SQUARE_MASK: u16 = 0b0000111111000000;
const FLAG_MASK: u16 = 0b1111000000000000;


#[derive(Clone, Copy)]
pub struct Move {
    pub move_value: u16,
}


impl Move {
    pub fn from_value(move_value: u16) -> Self {
        Self {
            move_value: move_value,
        }
    }
    pub fn from_start_end(start: u32, target: u32) -> Self {
        Self {
            move_value: (start | target << 6) as u16,
        }
    }
    pub fn from_start_end_flagged(start: u32, target: u32, flag: u32) -> Self {
        Self {
            move_value: (start | target << 6 | flag << 12) as u16,
        }
    }
    pub fn invalid_move() -> Self {
        Self::from_value(0)
    }
    pub fn same_move(a: Self, b: Self) -> bool {
        a.move_value == b.move_value
    }
    pub fn start(&self) -> u32 {
        (self.move_value & START_SQUARE_MASK) as u32
    }
    pub fn target(&self) -> u32 {
        ((self.move_value & TARGET_SQUARE_MASK) >> 6) as u32
    }
    pub fn is_promotion(&self) -> bool {
        let flag = self.move_flag();
        flag == QUEEN_PROMOTION || flag == ROOK_PROMOTION || flag == KNIGHT_PROMOTION || flag == BISHOP_PROMOTION
    }
    pub fn move_flag(&self) -> u32 {
        (self.move_value >> 12) as u32
    }
    pub fn promotion_ptype(&self) -> u32 {
        match self.move_flag() {
            3 => piece::QUEEN,
            4 => piece::KNIGHT,
            5 => piece::ROOK,
            6 => piece::BISHOP,
            _ => piece::NONE
        }
    }
    pub fn value(&self) -> u16 {
        self.move_value
    }
    pub fn is_invalid(&self) -> bool {
        self.move_value == 0
    }
    pub fn name(&self) -> String {
        "Not implemented. In: move.rs/Move/name".to_string()
    }
}
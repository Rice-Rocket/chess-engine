
pub const NONE: u32 = 0;
pub const KING: u32 = 1;
pub const PAWN: u32 = 2;
pub const KNIGHT: u32 = 3;
pub const BISHOP: u32 = 5;
pub const ROOK: u32 = 6;
pub const QUEEN: u32 = 7;


pub const WHITE: u32 = 8;
pub const BLACK: u32 = 16;
const TYPE_MASK: u32 = 0b00111;
const BLACK_MASK: u32 = 0b10000;
const WHITE_MASK: u32 = 0b01000;
const COLOR_MASK: u32 = BLACK_MASK | WHITE_MASK;

// pub struct Piece {
//     val: u8,
// }

// impl Piece {
//     pub const NONE: Self = Self { val: 0 };
//     pub const KING: Self = Self { val: 1 };
//     pub const PAWN: Self = Self { val: 2 };
//     pub const KNIGHT: Self = Self { val: 3 };
//     pub const BISHOP: Self = Self { val: 5 };
//     pub const ROOK: Self = Self { val: 6 };
//     pub const QUEEN: Self = Self { val: 7 };

//     pub fn is_color(&self, color: Team) -> bool {
//         return (self.val & COLOR_MASK)
//     }
// }


pub fn is_color(piece: u32, color: u32) -> bool {
    return (piece & COLOR_MASK) == color;
}

pub fn color(piece: u32) -> u32 {
    return piece & COLOR_MASK;
}

pub fn piece_type(piece: u32) -> u32 {
    return piece & TYPE_MASK;
}

pub fn is_rook_or_queen(piece: u32) -> bool {
    return (piece & 0b110) == 0b110;
}

pub fn is_bishop_or_queen(piece: u32) -> bool {
    return (piece & 0b101) == 0b101;
}

pub fn is_sliding_piece(piece: u32) -> bool {
    return (piece & 0b100) != 0;
}
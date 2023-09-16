const TYPE_MASK: u8 = 0b00111;
const BLACK_MASK: u8 = 0b10000;
const WHITE_MASK: u8 = 0b01000;
const COLOR_MASK: u8 = BLACK_MASK | WHITE_MASK;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    val: u8,
}

impl Piece {
    pub const NONE: u8 = 0;
    pub const KING: u8 = 1;
    pub const PAWN: u8 = 2;
    pub const KNIGHT: u8 = 3;
    pub const BISHOP: u8 = 5;
    pub const ROOK: u8 = 6;
    pub const QUEEN: u8 = 7;

    pub const WHITE: u8 = 8;
    pub const BLACK: u8 = 16;

    pub const NULL: Self = Self { val: 0 };

    pub fn new(val: u8) -> Piece {
        Piece {
            val
        }
    }
    pub fn value(self) -> u8 {
        self.val
    }
    pub fn is_color(self, color: u8) -> bool {
        return (self.val & COLOR_MASK) == color
    }
    pub fn color(self) -> u8 {
        return self.val & COLOR_MASK;
    }
    pub fn piece_type(self) -> u8 {
        return self.val & TYPE_MASK;
    }
    pub fn is_rook_or_queen(self) -> bool {
        return (self.val & 0b110) == 0b110;
    }
    pub fn is_bishop_or_queen(self) -> bool {
        return (self.val & 0b101) == 0b101;
    }
    pub fn is_sliding_piece(self) -> bool {
        return (self.val & 0b100) != 0;
    }
}





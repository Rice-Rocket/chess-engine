use std::ops::{BitOr, BitOrAssign};

use super::board::Board;

const TYPE_MASK: u8 = 0b0111;
const COLOR_MASK: u8 = 0b1000;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    val: u8,
}

impl Piece {
    pub const NONE: u8 = 0;   // 000
    pub const PAWN: u8 = 1;   // 001
    pub const KNIGHT: u8 = 2; // 010
    pub const BISHOP: u8 = 3; // 011
    pub const ROOK: u8 = 4;   // 100
    pub const QUEEN: u8 = 5;  // 101
    pub const KING: u8 = 6;   // 110

    pub const WHITE: u8 = 0;
    pub const BLACK: u8 = 8;

    pub const WHITE_PAWN: u8 = Piece::PAWN | Piece::WHITE;
    pub const WHITE_KNIGHT: u8 = Piece::KNIGHT | Piece::WHITE;
    pub const WHITE_BISHOP: u8 = Piece::BISHOP | Piece::WHITE;
    pub const WHITE_ROOK: u8 = Piece::ROOK | Piece::WHITE;
    pub const WHITE_QUEEN: u8 = Piece::QUEEN | Piece::WHITE;
    pub const WHITE_KING: u8 = Piece::KING | Piece::WHITE;

    pub const BLACK_PAWN: u8 = Piece::PAWN | Piece::BLACK;
    pub const BLACK_KNIGHT: u8 = Piece::KNIGHT | Piece::BLACK;
    pub const BLACK_BISHOP: u8 = Piece::BISHOP | Piece::BLACK;
    pub const BLACK_ROOK: u8 = Piece::ROOK | Piece::BLACK;
    pub const BLACK_QUEEN: u8 = Piece::QUEEN | Piece::BLACK;
    pub const BLACK_KING: u8 = Piece::KING | Piece::BLACK;

    pub const MAX_PIECE_INDEX: u8 = Piece::BLACK_KING;

    pub const PIECE_INDICES: [u8; 12] = [
        Piece::WHITE_PAWN, Piece::WHITE_KNIGHT, Piece::WHITE_BISHOP, Piece::WHITE_ROOK, Piece::WHITE_QUEEN, Piece::WHITE_KING,
        Piece::BLACK_PAWN, Piece::BLACK_KNIGHT, Piece::BLACK_BISHOP, Piece::BLACK_ROOK, Piece::BLACK_QUEEN, Piece::BLACK_KING,
    ];

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
        return (self.val & COLOR_MASK) == color && self.val != 0
    }
    pub fn color(self) -> u8 {
        return self.val & COLOR_MASK;
    }
    pub fn is_white(self) -> bool {
        self.color() == Piece::WHITE
    }
    pub fn piece_type(self) -> u8 {
        return self.val & TYPE_MASK;
    }
    pub fn is_rook_or_queen(self) -> bool {
        return self.piece_type() == Self::QUEEN || self.piece_type() == Self::ROOK;
    }
    pub fn is_bishop_or_queen(self) -> bool {
        return self.piece_type() == Self::QUEEN || self.piece_type() == Self::BISHOP;
    }
    pub fn is_sliding_piece(self) -> bool {
        return self.is_bishop_or_queen() || self.piece_type() == Self::ROOK;
    }
    pub fn index(self) -> usize {
        self.val as usize
    }
    pub fn color_index(self) -> usize {
        if self.is_white() { Board::WHITE_INDEX } else { Board::BLACK_INDEX }
    }
}


impl BitOr for Piece {
    type Output = Piece;
    fn bitor(self, rhs: Self) -> Self::Output {
        Piece::new(self.val | rhs.val)
    }
}

impl BitOrAssign for Piece {
    fn bitor_assign(&mut self, rhs: Self) {
        self.val |= rhs.val
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Piece({},{})", if self.is_white() { "white" } else { "black" }, match self.piece_type() {
            Piece::PAWN => "pawn",
            Piece::KNIGHT => "knight",
            Piece::BISHOP => "bishop",
            Piece::ROOK => "rook",
            Piece::QUEEN => "queen",
            Piece::KING => "king",
            _ => "none"
        }))
    }
}
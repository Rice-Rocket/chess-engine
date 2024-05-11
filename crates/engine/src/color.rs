use std::ops::{Index, IndexMut};

use crate::board::{coord::Coord, piece::Piece, Board};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black
}

impl Color {
    #[inline]
    pub fn flip(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn piece(self, ptype: u8) -> Piece {
        match self {
            Color::White => Piece::new(ptype | Piece::WHITE),
            Color::Black => Piece::new(ptype | Piece::BLACK),
        }
    }

    #[inline]
    pub fn piece_color(self) -> u8 {
        match self {
            Color::White => Piece::WHITE,
            Color::Black => Piece::BLACK,
        }
    }

    /// The `up` direction of this color, traveling towards the enemy side. 
    #[inline]
    pub fn up(self) -> Coord {
        match self {
            Color::White => Coord::new(0, 1),
            Color::Black => Coord::new(0, -1),
        }
    }

    /// The `down` direction of this color, traveling towards the home side. 
    #[inline]
    pub fn down(self) -> Coord {
        match self {
            Color::White => Coord::new(0, -1),
            Color::Black => Coord::new(0, 1),
        }
    }

    #[inline]
    pub fn at(self, file: i8, rank: i8) -> Coord {
        match self {
            Color::White => Coord::new(file, rank),
            Color::Black => Coord::new(file, 7 - rank),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

impl<T> Index<Color> for Vec<T> {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self[Board::WHITE_INDEX],
            Color::Black => &self[Board::BLACK_INDEX],
        }
    }
}

impl<T> IndexMut<Color> for Vec<T> {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self[Board::WHITE_INDEX],
            Color::Black => &mut self[Board::BLACK_INDEX],
        }
    }
}

impl<T> Index<Color> for [T] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self[Board::WHITE_INDEX],
            Color::Black => &self[Board::BLACK_INDEX],
        }
    }
}

impl<T> IndexMut<Color> for [T] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self[Board::WHITE_INDEX],
            Color::Black => &mut self[Board::BLACK_INDEX],
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::board::{piece::Piece, zobrist::Zobrist};

    use super::*;

    #[test]
    fn test_up_down() {
        let board = Board::load_position(None, &mut Zobrist::new());
        assert_eq!(board.square[Coord::new(2, 1)], Piece::new(Piece::WHITE_PAWN));
        assert_eq!(board.square[Coord::new(2, 1) + Color::White.down()], Piece::new(Piece::WHITE_BISHOP));
        assert_eq!(board.square[Coord::new(4, 6) + Color::Black.down()], Piece::new(Piece::BLACK_KING));
    }

    #[test]
    fn test_at() {
        let board = Board::load_position(None, &mut Zobrist::new());
        assert_eq!(board.square[Color::White.at(2, 1)], Piece::new(Piece::WHITE_PAWN));
        assert_eq!(board.square[Color::Black.at(2, 1)], Piece::new(Piece::BLACK_PAWN));
    }
}

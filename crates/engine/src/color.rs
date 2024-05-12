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
    pub fn back_rank(self) -> i8 {
        match self {
            Color::White => 0,
            Color::Black => 7,
        }
    }

    #[inline]
    pub fn first_rank(self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }
    
    #[inline]
    pub fn up_dir(self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    #[inline]
    pub fn down_dir(self) -> i8 {
        match self {
            Color::White => -1,
            Color::Black => 1,
        }
    }

    /// Iterates over ranks in the `up` direction of this color.
    #[inline]
    pub fn ranks_up(self) -> RanksIterator {
        match self {
            Color::White => RanksIterator::new(1, 0),
            Color::Black => RanksIterator::new(-1, 7),
        }
    }

    /// Iterates over ranks in the `down` direction of this color.
    #[inline]
    pub fn ranks_down(self) -> RanksIterator {
        match self {
            Color::White => RanksIterator::new(-1, 7),
            Color::Black => RanksIterator::new(1, 0),
        }
    }

    /// Iterates over ranks in the `up` direction of this color, starting at and including rank
    /// `start`.
    #[inline]
    pub fn ranks_up_from(self, start: i8) -> RanksIterator {
        match self {
            Color::White => RanksIterator::new(1, start),
            Color::Black => RanksIterator::new(-1, start),
        }
    }

    /// Iterates over ranks in the `down` direction of this color, starting at and including rank
    /// `start`.
    #[inline]
    pub fn ranks_down_from(self, start: i8) -> RanksIterator {
        match self {
            Color::White => RanksIterator::new(-1, start),
            Color::Black => RanksIterator::new(1, start),
        }
    }

    #[inline]
    pub fn ranks_up_till_incl(self, end: i8) -> RanksIteratorUntil {
        match self {
            Color::White => RanksIteratorUntil::new(1, 0, end + 1),
            Color::Black => RanksIteratorUntil::new(-1, 7, end - 1),
        }
    }

    #[inline]
    pub fn ranks_down_till_incl(self, end: i8) -> RanksIteratorUntil {
        match self {
            Color::White => RanksIteratorUntil::new(-1, 7, end - 1),
            Color::Black => RanksIteratorUntil::new(1, 0, end + 1),
        }
    }

    #[inline]
    pub fn ranks_up_till_excl(self, end: i8) -> RanksIteratorUntil {
        match self {
            Color::White => RanksIteratorUntil::new(1, 0, end),
            Color::Black => RanksIteratorUntil::new(-1, 7, end),
        }
    }

    #[inline]
    pub fn ranks_down_till_excl(self, end: i8) -> RanksIteratorUntil {
        match self {
            Color::White => RanksIteratorUntil::new(-1, 7, end),
            Color::Black => RanksIteratorUntil::new(1, 0, end),
        }
    }

    #[inline]
    pub fn at(self, file: i8, rank: i8) -> Coord {
        match self {
            Color::White => Coord::new(file, rank),
            Color::Black => Coord::new(file, 7 - rank),
        }
    }

    #[inline]
    pub fn is_white(self) -> bool {
        match self {
            Color::White => true,
            Color::Black => false,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}


pub struct RanksIterator {
    direction: i8,
    current: i8,
}

impl RanksIterator {
    pub fn new(direction: i8, start: i8) -> Self {
        Self { direction, current: start - direction }
    }
}

impl Iterator for RanksIterator {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += self.direction;
        if self.current < 0 || self.current > 7 { return None };
        Some(self.current)
    }
}


pub struct RanksIteratorUntil {
    direction: i8,
    current: i8,
    end: i8,
}

impl RanksIteratorUntil {
    pub fn new(direction: i8, start: i8, end: i8) -> Self {
        Self { direction, current: start - direction, end }
    }
}

impl Iterator for RanksIteratorUntil {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += self.direction;
        if self.current < 0 || self.current > 7 || self.current == self.end { return None };
        Some(self.current)
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

    #[test]
    fn test_ranks_iter() {
        let mut it = RanksIterator::new(-1, 7);
        assert_eq!(it.next(), Some(7));
        assert_eq!(it.next(), Some(6));
        assert_eq!(it.next(), Some(5));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), None);
    }
}

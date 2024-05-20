use std::ops::{Index, IndexMut, Range, RangeInclusive};

use crate::{board::{coord::Coord, piece::Piece, Board}, prelude::BitBoard};

pub trait Color {
    fn flip() -> impl Color;
    fn piece(ptype: u8) -> Piece;
    fn piece_color() -> u8;
    /// The `up` direction of this color, traveling towards the enemy side. 
    fn up() -> Coord;
    /// The `down` direction of this color, traveling towards the home side. 
    fn down() -> Coord;
    fn rank(n: i8) -> i8;
    fn ranks(n: RangeInclusive<i8>) -> RangeInclusive<i8>;
    fn back_rank() -> i8;
    fn first_rank() -> i8;
    fn home_side() -> BitBoard;
    fn up_dir() -> i8;
    fn down_dir() -> i8;
    /// Iterates over ranks in the `up` direction of this color.
    fn ranks_up() -> RanksIterator;
    /// Iterates over ranks in the `down` direction of this color.
    fn ranks_down() -> RanksIterator;
    /// Iterates over ranks in the `up` direction of this color, starting at and including rank
    /// `start`.
    fn ranks_up_from(start: i8) -> RanksIterator;
    /// Iterates over ranks in the `down` direction of this color, starting at and including rank
    /// `start`.
    fn ranks_down_from(start: i8) -> RanksIterator;
    fn ranks_up_till_incl(end: i8) -> RanksIteratorUntil;
    fn ranks_down_till_incl(end: i8) -> RanksIteratorUntil;
    fn ranks_up_till_excl(end: i8) -> RanksIteratorUntil;
    fn ranks_down_till_excl(end: i8) -> RanksIteratorUntil;
    fn at(file: i8, rank: i8) -> Coord;
    fn offset(file: i8, rank: i8) -> Coord;
    fn is_white() -> bool;
    fn index() -> usize;
}


pub struct White;
pub struct Black;

impl Color for White {
    fn flip() -> impl Color {
        Black
    }

    fn piece(ptype: u8) -> Piece {
        Piece::new(ptype | Piece::WHITE)
    }

    fn piece_color() -> u8 {
        Piece::WHITE
    }

    fn up() -> Coord {
        Coord::new(0, 1)
    }

    fn down() -> Coord {
        Coord::new(0, -1)
    }

    fn rank(n: i8) -> i8 {
        n
    }

    fn ranks(n: RangeInclusive<i8>) -> RangeInclusive<i8> {
        n
    }

    fn back_rank() -> i8 {
        0
    }

    fn first_rank() -> i8 {
        1
    }

    fn home_side() -> BitBoard {
        BitBoard::RANK_1 | BitBoard::RANK_2 | BitBoard::RANK_3 | BitBoard::RANK_4
    }

    fn up_dir() -> i8 {
        1
    }

    fn down_dir() -> i8 {
        -1
    }

    fn ranks_up() -> RanksIterator {
        RanksIterator::new(1, 0)
    }

    fn ranks_down() -> RanksIterator {
        RanksIterator::new(-1, 7)
    }

    fn ranks_up_from(start: i8) -> RanksIterator {
        RanksIterator::new(1, start)
    }

    fn ranks_down_from(start: i8) -> RanksIterator {
        RanksIterator::new(-1, start)
    }

    fn ranks_up_till_incl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(1, 0, end + 1)
    }

    fn ranks_down_till_incl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(-1, 7, end - 1)
    }

    fn ranks_up_till_excl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(1, 0, end)
    }

    fn ranks_down_till_excl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(-1, 7, end)
    }

    fn at(file: i8, rank: i8) -> Coord {
        Coord::new(file, rank)
    }

    fn offset(file: i8, rank: i8) -> Coord {
        Coord::new(file, rank)
    }

    fn is_white() -> bool {
        true
    }

    fn index() -> usize {
        Board::WHITE_INDEX
    }
}

impl Color for Black {
    fn flip() -> impl Color {
        White
    }

    fn piece(ptype: u8) -> Piece {
        Piece::new(ptype | Piece::BLACK)
    }

    fn piece_color() -> u8 {
        Piece::BLACK
    }

    fn up() -> Coord {
        Coord::new(0, -1)
    }

    fn down() -> Coord {
        Coord::new(0, 1)
    }

    fn rank(n: i8) -> i8 {
        7 - n
    }

    fn ranks(n: RangeInclusive<i8>) -> RangeInclusive<i8> {
        (7 - n.end())..=(7 - n.start())
    }

    fn back_rank() -> i8 {
        7
    }

    fn first_rank() -> i8 {
        6
    }

    fn home_side() -> BitBoard {
        BitBoard::RANK_5 | BitBoard::RANK_6 | BitBoard::RANK_7 | BitBoard::RANK_8
    }

    fn up_dir() -> i8 {
        -1
    }

    fn down_dir() -> i8 {
        1
    }

    fn ranks_up() -> RanksIterator {
        RanksIterator::new(-1, 7)
    }

    fn ranks_down() -> RanksIterator {
        RanksIterator::new(1, 0)
    }

    fn ranks_up_from(start: i8) -> RanksIterator {
        RanksIterator::new(-1, start)
    }

    fn ranks_down_from(start: i8) -> RanksIterator {
        RanksIterator::new(1, start)
    }

    fn ranks_up_till_incl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(-1, 7, end - 1)
    }

    fn ranks_down_till_incl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(1, 0, end + 1)
    }

    fn ranks_up_till_excl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(-1, 7, end)
    }

    fn ranks_down_till_excl(end: i8) -> RanksIteratorUntil {
        RanksIteratorUntil::new(1, 0, end)
    }

    fn at(file: i8, rank: i8) -> Coord {
        Coord::new(file, 7 - rank)
    }

    fn offset(file: i8, rank: i8) -> Coord {
        Coord::new(file, -rank)
    }

    fn is_white() -> bool {
        false
    }

    fn index() -> usize {
        Board::BLACK_INDEX
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



#[cfg(test)]
mod tests {
    use crate::board::{piece::Piece, zobrist::Zobrist};

    use super::*;

    #[test]
    fn test_up_down() {
        let board = Board::load_position(None, &mut Zobrist::new());
        assert_eq!(board.square[Coord::new(2, 1)], Piece::new(Piece::WHITE_PAWN));
        assert_eq!(board.square[Coord::new(2, 1) + White::down()], Piece::new(Piece::WHITE_BISHOP));
        assert_eq!(board.square[Coord::new(4, 6) + Black::down()], Piece::new(Piece::BLACK_KING));
    }

    #[test]
    fn test_at() {
        let board = Board::load_position(None, &mut Zobrist::new());
        assert_eq!(board.square[White::at(2, 1)], Piece::new(Piece::WHITE_PAWN));
        assert_eq!(board.square[Black::at(2, 1)], Piece::new(Piece::BLACK_PAWN));
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

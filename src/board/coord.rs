use crate::{utils::representation::square_name_from_coord, bitboard::bb::BitBoard};
use std::ops::{Add, Sub, Mul};

// First 4 bits: rank, last 4 bits: file
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord(i8);

impl Coord {
    pub const ROOK_DIRECTIONS: [Self; 4] = [Coord::new(-1, 0), Coord::new(1, 0), Coord::new(0, 1), Coord::new(0, -1)];
    pub const BISHOP_DIRECTIONS: [Self; 4] = [Coord::new(-1, 1), Coord::new(1, 1), Coord::new(1, -1), Coord::new(-1, -1)];

    pub const A1: Self = Self::new(0, 0);
    pub const B1: Self = Self::new(1, 0);
    pub const C1: Self = Self::new(2, 0);
    pub const D1: Self = Self::new(3, 0);
    pub const E1: Self = Self::new(4, 0);
    pub const F1: Self = Self::new(5, 0);
    pub const G1: Self = Self::new(6, 0);
    pub const H1: Self = Self::new(7, 0);

    pub const A8: Self = Self::new(0, 7);
    pub const B8: Self = Self::new(1, 7);
    pub const C8: Self = Self::new(2, 7);
    pub const D8: Self = Self::new(3, 7);
    pub const E8: Self = Self::new(4, 7);
    pub const F8: Self = Self::new(5, 7);
    pub const G8: Self = Self::new(6, 7);
    pub const H8: Self = Self::new(7, 7);

    pub const NULL: Self = Self::A1;

    #[inline]
    pub const fn new(file: i8, rank: i8) -> Self {
        debug_assert!(file.abs() < 8);
        debug_assert!(rank.abs() < 8);
        Self::new_unchecked(file, rank)
    }

    #[inline]
    const fn new_unchecked(file: i8, rank: i8) -> Self {
        let sign_file = if file.signum() == -1 { 0b00001000 } else { 0 };
        let sign_rank = if rank.signum() == -1 { -128i8 /* 0b10000000 */ } else { 0 };
        Self(((rank & 0b00001111) << 4) | (file & 0b00001111) | sign_file | sign_rank)
    }

    #[inline]
    pub fn new_clamp(file: i8, rank: i8) -> Self {
        Self::new_unchecked(file.clamp(0, 7), rank.clamp(0, 7))
    }

    /// Iterates squares from indices [0, 64)
    #[inline]
    pub const fn iter_squares() -> CoordIterator {
        CoordIterator { curr: -1 }
    }

    #[inline]
    pub const fn from_idx(idx: i8) -> Self {
        Self::new(idx & 0b000111, (idx >> 3) & 0b000111)
    }

    #[inline]
    pub const fn rank(self) -> i8 {
        ((self.0 & 0b01110000) >> 4) * (if self.0 >> 7  == 0 { 1 } else { -1 })
    }

    #[inline]
    pub const fn file(self) -> i8 {
        (self.0 & 0b00000111) * (if (self.0 & 0b00001000) >> 3 == 0 { 1 } else { -1 })
    }

    pub fn is_light_square(self) -> bool {
        (self.file() + self.rank()) % 2 != 0
    }

    /// The index of the coord (usize)
    #[inline]
    pub const fn index(self) -> usize {
        self.square() as usize
    }

    /// The square index of the coord (i8)
    #[inline]
    pub const fn square(self) -> i8 {
        ((self.0 & 0b01110000) >> 1) + (self.0 & 0b00000111)
    }

    /// Performs other - self
    #[inline]
    pub fn delta(self, other: Coord) -> Coord {
        Coord::new(other.file() - self.file(), other.rank() - self.rank())
    }

    /// Checks if coord is inside bounds
    #[inline]
    pub fn is_valid(&self) -> bool {
        (self.0 & -128 /* 0b10000000 */) == 0 && (self.0 & 0b00001000) == 0
    }

    /// Converts the coord into a bitboard with only that square
    pub fn to_bitboard(self) -> BitBoard {
        BitBoard(1u64 << self.index())
    }

    pub fn flip_rank(self) -> Coord {
        Coord::new(self.file(), 7 - self.rank())
    }

    pub fn flip_file(self) -> Coord {
        Coord::new(7 - self.file(), self.rank())
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", square_name_from_coord(self.file(), self.rank()))
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord::new_unchecked(self.file() + rhs.file(), self.rank() + rhs.rank())
    }
}

impl Add<i8> for Coord {
    type Output = Coord;
    fn add(self, rhs: i8) -> Self::Output {
        Coord::from_idx(self.square() + rhs)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord::new_unchecked(self.file() - rhs.file(), self.rank() - rhs.rank())
    }
}

impl Sub<i8> for Coord {
    type Output = Coord;
    fn sub(self, rhs: i8) -> Self::Output {
        let rank = rhs >> 3;
        let file = rhs & 0b000111;
        Coord::new_unchecked(self.file() - file, self.rank() - rank)
    }
}

impl Mul<i8> for Coord {
    type Output = Coord;
    fn mul(self, rhs: i8) -> Self::Output {
        Coord::new_unchecked(self.file() * rhs, self.rank() * rhs)
    }
}

impl From<(i8, i8)> for Coord {
    fn from(value: (i8, i8)) -> Self {
        Coord::new_unchecked(value.0, value.1)
    }
}

impl From<Coord> for (i8, i8) {
    fn from(value: Coord) -> Self {
        (value.file(), value.rank())
    }
}


pub struct CoordIterator {
    curr: i8,
}

impl Iterator for CoordIterator {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;
        if self.curr > 63 || self.curr < 0 {
            return None;
        };
        Some(Coord::from_idx(self.curr))
    }
}


#[cfg(test)]
mod tests {
    use super::Coord;

    #[test]
    fn test_coord() {
        let c = Coord::new(3, 4);
        assert_eq!(c.0, 0b01000011);
        assert_eq!(c.file(), 3);
        assert_eq!(c.rank(), 4);
    }

    #[test]
    fn test_idx() {
        let c = Coord::from_idx(5);
        assert_eq!(c.rank(), 0);
    }

    #[test]
    fn test_is_valid() {
        assert!(Coord::new(7, 7).is_valid());
        assert!(Coord::new(0, 0).is_valid());
        assert!(Coord::new(3, 2).is_valid());
        assert!(Coord::new(7, 0).is_valid());
        assert!(Coord::new(0, 7).is_valid());
    }
}

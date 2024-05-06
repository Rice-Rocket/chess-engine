use crate::{utils::representation::square_name_from_coord, bitboard::bb::BitBoard};
use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy, PartialEq)]
pub struct Coord {
    rank: i8,
    file: i8,
}

impl Coord {
    pub const ROOK_DIRECTIONS: [Self; 4] = [Coord::new_const(-1, 0), Coord::new_const(1, 0), Coord::new_const(0, 1), Coord::new_const(0, -1)];
    pub const BISHOP_DIRECTIONS: [Self; 4] = [Coord::new_const(-1, 1), Coord::new_const(1, 1), Coord::new_const(1, -1), Coord::new_const(-1, -1)];

    pub const A1: Self = Self { file: 0, rank: 0 };
    pub const B1: Self = Self { file: 1, rank: 0 };
    pub const C1: Self = Self { file: 2, rank: 0 };
    pub const D1: Self = Self { file: 3, rank: 0 };
    pub const E1: Self = Self { file: 4, rank: 0 };
    pub const F1: Self = Self { file: 5, rank: 0 };
    pub const G1: Self = Self { file: 6, rank: 0 };
    pub const H1: Self = Self { file: 7, rank: 0 };

    pub const A8: Self = Self { file: 0, rank: 7 };
    pub const B8: Self = Self { file: 1, rank: 7 };
    pub const C8: Self = Self { file: 2, rank: 7 };
    pub const D8: Self = Self { file: 3, rank: 7 };
    pub const E8: Self = Self { file: 4, rank: 7 };
    pub const F8: Self = Self { file: 5, rank: 7 };
    pub const G8: Self = Self { file: 6, rank: 7 };
    pub const H8: Self = Self { file: 7, rank: 7 };

    pub const NULL: Self = Self::A1;

    pub fn new(file: i8, rank: i8) -> Self {
        Self {
            rank, file
        }
    }
    /// Iterates squares from indices [0, 64)
    pub fn iterate_squares() -> CoordIterator {
        CoordIterator { curr: -1 }
    }

    pub const fn new_const(file: i8, rank: i8) -> Self {
        Self {
            rank, file
        }
    }
    pub fn from_idx(idx: i8) -> Self {
        Self {
            rank: (idx >> 3) & 0b000111,
            file: idx & 0b000111,
        }
    }
    pub fn rank(&self) -> i8 {
        self.rank
    }
    pub fn file(&self) -> i8 {
        self.file
    }
    pub fn is_light_square(&self) -> bool {
        (self.file() + self.rank()) % 2 != 0
    }
    pub fn compare_to(&self, other: Self) -> u32 {
        if self.file == other.file && self.rank == other.rank { 0 } else { 1 }
    }
    pub fn is_eq(&self, other: Self) -> bool {
        self.file == other.file && self.rank == other.rank
    }
    /// The index of the coord (usize)
    pub fn index(&self) -> usize {
        (self.rank * 8 + self.file) as usize
    }
    pub const fn const_idx(&self) -> usize {
        (self.rank * 8 + self.file) as usize
    }
    /// The square index of the coord (i8)
    pub fn square(&self) -> i8 {
        self.rank * 8 + self.file
    }
    /// Performs other - self
    pub fn delta(self, other: Coord) -> Coord {
        Coord::new(other.file() - self.file(), other.rank() - self.rank())
    }
    /// Checks if coord is inside bounds
    pub fn is_valid(&self) -> bool {
        self.rank >= 0 && self.file >= 0 && self.rank < 8 && self.file < 8
    }
    /// Converts the coord into a bitboard with only that square
    pub fn to_bitboard(self) -> BitBoard {
        BitBoard(1u64 << self.index())
    }

    pub fn flip_rank(&self) -> Coord {
        Coord::new(self.file, 7 - self.rank)
    }
    pub fn flip_file(&self) -> Coord {
        Coord::new(7 - self.file, self.rank)
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
        Coord::new(self.file + rhs.file, self.rank + rhs.rank)
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
        Coord::new(self.file - rhs.file, self.rank - rhs.rank)
    }
}

impl Sub<i8> for Coord {
    type Output = Coord;
    fn sub(self, rhs: i8) -> Self::Output {
        let rank = rhs >> 3;
        let file = rhs & 0b000111;
        Coord::new(self.file - file, self.rank - rank)
    }
}

impl Mul<i8> for Coord {
    type Output = Coord;
    fn mul(self, rhs: i8) -> Self::Output {
        Coord::new(self.file * rhs, self.rank * rhs)
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

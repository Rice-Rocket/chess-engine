use crate::game::representation::square_name_from_coord;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq)]
pub struct Coord {
    idx: u8,
}

impl Coord {
    pub const A1: Self = Self { idx: 0 };
    pub const B1: Self = Self { idx: 1 };
    pub const C1: Self = Self { idx: 2 };
    pub const D1: Self = Self { idx: 3 };
    pub const E1: Self = Self { idx: 4 };
    pub const F1: Self = Self { idx: 5 };
    pub const G1: Self = Self { idx: 6 };
    pub const H1: Self = Self { idx: 7 };

    pub const A8: Self = Self { idx: 56 };
    pub const B8: Self = Self { idx: 57 };
    pub const C8: Self = Self { idx: 58 };
    pub const D8: Self = Self { idx: 59 };
    pub const E8: Self = Self { idx: 60 };
    pub const F8: Self = Self { idx: 61 };
    pub const G8: Self = Self { idx: 62 };
    pub const H8: Self = Self { idx: 63 };

    pub const NULL: Self = Self::A1;

    pub fn new(file: u8, rank: u8) -> Self {
        Self {
            idx: rank * 8 + file
        }
    }
    pub fn from_idx(idx: u8) -> Self {
        Self {
            idx
        }
    }
    pub fn rank(&self) -> u8 {
        self.idx >> 3
    }
    pub fn file(&self) -> u8 {
        self.idx & 0b000111
    }
    pub fn is_light_square(&self) -> bool {
        return (self.file() + self.rank()) % 2 != 0;
    }
    pub fn compare_to(&self, other: Self) -> u32 {
        return if self.idx == other.idx { 0 } else { 1 };
    }
    pub fn is_eq(&self, other: Self) -> bool {
        return if self.idx == other.idx { true } else { false };
    }
    pub fn index(&self) -> usize {
        self.idx as usize
    }
    pub fn square(&self) -> u8 {
        self.idx
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
        Coord::from_idx(self.idx + rhs.idx)
    }
}

impl Add<u8> for Coord {
    type Output = Coord;
    fn add(self, rhs: u8) -> Self::Output {
        Coord::from_idx(self.idx + rhs)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord::from_idx(self.idx - rhs.idx)
    }
}

impl Sub<u8> for Coord {
    type Output = Coord;
    fn sub(self, rhs: u8) -> Self::Output {
        Coord::from_idx(self.idx - rhs)
    }
}
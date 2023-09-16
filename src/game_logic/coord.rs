use crate::game_logic::representation::square_name_from_coord;

#[derive(Clone, Copy, PartialEq)]
pub struct Coord {
    pub file_idx: u32,
    pub rank_idx: u32,
}

impl Coord {
    pub fn new(file_idx: u32, rank_idx: u32) -> Self {
        Self {
            file_idx, rank_idx
        }
    }
    pub fn is_light_square(&self) -> bool {
        return (self.file_idx + self.rank_idx) % 2 != 0;
    }
    pub fn compare_to(&self, other: Self) -> u32 {
        return if self.file_idx == other.file_idx && self.rank_idx == other.rank_idx { 0 } else { 1 };
    }
    pub fn is_eq(&self, other: Self) -> bool {
        return if self.file_idx == other.file_idx && self.rank_idx == other.rank_idx { true } else { false };
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", square_name_from_coord(self.file_idx, self.rank_idx))
    }
}
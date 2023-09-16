use super::coord;

pub const FILE_NAMES: &str = "abcdefgh";
pub const RANK_NAMES: &str = "12345678";

pub const A1: u32 = 0;
pub const B1: u32 = 1;
pub const C1: u32 = 2;
pub const D1: u32 = 3;
pub const E1: u32 = 4;
pub const F1: u32 = 5;
pub const G1: u32 = 6;
pub const H1: u32 = 7;

pub const A8: u32 = 56;
pub const B8: u32 = 57;
pub const C8: u32 = 58;
pub const D8: u32 = 59;
pub const E8: u32 = 60;
pub const F8: u32 = 61;
pub const G8: u32 = 62;
pub const H8: u32 = 63;

pub fn rank_idx(square: u32) -> u32 {
    square >> 3
}

pub fn file_idx(square: u32) -> u32 {
    square & 0b000111
}

pub fn idx_from_coord(file_idx: u32, rank_idx: u32) -> u32 {
    rank_idx * 8 + file_idx
}

pub fn coord_from_idx(square: u32) -> coord::Coord {
    coord::Coord::new(file_idx(square), rank_idx(square))
}

pub fn light_square(file_idx: u32, rank_idx: u32) -> bool {
    (file_idx + rank_idx) % 2 != 0
}

pub fn square_name_from_coord(file_idx: u32, rank_idx: u32) -> String {
    FILE_NAMES.chars().nth(file_idx as usize).unwrap().to_string() + &(rank_idx + 1).to_string()
}

pub fn square_name_from_idx(square: u32) -> String {
    let coordinate = coord_from_idx(square);
    square_name_from_coord(coordinate.file_idx, coordinate.rank_idx)
}
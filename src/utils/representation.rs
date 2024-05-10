use crate::board::coord::Coord;

pub const FILE_NAMES: &str = "abcdefgh";
pub const RANK_NAMES: &str = "12345678";

pub fn light_square(file_idx: u32, rank_idx: u32) -> bool {
    (file_idx + rank_idx) % 2 != 0
}

pub fn square_name_from_coord(file_idx: i8, rank_idx: i8) -> String {
    FILE_NAMES.chars().nth(file_idx as usize).unwrap().to_string() + &(rank_idx + 1).to_string()
}

pub fn square_name_from_idx(square: i8) -> String {
    let coordinate = Coord::from_idx(square);
    square_name_from_coord(coordinate.file(), coordinate.rank())
}

pub fn coord_from_name(name: &str) -> Option<Coord> {
    Some(Coord::new(
        FILE_NAMES.chars().position(|c| c == if let Some(v) = name.chars().next() { v } else { ' ' })? as i8,
        RANK_NAMES.chars().position(|c| c == if let Some(v) = name.chars().nth(1) { v } else { ' ' })? as i8,
    ))
}

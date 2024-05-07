use crate::board::coord::Coord;
use super::state::State;

pub fn pinned_direction(state: &State, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of knights attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn knight_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of bishops attacking `sqr`, including xray attacks through queens. 
/// If s2 specified, only counts attacks coming from that square.  
pub fn bishop_xray_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of rooks attacking `sqr`, including xray attacks through queens. 
/// If s2 specified, only counts attacks coming from that square.  
pub fn rook_xray_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of queens attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn queen_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of pawns attacking `sqr`, excluding pins and en-passant. 
/// If s2 specified, only counts attacks coming from that square. 
pub fn pawn_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of kings attacking `sqr`. If s2 specified, only counts attacks coming
/// from that square. 
pub fn king_attack(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of attacks on `sqr` by all pieces.
pub fn attack(state: &State, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates whether or not `sqr` is pinned.
pub fn pinned(state: &State, sqr: Coord) -> i32 {
    todo!();
}

/// Calculates the number of queens attacking `sqr` diagonally. If s2 specified, only counts
/// attacks coming from that square. 
pub fn queen_attack_diagonal(state: &State, s2: Option<Coord>, sqr: Coord) -> i32 {
    todo!();
}

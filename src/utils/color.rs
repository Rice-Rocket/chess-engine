use bevy::prelude::*;
use std::ops::Index;


#[derive(PartialEq)]
pub enum Team {
    White,
    Black,
}

// For indexing
impl From<Team> for usize {
    fn from(value: Team) -> Self {
        if value == Team::White { 0 } else { 1 }
    }
}

// For pieces
impl From<Team> for u8 {
    fn from(value: Team) -> Self {
        if value == Team::White { 8 } else { 16 }
    }
}
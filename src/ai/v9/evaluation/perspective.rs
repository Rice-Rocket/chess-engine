use crate::board::{piece::Piece, board::Board};

#[derive(PartialEq, Clone, Copy)]
pub enum Perspective {
    White,
    Black,
}

impl Perspective {
    pub fn is_white(&self) -> bool {
        match self {
            Self::White => true,
            Self::Black => false,
        }
    }
    pub fn is_color(&self, color: u8) -> bool {
        match self {
            Self::White => color == Piece::WHITE,
            Self::Black => color == Piece::BLACK,
        }
    }
    pub fn color(&self) -> u8 {
        match self {
            Self::White => Piece::WHITE,
            Self::Black => Piece::BLACK,
        }
    }
    pub fn color_idx(&self) -> usize {
        match self {
            Self::White => Board::WHITE_INDEX,
            Self::Black => Board::BLACK_INDEX,
        }
    }
    pub fn other(&self) -> Perspective {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn friendly_piece(&self, ptype: u8) -> Piece {
        Piece::new(ptype | self.color())
    }
    pub fn enemy_piece(&self, ptype: u8) -> Piece {
        Piece::new(ptype | self.other().color())
    }

    pub fn iter_ranks_forward(&self) -> RanksIterator {
        match self {
            Self::White => RanksIterator { curr: -1, rev: false },
            Self::Black => RanksIterator { curr: 8, rev: true },
        }
    }
    pub fn iter_ranks_backward(&self) -> RanksIterator {
        match self {
            Self::White => RanksIterator { curr: 8, rev: true },
            Self::Black => RanksIterator { curr: -1, rev: false },
        }
    }
    pub fn iter_ranks_forward_from_incl(&self, start_rank: i8) -> RanksIterator {
        match self {
            Self::White => RanksIterator { curr: start_rank - 1, rev: false },
            Self::Black => RanksIterator { curr: start_rank + 1, rev: true },
        }
    }
    pub fn iter_ranks_backward_from_incl(&self, start_rank: i8) -> RanksIterator {
        match self {
            Self::White => RanksIterator { curr: start_rank + 1, rev: true },
            Self::Black => RanksIterator { curr: start_rank - 1, rev: false },
        }
    }
    pub fn iter_ranks_forward_from_excl(&self, start_rank: i8) -> RanksIterator {
        match self {
            Self::White => RanksIterator { curr: start_rank, rev: false },
            Self::Black => RanksIterator { curr: start_rank, rev: true },
        }
    }
    pub fn iter_ranks_backward_from_excl(&self, start_rank: i8) -> RanksIterator {
        match self {
            Self::White => RanksIterator { curr: start_rank, rev: true },
            Self::Black => RanksIterator { curr: start_rank, rev: false },
        }
    }

    pub fn home_rank(&self) -> i8 {
        match self {
            Self::White => 0,
            Self::Black => 7,
        }
    }
    pub fn enemy_rank(&self) -> i8 {
        7 - self.home_rank()
    }
    pub fn home_outbounds_rank(&self) -> i8 {
        match self {
            Self::White => -1,
            Self::Black => 8,
        }
    }
    pub fn enemy_outbounds_rank(&self) -> i8 {
        7 - self.home_outbounds_rank()
    }

    // Checks if rank a is closer to the home side than rank b
    pub fn rank_is_closer(&self, a: i8, b: i8) -> bool {
        match self {
            Self::White => { a < b },
            Self::Black => { a > b },
        }
    }
    // Checks if rank a is farther from the home side than rank b
    pub fn rank_is_farther(&self, a: i8, b: i8) -> bool {
        match self {
            Self::White => { a > b },
            Self::Black => { a < b },
        }
    }
    // Checks if rank a is closer to the home side or equal to rank b
    pub fn rank_is_closer_or_eq(&self, a: i8, b: i8) -> bool {
        match self {
            Self::White => { a <= b },
            Self::Black => { a >= b },
        }
    }
    // Checks if rank a is farther from the home side or equal to rank b
    pub fn rank_is_farther_or_eq(&self, a: i8, b: i8) -> bool {
        match self {
            Self::White => { a >= b },
            Self::Black => { a <= b },
        }
    }
    // Produces a rank that is n closer to the home side
    pub fn rank_closer_by(&self, rank: i8, n: i8) -> i8 {
        match self {
            Self::White => { rank - n },
            Self::Black => { rank + n },
        }
    }
    // Produces a rank that is n farther to the home side
    pub fn rank_farther_by(&self, rank: i8, n: i8) -> i8 {
        match self {
            Self::White => { rank + n },
            Self::Black => { rank - n },
        }
    }
    // Determines if the rank is on the close half of the board
    pub fn rank_is_close_half(&self, rank: i8) -> bool {
        match self {
            Self::White => { rank < 4 },
            Self::Black => { rank >= 4 },
        }
    }
    // Determines if the rank is on the far half of the board
    pub fn rank_is_far_half(&self, rank: i8) -> bool {
        match self {
            Self::White => { rank >= 4 },
            Self::Black => { rank < 4 },
        }
    }
}

pub struct RanksIterator {
    curr: i8,
    rev: bool,
}

impl Iterator for RanksIterator {
    type Item = i8;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += if self.rev { -1 } else { 1 };
        if self.curr > 7 || self.curr < 0 { return None; }
        return Some(self.curr);
    }
}
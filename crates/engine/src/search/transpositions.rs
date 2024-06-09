use std::ops::Index;

use crate::board::moves::Move;

use super::Searcher;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TranspositionNodeType {
    #[default]
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Default, Clone, Copy)]
pub struct TranspositionEntry {
    pub key: u64,
    pub eval: i32,
    pub m: Move,
    pub depth: u8,
    pub node_type: TranspositionNodeType,
}

impl TranspositionEntry {
    pub fn new(key: u64, eval: i32, m: Move, depth: u8, node_type: TranspositionNodeType) -> Self {
        Self {
            key,
            eval,
            m,
            depth,
            node_type,
        }
    }

    pub const fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}


pub struct TranspositionTable<const N: usize> {
    entries: Vec<TranspositionEntry>,
    enabled: bool,
}

impl<const N: usize> TranspositionTable<N> {
    pub fn new() -> Self {
        Self {
            entries: vec![TranspositionEntry::default(); N],
            enabled: true,
        }
    }

    pub fn clear(&mut self) {
        self.entries.iter_mut().for_each(|e| *e = TranspositionEntry::default());
    }

    pub fn index(&self, key: u64) -> u64 {
        key % N as u64
    }

    pub fn get_stored_move(&self, key: u64) -> Move {
        self.entries[self.index(key) as usize].m
    }

    pub fn lookup(&self, key: u64, depth: u8, dst_from_root: u8, alpha: i32, beta: i32) -> Option<i32> {
        if !self.enabled {
            return None;
        }

        let entry = self.entries[self.index(key) as usize];

        if entry.key == key && entry.depth >= depth {
            let score = Self::correct_retrieved_mate_score(entry.eval, dst_from_root);

            if entry.node_type == TranspositionNodeType::Exact {
                return Some(score);
            }

            if entry.node_type == TranspositionNodeType::UpperBound && score <= alpha {
                return Some(score);
            }
            
            if entry.node_type == TranspositionNodeType::LowerBound && score >= beta {
                return Some(score);
            }
        }

        None
    }

    pub fn store(&mut self, key: u64, depth: u8, dst_from_root: u8, eval: i32, eval_type: TranspositionNodeType, m: Move) {
        if !self.enabled {
            return;
        }

        let index = self.index(key) as usize;
        let entry = TranspositionEntry::new(key, Self::correct_store_mate_score(eval, dst_from_root), m, depth, eval_type);
        self.entries[index] = entry;
    }

    pub fn correct_store_mate_score(score: i32, num_positions_searched: u8) -> i32 {
        if score.abs() > Searcher::IMMEDIATE_MATE_SCORE - 1000 {
            let sign = score.signum();
            (score * sign + num_positions_searched as i32) * sign
        } else {
            score
        }
    }

    pub fn correct_retrieved_mate_score(score: i32, num_positions_searched: u8) -> i32 {
        if score.abs() > Searcher::IMMEDIATE_MATE_SCORE - 1000 {
            let sign = score.signum();
            (score * sign - num_positions_searched as i32) * sign
        } else {
            score
        }
    }

    pub fn get(&self, key: u64) -> TranspositionEntry {
        self.entries[(key % N as u64) as usize]
    }
}

impl<const N: usize> Default for TranspositionTable<N> {
    fn default() -> Self {
        Self::new()
    }
}

use crate::board::{moves::Move, board::Board};
use super::searcher::Searcher;

#[derive(PartialEq, Clone)]
pub enum EvaluationType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone)]
pub struct TranspositionEntry {
    pub key: u64,
    pub value: i32,
    pub mov: Move,
    pub depth: u8,
    pub node_type: EvaluationType,
}

pub struct TranspositionTable {
    pub entries: Vec<Option<TranspositionEntry>>,
    pub count: u64,
    pub enabled: bool,
}

impl TranspositionTable {
    const TT_ENTRY_SIZE_BYTES: usize = 16;

    pub fn new(size_mb: usize) -> Self {
        let desired_table_size_bytes = size_mb * 1024 * 1024;
        let n_entries = desired_table_size_bytes / Self::TT_ENTRY_SIZE_BYTES;

        Self {
            entries: vec![None; n_entries],
            count: n_entries as u64,
            enabled: true,
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn index(&self, board: &Board) -> usize {
        (board.current_state.zobrist_key % self.count) as usize
    }
    pub fn get_stored_move(&self, board: &Board) -> Option<Move> {
        if let Some(entry) = &self.entries[self.index(board)] {
            Some(entry.mov)
        } else {
            None
        }
    }
    pub fn get_evaluation(&self, depth_remaining: u8, current_depth: u8, alpha: i32, beta: i32, board: &Board) -> Option<i32> {
        if !self.enabled {
            return None;
        }
        if let Some(entry) = &self.entries[self.index(board)] {
            if entry.key == board.current_state.zobrist_key {
                if entry.depth >= depth_remaining {
                    let corrected_score = Self::correct_retrieved_mate_score(entry.value, current_depth);
                    if entry.node_type == EvaluationType::Exact {
                        return Some(corrected_score);
                    }
                    if entry.node_type == EvaluationType::UpperBound && corrected_score <= alpha {
                        return Some(corrected_score);
                    }
                    if entry.node_type == EvaluationType::LowerBound && corrected_score >= beta {
                        return Some(corrected_score);
                    }
                }
            }
        }
        None
    }
    pub fn store_evaluation(&mut self, depth_remaining: u8, current_depth: u8, eval: i32, eval_type: EvaluationType, mov: Move, board: &Board) {
        if !self.enabled {
            return;
        }
        let index = self.index(board);
        let entry = TranspositionEntry {
            key: board.current_state.zobrist_key,
            value: Self::correct_stored_mate_score(eval, current_depth),
            mov,
            depth: depth_remaining,
            node_type: eval_type,
        };
        self.entries[index] = Some(entry);
    }
    fn correct_stored_mate_score(score: i32, current_depth: u8) -> i32 {
        if Searcher::is_mate_score(score) {
            let sign = score.signum();
            return (score * sign + current_depth as i32) * sign;
        }
        return score;
    }
    fn correct_retrieved_mate_score(score: i32, current_depth: u8) -> i32 {
        if Searcher::is_mate_score(score) {
            let sign = score.signum();
            return (score * sign - current_depth as i32) * sign;
        }
        return score;
    }

    pub fn get_entry(&self, key: u64) -> Option<TranspositionEntry> {
        return self.entries[(key % self.entries.len() as u64) as usize].clone();
    }
}
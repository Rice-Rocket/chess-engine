use crate::board::Board;

pub struct RepetitionTable {
    hashes: [u64; 256],
    start_indices: [u32; 257],
    count: usize,
}

impl RepetitionTable {
    pub fn new(board: &Board) -> Self {
        let init_hashes = board.repeat_position_history.clone().into_iter().rev();
        let count = init_hashes.len();

        let mut hashes = [0; 256];
        for (hash, init) in hashes.iter_mut().zip(init_hashes) {
            *hash = init;
        }

        RepetitionTable {
            hashes,
            start_indices: [0; 257],
            count,
        }
    }

    pub fn push(&mut self, hash: u64, reset: bool) {
        if self.count <= self.hashes.len() {
            self.hashes[self.count] = hash;
            self.start_indices[self.count + 1] = if reset { self.count as u32 } else { self.start_indices[self.count] };
        }
        self.count += 1;
    }

    pub fn pop(&mut self) {
        self.count = self.count.max(1) - 1;
    }

    pub fn contains(&self, hash: u64) -> bool {
        let s = self.start_indices[self.count] as usize;

        for h in self.hashes.iter().skip(s).take(self.count - 1) {
            if *h == hash {
                return true;
            }
        }

        false
    }
}

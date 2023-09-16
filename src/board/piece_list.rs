#[derive(Clone)]
pub struct PieceList {
    pub occupied_squares: Vec<u32>,
    map: Vec<u32>,
    n_pieces: u32,
}

impl PieceList {
    pub fn new(max_piece_count: u32) -> Self {
        Self {
            occupied_squares: vec![0; max_piece_count as usize],
            map: vec![0; 64],
            n_pieces: 0
        }
    }
    pub fn count(&self) -> u32 {
        return self.n_pieces;
    }
    pub fn add_piece(&mut self, square: u32) {
        self.occupied_squares[self.n_pieces as usize] = square;
        self.map[square as usize] = self.n_pieces;
        self.n_pieces += 1;
    }
    pub fn remove_piece(&mut self, square: u32) {
        let piece_idx = self.map[square as usize];
        self.occupied_squares[piece_idx as usize] = self.occupied_squares[(self.n_pieces - 1) as usize];
        self.map[self.occupied_squares[piece_idx as usize] as usize] = piece_idx;
        self.n_pieces -= 1;
    }
    pub fn move_piece(&mut self, start: u32, target: u32) {
        let piece_idx = self.map[start as usize];
        self.occupied_squares[piece_idx as usize] = target;
        self.map[target as usize] = piece_idx;
    }
    pub fn index(&self, idx: u32) -> u32 {
        self.occupied_squares[idx as usize]
    }
}


impl std::ops::Index<usize> for PieceList {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.occupied_squares[index]
    }
}
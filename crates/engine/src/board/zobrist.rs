use rand::{Rng, SeedableRng, rngs::StdRng};
use super::Board;
use super::piece::*;

const SEED: u64 = 29426028;

pub struct Zobrist {
    pub pieces_array: [[u64; (Piece::MAX_PIECE_INDEX + 1) as usize]; 64], // index [square][piece]
    pub castling_rights: [u64; 16],
    pub en_passant_file: [u64; 9],
    pub side_to_move: u64,
    prng: StdRng,
}

impl Zobrist {
    pub fn new() -> Self {
        let mut zobrist = Self {
            pieces_array: [[0; (Piece::MAX_PIECE_INDEX + 1) as usize]; 64],
            castling_rights: [0; 16],
            en_passant_file: [0; 9],
            side_to_move: 0,
            prng: SeedableRng::seed_from_u64(SEED),
        };

        for square_idx in 0..64 {
            for piece in Piece::PIECE_INDICES {
                zobrist.pieces_array[square_idx][piece as usize] = zobrist.prng.gen_range(u64::MIN..=u64::MAX);
                zobrist.pieces_array[square_idx][piece as usize] = zobrist.prng.gen_range(u64::MIN..=u64::MAX);
            }
        }
        for i in 0..zobrist.castling_rights.len() {
            zobrist.castling_rights[i] = zobrist.prng.gen_range(u64::MIN..=u64::MAX);
        }      
        for i in 0..zobrist.en_passant_file.len() {
            zobrist.en_passant_file[i] = zobrist.prng.gen_range(u64::MIN..=u64::MAX);
        }  
        zobrist.side_to_move = zobrist.prng.gen_range(u64::MIN..=u64::MAX);

        zobrist
    }

    pub fn calc_zobrist_key(&mut self, board: &Board) -> u64 {
        let mut zobrist_key: u64 = 0;
        
        for sqr_idx in 0..64 {
            let piece = board.square[sqr_idx as usize];
            if piece != Piece::NULL {
                zobrist_key ^= self.pieces_array[sqr_idx as usize][piece.value() as usize];
            }
        }

        zobrist_key ^= self.en_passant_file[board.current_state.en_passant_file as usize];
        if board.move_color == Piece::BLACK {
            zobrist_key ^= self.side_to_move;
        }
        zobrist_key ^= self.castling_rights[board.current_state.castling_rights as usize];

        zobrist_key
    }
}

impl Default for Zobrist {
    fn default() -> Self {
        Zobrist::new()
    }
}

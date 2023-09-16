use rand::{Rng, SeedableRng, rngs::StdRng};
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::VecDeque;
use super::board::Board;
use super::piece::*;

const SEED: u64 = 2361912;
const RAND_NUMS_FILENAME: &str = "assets/logic/random_numbers.txt";

#[derive(Clone)]
pub struct Zobrist {
    pub pieces_array: [[[u64; 8]; 2]; 64],
    pub castling_rights: [u64; 16],
    pub en_passant_file: [u64; 9],
    pub side_to_move: u64,
    prng: StdRng,
}

impl Zobrist {
    pub fn write_rand_numbers(&mut self) {
        self.prng = SeedableRng::seed_from_u64(SEED);
        let mut rand_num_str = "".to_string();
        let n_rand_nums: i32 = 64 * 8 * 2 + self.castling_rights.len() as i32 + 9 + 1;
        
        for i in 0..n_rand_nums {
            rand_num_str += &self.prng.gen_range(u64::MIN..u64::MAX).to_string();
            if i != n_rand_nums - 1 {
                rand_num_str += ",";
            }
        }

        let mut filepath = File::create(RAND_NUMS_FILENAME).unwrap();
        filepath.write_all(rand_num_str.as_bytes()).unwrap();
    }
    pub fn read_rand_numbers(&mut self) -> VecDeque<u64> {
        if !Path::new(&RAND_NUMS_FILENAME).exists() {
            println!("Writing zobrist random numbers to {}", RAND_NUMS_FILENAME);
            self.write_rand_numbers();
        }

        let mut rand_numbers: VecDeque<u64> = VecDeque::new();
        let file = File::open(&RAND_NUMS_FILENAME).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut nums_string = String::new();
        buf_reader.read_to_string(&mut nums_string).unwrap();

        for n in nums_string.split(",") {
            let num = n.parse::<u64>().unwrap();
            rand_numbers.push_back(num);
        }

        return rand_numbers;
    }
    pub fn new() -> Self {
        let mut default = Self {
            pieces_array: [[[0; 8]; 2]; 64],
            castling_rights: [0; 16],
            en_passant_file: [0; 9],
            side_to_move: 0,
            prng: SeedableRng::seed_from_u64(SEED),
        };
        let mut rand_nums = default.read_rand_numbers();

        for square_idx in 0..64 {
            for piece_idx in 0..8 {
                default.pieces_array[square_idx][Board::WHITE_INDEX][piece_idx] = rand_nums.pop_front().unwrap();
                default.pieces_array[square_idx][Board::BLACK_INDEX][piece_idx] = rand_nums.pop_front().unwrap();
            }
        }
        for i in 0..16 {
            default.castling_rights[i] = rand_nums.pop_front().unwrap();
        }      
        for i in 0..default.en_passant_file.len() {
            default.en_passant_file[i] = rand_nums.pop_front().unwrap();
        }  
        default.side_to_move = rand_nums.pop_front().unwrap();

        return default;
    }
    pub fn calc_zobrist_key(&mut self, board: Board) -> u64 {
        let mut zobrist_key: u64 = 0;
        
        for sqr_idx in 0..64 {
            if board.square[sqr_idx as usize] != Piece::NULL {
                let ptype = board.square[sqr_idx as usize].piece_type();
                let pcolor = board.square[sqr_idx as usize].color();
                zobrist_key ^= self.pieces_array[sqr_idx as usize][if pcolor == Piece::WHITE { Board::WHITE_INDEX } else { Board::BLACK_INDEX }][ptype as usize];
            }
        }

        let ep_idx = ((board.current_game_state >> 4) & 15) as i32;
        if ep_idx != -1 {
            zobrist_key ^= self.en_passant_file[ep_idx as usize];
        }
        if board.color_to_move == Piece::BLACK {
            zobrist_key ^= self.side_to_move;
        }
        zobrist_key ^= self.castling_rights[(board.current_game_state & 0b1111) as usize];

        return zobrist_key;
    }
}
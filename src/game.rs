use std::collections::VecDeque;

use crate::{bitboard::{bbutils::BitBoardUtils, precomp_bits::PrecomputedBits}, board::{coord::Coord, moves::Move, zobrist::Zobrist, Board}, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData}};

pub struct Game {
    pub board: Board,
    pub precomp: PrecomputedMoveData,
    pub bbutils: BitBoardUtils,
    pub zobrist: Zobrist,
    pub magics: MagicBitBoards,
    pub precomp_bits: PrecomputedBits,
    pub movegen: MoveGenerator,

    history: VecDeque<Move>,
}

impl Game {
    pub fn new(start_fen: Option<String>) -> Self {
        let precomp = PrecomputedMoveData::new();
        let bbutils = BitBoardUtils::new();
        let mut zobrist = Zobrist::new();
        let board = Board::load_position(start_fen, &mut zobrist);
        let magics = MagicBitBoards::default();
        let precomp_bits = PrecomputedBits::new(&bbutils);
        let movegen = MoveGenerator::default();

        Self {
            board,
            precomp,
            bbutils,
            zobrist,
            magics,
            precomp_bits,
            movegen,

            history: VecDeque::new(),
        }
    }

    pub fn make_move(&mut self, m: Move) {
        self.board.make_move(m, false, &self.zobrist);
        self.history.push_back(m);
    }

    pub fn undo_move(&mut self) {
        let Some(m) = self.history.pop_back() else { return };
        self.board.unmake_move(m, false);
    }

    pub fn valid_moves(&mut self, sqr: Coord) -> Vec<Move> {
        self.movegen.generate_moves(&self.board, &self.precomp, &self.bbutils, &self.magics, false);
        self.movegen.moves.iter().cloned().filter(|m| m.start() == sqr).collect()
    }
}

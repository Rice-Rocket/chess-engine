use std::collections::VecDeque;

use crate::{board::{coord::Coord, moves::Move, zobrist::Zobrist, Board}, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed};

pub struct Game {
    pub board: Board,
    pub precomp: Precomputed,
    pub zobrist: Zobrist,
    pub magics: MagicBitBoards,
    pub movegen: MoveGenerator,
}

impl Game {
    pub fn new(start_fen: Option<String>) -> Self {
        let precomp = Precomputed::new();
        let mut zobrist = Zobrist::new();
        let board = Board::load_position(start_fen, &mut zobrist);
        let magics = MagicBitBoards::default();
        let movegen = MoveGenerator::default();

        Self {
            board,
            precomp,
            zobrist,
            magics,
            movegen,
        }
    }

    pub fn make_move(&mut self, m: Move) {
        self.board.make_move(m, false, &self.zobrist);
    }

    pub fn undo_move(&mut self) {
        let Some(m) = self.board.move_log.last() else { return };
        self.board.unmake_move(*m, false);
    }

    pub fn valid_moves(&mut self, sqr: Coord) -> Vec<Move> {
        self.movegen.generate_moves(&self.board, &self.precomp, &self.magics, false);
        self.movegen.moves.iter().cloned().filter(|m| m.start() == sqr).collect()
    }
}

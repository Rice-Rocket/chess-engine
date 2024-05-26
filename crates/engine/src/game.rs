use std::collections::VecDeque;

use crate::{board::{coord::Coord, moves::Move, piece::Piece, zobrist::Zobrist, Board}, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed, prelude::BitBoard, result::GameResult, search::{options::SearchOptions, Searcher}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    Human,
    Computer,
}

pub struct Game<'a> {
    pub board: Board,
    pub precomp: Precomputed,
    pub zobrist: Zobrist,
    pub magics: MagicBitBoards,
    pub movegen: MoveGenerator,

    pub white: PlayerType,
    pub black: PlayerType,
    pub searcher: Searcher<'a>,
    pub search_opts: SearchOptions,
    pub player_to_move: PlayerType,
}

impl<'a> Game<'a> {
    pub fn new(start_fen: Option<String>, search_opts: SearchOptions, white: PlayerType, black: PlayerType) -> Self {
        let precomp = Precomputed::new();
        let mut zobrist = Zobrist::new();
        let board = Board::load_position(start_fen, &mut zobrist);
        let magics = MagicBitBoards::default();
        let mut movegen = MoveGenerator::default();
        let mut searcher = Searcher::new();

        movegen.generate_moves(&board, &precomp, &magics, false);
        let player_to_move = if board.white_to_move { white } else { black };

        if player_to_move == PlayerType::Computer {
            searcher.begin_search(search_opts, board.clone(), &precomp, &magics, &zobrist, movegen.clone());
        }

        Self {
            board,
            precomp,
            zobrist,
            magics,
            movegen,
            white,
            black,
            searcher,
            search_opts,
            player_to_move,
        }
    }

    pub fn make_move(&mut self, m: Move) -> GameResult {
        self.board.make_move(m, false, &self.zobrist);
        let result = self.get_game_result();

        if result.is_terminal() {
            return result;
        }

        self.player_to_move = if self.board.white_to_move { self.white } else { self.black };
        match self.player_to_move {
            PlayerType::Human => {

            },
            PlayerType::Computer => {
                self.searcher.begin_search(self.search_opts, self.board.clone(), &self.precomp, &self.magics, &self.zobrist, self.movegen.clone());
            },
        }

        result
    }

    pub fn try_make_computer_move(&mut self) -> Option<GameResult> {
        self.searcher.best_move().map(|m| self.make_move(m))
    }

    pub fn get_game_result(&mut self) -> GameResult {
        self.movegen.generate_moves(&self.board, &self.precomp, &self.magics, false);
        let moves = &self.movegen.moves;

        if moves.is_empty() {
            if self.movegen.in_check() {
                return if self.board.white_to_move { GameResult::WhiteMated } else { GameResult::BlackMated };
            }
            return GameResult::Stalemate;
        }

        if self.board.current_state.fifty_move_counter >= 100 {
            return GameResult::FiftyMoveRule;
        }

        let rep_count = self.board.repeat_position_history.iter().filter(|x| **x == self.board.current_state.zobrist_key).count();
        if rep_count == 3 {
            return GameResult::Repetition;
        }

        if self.insufficient_material() {
            return GameResult::InsufficientMaterial;
        }

        GameResult::InProgress
    }

    fn insufficient_material(&self) -> bool {
        if self.board.piece_bitboards[Piece::new(Piece::WHITE_PAWN)].count() > 0
        || self.board.piece_bitboards[Piece::new(Piece::BLACK_PAWN)].count() > 0 {
            return false;
        }

        if self.board.friendly_orthogonal_sliders.0 != 0 || self.board.enemy_orthogonal_sliders.0 != 0 {
            return false;
        }

        let n_white_bishops = self.board.piece_bitboards[Piece::new(Piece::WHITE_BISHOP)].count();
        let n_black_bishops = self.board.piece_bitboards[Piece::new(Piece::BLACK_BISHOP)].count();
        let n_white_knights = self.board.piece_bitboards[Piece::new(Piece::WHITE_KNIGHT)].count();
        let n_black_knights = self.board.piece_bitboards[Piece::new(Piece::BLACK_KNIGHT)].count();
        let n_white_minors = n_white_bishops + n_white_knights;
        let n_black_minors = n_black_bishops + n_black_knights;
        let n_minors = n_white_minors + n_black_minors;

        if n_minors <= 1 {
            return true;
        }

        if n_minors == 2 && n_white_bishops == 1 && n_black_bishops == 1 {
            let white_bishop_light = (BitBoard::LIGHT_SQUARES & self.board.piece_bitboards[Piece::new(Piece::WHITE_BISHOP)]).0 != 0;
            let black_bishop_light = (BitBoard::LIGHT_SQUARES & self.board.piece_bitboards[Piece::new(Piece::BLACK_BISHOP)]).0 != 0;
            return white_bishop_light == black_bishop_light;
        }

        false
    }

    pub fn undo_move(&mut self) {
        let Some(m) = self.board.move_log.last() else { return };
        self.board.unmake_move(*m, false);
    }

    pub fn valid_human_moves(&mut self, sqr: Coord) -> Vec<Move> {
        if !self.searcher.in_search {
            self.movegen.moves.iter().cloned().filter(|m| m.start() == sqr).collect()
        } else {
            Vec::new()
        }
    }
}

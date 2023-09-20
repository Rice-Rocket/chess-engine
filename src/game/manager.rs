use std::time::SystemTime;

use bevy::prelude::*;

use crate::{board::{moves::Move, board::Board, zobrist::Zobrist, piece::Piece}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::utils::BitBoardUtils, magics::MagicBitBoards}, ui::ingame_menu::CalcStatistics, state::{AppState, AppMode}};

#[derive(Clone, Copy)]
pub enum GameResult {
    None,
    Playing,
    WhiteIsMated,
    BlackIsMated,
    Stalemate,
    Repetition,
    FiftyMoveRule,
    InsufficientMaterial,
    DrawByArbiter,
    WhiteTimeout,
    BlackTimeout
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerType {
    Human, AI
}

#[derive(Resource)]
pub struct GameManager {
    pub custom_position: Option<String>,
    pub white_player_type: PlayerType,
    pub black_player_type: PlayerType,
    pub game_result: GameResult,
    pub game_moves: Vec<Move>,
    pub move_color: u8,
    executed_board_move: Option<Move>,
}

impl GameManager {
    // Regenerates and returns game result given board and legal moves
    pub fn gen_game_result(&mut self, board: &Board, moves: &Vec<Move>, in_check: bool) -> GameResult {
        if moves.len() == 0 {
            if in_check {
                self.game_result = if board.white_to_move { GameResult::WhiteIsMated } else { GameResult::BlackIsMated };
                return self.game_result;
            }
            self.game_result = GameResult::Stalemate;
            return self.game_result;
        }
        if board.current_state.fifty_move_counter >= 100 {
            self.game_result = GameResult::FiftyMoveRule;
            return self.game_result;
        }
        let mut rep_count = 0;
        for position in board.repeat_position_history.iter() {
            if *position == board.current_state.zobrist_key {
                rep_count += 1;
            };
            if rep_count >= 3 {
                self.game_result = GameResult::Repetition;
                return self.game_result;
            };
        };
        if self.insufficient_material(board) {
            self.game_result = GameResult::InsufficientMaterial;
            return self.game_result;
        }
        self.game_result = GameResult::Playing;
        return self.game_result;
    }
    fn insufficient_material(&self, board: &Board) -> bool {
        if board.friendly_orthogonal_sliders != 0 || board.enemy_orthogonal_sliders != 0 {
            return false;
        };
        if board.get_piece_list(Piece::PAWN, Board::WHITE_INDEX).count() > 0 || 
            board.get_piece_list(Piece::PAWN, Board::BLACK_INDEX).count() > 0 {
                return false;
        };

        let n_white_bishops = board.get_piece_list(Piece::BISHOP, Board::WHITE_INDEX).count();
        let n_black_bishops = board.get_piece_list(Piece::BISHOP, Board::BLACK_INDEX).count();
        let n_white_knights = board.get_piece_list(Piece::KNIGHT, Board::WHITE_INDEX).count();
        let n_black_knights = board.get_piece_list(Piece::KNIGHT, Board::BLACK_INDEX).count();
        let n_white_minors = n_white_bishops + n_white_knights;
        let n_black_minors = n_black_bishops + n_black_knights;

        if n_white_minors == 0 && n_black_minors == 0 {
            return true;
        };
        if (n_white_minors == 1 && n_black_minors == 0) || (n_white_minors == 0 && n_black_minors == 1) {
            return true;
        };
        return false;
    }
}

#[derive(Event)]
pub struct BoardMakeMove {
    pub mov: Move
}

#[derive(Event)]
pub struct ProcessedMove {}

#[derive(Event)]
pub struct CanMakeMove {}

pub fn initialize_game(
    mut move_gen: ResMut<MoveGenerator>,
    board: Res<Board>,
    precomp: Res<PrecomputedMoveData>,
    bbutils: Res<BitBoardUtils>,
    magic: Res<MagicBitBoards>,
) {
    move_gen.generate_moves(&board, &precomp, &bbutils, &magic, false);
}

pub fn spawn_game_manager(
    mut commands: Commands,
    app_mode: Res<State<AppMode>>,
) {
    let (white, black) = match app_mode.clone() {
        AppMode::GameHumanHuman => (PlayerType::Human, PlayerType::Human),
        AppMode::GameHumanAI => (PlayerType::Human, PlayerType::AI),
        AppMode::GameAIAI => (PlayerType::AI, PlayerType::AI),
        AppMode::None => (PlayerType::Human, PlayerType::Human),
    };
    commands.insert_resource(GameManager {
        custom_position: None,
        white_player_type: white,
        black_player_type: black,
        game_result: GameResult::Playing,
        game_moves: Vec::new(),
        move_color: if app_mode.clone() == AppMode::GameAIAI { 255 } else { Piece::WHITE },
        executed_board_move: None,
    });
}

pub fn execute_board_move(
    mut make_move_evr: EventReader<BoardMakeMove>,
    mut board: ResMut<Board>,
    zobrist: Res<Zobrist>,
    mut manager: ResMut<GameManager>,
) {
    for make_move_event in make_move_evr.iter() {
        let mov = make_move_event.mov;

        // let piece = board.square[mov.start().index()];
        // println!("sqr: {}, piece: {:?}, num_pieces: {}", mov.start().square(), piece, board.get_piece_list(piece.piece_type(), piece.color_index()).count());

        board.make_move(mov, false, &zobrist);
        manager.executed_board_move = Some(mov);
    }
}

pub fn on_make_move(
    mut commands: Commands,
    mut processed_move_evw: EventWriter<ProcessedMove>,
    mut move_gen: ResMut<MoveGenerator>,
    board: Res<Board>,
    precomp: Res<PrecomputedMoveData>,
    bbutils: Res<BitBoardUtils>,
    magic: Res<MagicBitBoards>,
    mut manager: ResMut<GameManager>,
    mut stats: ResMut<CalcStatistics>,
) {
    if let Some(mov) = manager.executed_board_move {
        let time_start = SystemTime::now();
        move_gen.generate_moves(&board, &precomp, &bbutils, &magic, false);
        let move_gen_time = SystemTime::now().duration_since(time_start).unwrap().as_micros();
        stats.move_gen_time = move_gen_time as f32;
    
        manager.game_moves.push(mov);
    
        match manager.gen_game_result(board.as_ref(), &move_gen.moves, move_gen.in_check()) {
            GameResult::None => (),
            GameResult::Playing => (),
            GameResult::WhiteIsMated => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::BlackIsMated => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::Stalemate => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::Repetition => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::FiftyMoveRule => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::InsufficientMaterial => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::DrawByArbiter => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::WhiteTimeout => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
            GameResult::BlackTimeout => { commands.insert_resource(NextState(Some(AppState::GameOver))) },
        }
        manager.executed_board_move = None;
        processed_move_evw.send(ProcessedMove {});
    }
}

pub fn advance_turn(
    mut manager: ResMut<GameManager>,
    mut processed_move_evr: EventReader<ProcessedMove>,
    mut can_make_move_evw: EventWriter<CanMakeMove>,
    board: Res<Board>,
) {
    for _event in processed_move_evr.iter() {
        manager.move_color = board.move_color;
        can_make_move_evw.send(CanMakeMove {});
    }
}


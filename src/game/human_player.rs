use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    ui::{board::{BoardUITransform, BoardUIResetPiecePosition, BoardSetSquareColor, BoardUI, BoardResetSquareColors}, theme::SquareColorTypes}, 
    board::moves::Move,
    board::coord::Coord,
    board::piece::*,
    board::board::Board,
    game::player::Player,
    move_gen::move_generator::MoveGenerator,
    game::manager::BoardMakeMove,
};


#[derive(PartialEq)]
pub enum PlayerInputState {
    None,
    PieceSelected, 
    DraggingPiece,
}

#[derive(Component)]
pub struct HumanPlayer {
    pub current_state: PlayerInputState,
    selected_piece_sqr: Coord,
}

impl Default for HumanPlayer {
    fn default() -> Self {
        HumanPlayer {
            current_state: PlayerInputState::None,
            selected_piece_sqr: Coord::new(0, 0),
        }
    }
}


pub fn handle_player_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut HumanPlayer, &Player)>,
    buttons: Res<Input<MouseButton>>,
    board_transform: Res<BoardUITransform>,
    board: Res<Board>,
    mut reset_piece_position_evw: EventWriter<BoardUIResetPiecePosition>,
    mut make_move_evw: EventWriter<BoardMakeMove>,
    mut set_sqr_color_evw: EventWriter<BoardSetSquareColor>,
    mut reset_sqr_color_evw: EventWriter<BoardResetSquareColors>,
    mut board_ui: ResMut<BoardUI>,
    move_gen: Res<MoveGenerator>,
) {
    if let Some(mpos) = window_query.single().cursor_position() {
        for (mut player, player_data) in player_query.iter_mut() {
            if player_data.team != board.move_color { continue };
            if player.current_state == PlayerInputState::None {
                handle_piece_selection(
                    &buttons,
                    &board_transform,
                    &board,
                    &mut player,
                    mpos,
                    &mut set_sqr_color_evw,
                    &mut board_ui,
                    &move_gen,
                );
            } else if player.current_state == PlayerInputState::DraggingPiece {
                if buttons.just_released(MouseButton::Left) {
                    board_ui.dragged_piece = None;
                    handle_piece_placement(
                        &mut player,
                        &board_transform,
                        &buttons,
                        &mut reset_piece_position_evw,
                        &board,
                        mpos,
                        &mut make_move_evw,
                        &mut set_sqr_color_evw,
                        &mut reset_sqr_color_evw,
                        &mut board_ui,
                        &move_gen,
                    );
                }
            } else if player.current_state == PlayerInputState::PieceSelected {
                if buttons.just_pressed(MouseButton::Left) {
                    handle_piece_placement(
                        &mut player,
                        &board_transform,
                        &buttons,
                        &mut reset_piece_position_evw,
                        &board,
                        mpos,
                        &mut make_move_evw,
                        &mut set_sqr_color_evw,
                        &mut reset_sqr_color_evw,
                        &mut board_ui,
                        &move_gen,
                    );
                }
            }

            if buttons.just_pressed(MouseButton::Right) {
                cancel_piece_selection(
                    &mut player,
                    &mut reset_piece_position_evw,
                    &mut reset_sqr_color_evw,
                )
            }
        }
    }
}

pub fn handle_piece_selection(
    buttons: &Res<Input<MouseButton>>,
    board_transform: &Res<BoardUITransform>,
    board: &Res<Board>,
    player: &mut Mut<HumanPlayer>,
    mpos: Vec2,
    set_sqr_color_evw: &mut EventWriter<BoardSetSquareColor>,
    board_ui: &mut ResMut<BoardUI>,
    move_gen: &Res<MoveGenerator>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(piece_sqr) = board_transform.get_hovered_square(mpos) {
            player.selected_piece_sqr = piece_sqr;
            let idx = piece_sqr.index();
            if board.square[idx as usize].is_color(board.move_color) {
                // println!("Highlight legal moves");
                for legal_move in move_gen.moves.iter() {
                    if legal_move.start() == player.selected_piece_sqr {
                        set_sqr_color_evw.send(BoardSetSquareColor {
                            color: SquareColorTypes::Legal,
                            rank: legal_move.target().rank(),
                            file: legal_move.target().file(),
                        })
                    }
                }
                set_sqr_color_evw.send(BoardSetSquareColor {
                    color: SquareColorTypes::Selected,
                    rank: player.selected_piece_sqr.rank(),
                    file: player.selected_piece_sqr.file(),
                });
                board_ui.dragged_piece = Some(player.selected_piece_sqr);
                player.current_state = PlayerInputState::DraggingPiece;
            }
        }
    }
}

pub fn cancel_piece_selection(
    player: &mut Mut<HumanPlayer>,
    reset_piece_position_evw: &mut EventWriter<BoardUIResetPiecePosition>,
    reset_sqr_color_evw: &mut EventWriter<BoardResetSquareColors>,
) {
    if player.current_state != PlayerInputState::None {
        player.current_state = PlayerInputState::None;
        reset_sqr_color_evw.send(BoardResetSquareColors {
            color: None,
        });
        reset_piece_position_evw.send(BoardUIResetPiecePosition {
            origin_file: player.selected_piece_sqr.file(),
            origin_rank: player.selected_piece_sqr.rank(),
        });
    }
}

pub fn handle_piece_placement(
    mut player: &mut Mut<HumanPlayer>,
    board_transform: &Res<BoardUITransform>,
    buttons: &Res<Input<MouseButton>>,
    mut reset_piece_position_evw: &mut EventWriter<BoardUIResetPiecePosition>,
    board: &Res<Board>,
    mpos: Vec2,
    mut make_move_evw: &mut EventWriter<BoardMakeMove>,
    mut set_sqr_color_evw: &mut EventWriter<BoardSetSquareColor>,
    mut reset_sqr_color_evw: &mut EventWriter<BoardResetSquareColors>,
    mut board_ui: &mut ResMut<BoardUI>,
    move_gen: &Res<MoveGenerator>,
) {
    if let Some(target_sqr) = board_transform.get_hovered_square(mpos) {
        if target_sqr.is_eq(player.selected_piece_sqr) {
            reset_piece_position_evw.send(BoardUIResetPiecePosition {
                origin_file: target_sqr.file(),
                origin_rank: target_sqr.rank(),
            });
            if player.current_state == PlayerInputState::DraggingPiece {
                player.current_state = PlayerInputState::PieceSelected;
            } else {
                player.current_state = PlayerInputState::None;
                reset_sqr_color_evw.send(BoardResetSquareColors {
                    color: None,
                });
            }
        } else {
            let target_idx = target_sqr.square();
            if board.square[target_idx as usize].is_color(board.move_color) && board.square[target_idx as usize] != Piece::NULL {
                cancel_piece_selection(&mut player, &mut reset_piece_position_evw, &mut reset_sqr_color_evw);
                handle_piece_selection(
                    &buttons,
                    &board_transform,
                    board,
                    player,
                    mpos,
                    &mut set_sqr_color_evw,
                    &mut board_ui,
                    move_gen
                );
            } else {
                player_make_move(
                    Move::from_start_end(player.selected_piece_sqr.square(), target_idx), 
                    &mut make_move_evw
                );
                cancel_piece_selection(&mut player, &mut reset_piece_position_evw, &mut reset_sqr_color_evw)
            }
        }
    } else {
        cancel_piece_selection(&mut player, &mut reset_piece_position_evw, &mut reset_sqr_color_evw);
    }
}

pub fn player_make_move(
    mov: Move,
    make_move_evw: &mut EventWriter<BoardMakeMove>,
) {
    make_move_evw.send(BoardMakeMove {
        mov, 
    });
}
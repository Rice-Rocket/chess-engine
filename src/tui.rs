use std::io::{stdout, Stdout, Write};
use termion::{async_stdin, clear, color, cursor, event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}};

use engine::{bitboard::bb::BitBoard, board::{coord::Coord, moves::Move, piece::Piece, Board}, color::{Black, White}, eval::Evaluation, game::{Game, PlayerType}, result::GameResult, utils};


// const BOARD_CHARACTERS_LIGHT: &str = "─│┌┐└┘├┤┬┴┼";
// const BOARD_CHARACTERS_HEAVY: &str = "━┃┏┓┗┛┣┫┳┻╋";

const TRUECOLOR_DARK_SQUARE: color::Rgb = color::Rgb(88, 113, 61);
const TRUECOLOR_LIGHT_SQUARE: color::Rgb = color::Rgb(128, 164, 91);
const TRUECOLOR_BLACK_PIECE: color::Rgb = color::Rgb(71, 68, 66);
const TRUECOLOR_WHITE_PIECE: color::Rgb = color::Rgb(249, 249, 249);
const TRUECOLOR_DARK_LASTMOVE: color::Rgb = color::Rgb(227, 166, 34);
const TRUECOLOR_LIGHT_LASTMOVE: color::Rgb = color::Rgb(251, 183, 68);
const TRUECOLOR_DARK_VALID: color::Rgb = color::Rgb(212, 109, 81);
const TRUECOLOR_LIGHT_VALID: color::Rgb = color::Rgb(236, 126, 106);
const TRUECOLOR_DARK_BB: color::Rgb = color::Rgb(115, 187, 218);
const TRUECOLOR_LIGHT_BB: color::Rgb = color::Rgb(88, 170, 193);

fn display_board(
    stdout: &mut RawTerminal<Stdout>,
    board: &Board,
    cursor: (i8, i8),
    selected: Option<(i8, i8)>,
    valid_moves: &[Move],
    overlayed_bb: Option<BitBoard>,
    truecolor: bool,
) {
    for mut sqr in Coord::iter_squares() {
        sqr = sqr.flip_rank();
        let is_light = (sqr.rank() + sqr.file()) % 2 != 0;

        if sqr.square() % 8 == 0 {
            if sqr.rank() == 7 {
                write!(stdout, "{}┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓{}\n\r", color::Fg(color::LightBlack), color::Fg(color::Reset)).unwrap();
            } else {
                write!(stdout, "\n\r{}┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫{}\n\r", color::Fg(color::LightBlack), color::Fg(color::Reset)).unwrap();
            }
        }

        let mut s = String::from("   ");
        for piece in 0..Piece::MAX_PIECE_INDEX as usize + 1 {
            if board.piece_bitboards[piece].contains_square(sqr.square()) {
                if truecolor {
                    let p = Piece::new(piece as u8 | Piece::BLACK);
                    if Piece::new(piece as u8).color() == Piece::WHITE {
                        s = format!(" {}{}{} ", color::Fg(TRUECOLOR_WHITE_PIECE), p, color::Fg(color::Reset));
                    } else {
                        s = format!(" {}{}{} ", color::Fg(TRUECOLOR_BLACK_PIECE), p, color::Fg(color::Reset));
                    }
                } else {
                    s = format!(" {} ", Piece::new(piece as u8));
                }
                break;
            }
        }

        if sqr.file() == 0 {
            write!(
                stdout,
                "{}┃{}",
                color::Fg(color::LightBlack),
                color::Fg(color::Reset),
            ).unwrap();
        }

        if truecolor {
            if let Some(bb) = overlayed_bb {
                if bb.contains_square(sqr.square()) {
                    if is_light {
                        write!(stdout, "{}", color::Bg(TRUECOLOR_LIGHT_BB)).unwrap();
                    } else {
                        write!(stdout, "{}", color::Bg(TRUECOLOR_DARK_BB)).unwrap();
                    }
                } else if is_light {
                    write!(stdout, "{}", color::Bg(TRUECOLOR_LIGHT_SQUARE)).unwrap();
                } else {
                    write!(stdout, "{}", color::Bg(TRUECOLOR_DARK_SQUARE)).unwrap();
                }
            } else if is_light {
                write!(stdout, "{}", color::Bg(TRUECOLOR_LIGHT_SQUARE)).unwrap();
            } else {
                write!(stdout, "{}", color::Bg(TRUECOLOR_DARK_SQUARE)).unwrap();
            }
        } else if let Some(bb) = overlayed_bb {
            if bb.contains_square(sqr.square()) {
                write!(stdout, "{}{}", color::Bg(color::White), color::Fg(color::Black)).unwrap();
            } else {
                write!(stdout, "{}{}", color::Bg(color::Black), color::Fg(color::LightWhite)).unwrap();
            }
        } else if is_light {
            write!(stdout, "{}{}", color::Bg(color::White), color::Fg(color::Black)).unwrap();
        } else {
            write!(stdout, "{}{}", color::Bg(color::Black), color::Fg(color::LightWhite)).unwrap();
        }

        if let Some(p) = selected {
            if sqr.rank() == p.1 && sqr.file() == p.0 {
                if truecolor {
                    if is_light {
                        write!(stdout, "{}", color::Bg(TRUECOLOR_LIGHT_LASTMOVE)).unwrap();
                    } else {
                        write!(stdout, "{}", color::Bg(TRUECOLOR_DARK_LASTMOVE)).unwrap();
                    }
                } else {
                    write!(stdout, "{}{}", color::Bg(color::LightGreen), color::Fg(color::Black)).unwrap();
                }
            }
        }

        for m in valid_moves.iter() {
            if m.target() == sqr {
                if truecolor {
                    if is_light {
                        write!(stdout, "{}", color::Bg(TRUECOLOR_LIGHT_VALID)).unwrap();
                    } else {
                        write!(stdout, "{}", color::Bg(TRUECOLOR_DARK_VALID)).unwrap();
                    }
                } else {
                    write!(stdout, "{}{}", color::Bg(color::LightRed), color::Fg(color::Black)).unwrap();
                }
                break;
            }
        }

        if sqr.rank() == cursor.1 && sqr.file() == cursor.0 {
            if truecolor {
                if is_light {
                    write!(stdout, "{}", color::Bg(TRUECOLOR_LIGHT_LASTMOVE)).unwrap();
                } else {
                    write!(stdout, "{}", color::Bg(TRUECOLOR_DARK_LASTMOVE)).unwrap();
                }
            } else {
                write!(stdout, "{}{}", color::Bg(color::LightYellow), color::Fg(color::Black)).unwrap();
            }
        }

        write!(
            stdout,
            "{}{}{}┃{}",
            s, color::Bg(color::Reset),
            color::Fg(color::LightBlack),
            color::Fg(color::Reset),
        ).unwrap();
    }

    write!(stdout, "\n\r{}┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛{}\n\r", color::Fg(color::LightBlack), color::Fg(color::Reset)).unwrap();

    stdout.flush().unwrap();
}


enum InputMode {
    Normal,
    Print,
    Overlay,
    Replace,
    SelectPieceOverlay(u8, Box<InputMode>),
    SelectPieceReplace(u8, Box<InputMode>),
    SelectPrecompBitBoard,
}


pub fn start(fen: String, white: PlayerType, black: PlayerType, truecolor: bool, debug: bool) {
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::new(Some(fen), white, black);
    let mut result = GameResult::InProgress;
    let mut cursor = (1, 1);
    let mut selected: Option<(i8, i8)> = None;
    let mut valid_moves: Vec<Move> = vec![];
    let mut move_cycle_index = 0;
    let mut mode = InputMode::Normal;
    let mut printed_dbg_len = None;
    let mut overlayed_bitboard: Option<BitBoard> = None;
    let mut force_move = false;
    let mut game_over = false;

    write!(stdout, "{}", cursor::Hide).unwrap();
    display_board(&mut stdout, &game.board, cursor, None, &valid_moves, None, truecolor);

    stdout.flush().unwrap();

    'main: loop {
        if let Some(c) = stdin.next() {
            write!(stdout, "{}{}", cursor::Up(17), clear::AfterCursor).unwrap();

            match mode {
                InputMode::Normal => match c.unwrap() {
                    Key::Char('q') => break 'main,
                    Key::Char('j') | Key::Down => cursor.1 -= 1,
                    Key::Char('k') | Key::Up => cursor.1 += 1,
                    Key::Char('h') | Key::Left => cursor.0 -= 1,
                    Key::Char('l') | Key::Right => cursor.0 += 1,
                    Key::Char(' ') => if let Some(pos) = selected {
                        if game.player_to_move == PlayerType::Human {
                            if let Some(i) = valid_moves.iter().position(|m| m.target() == cursor.into()) {
                                result = game.make_move(valid_moves[i]);
                                valid_moves.clear();
                                selected = None;
                            } else if force_move {
                                selected = None;
                                result = game.make_move(Move::from_start_end(Coord::from(pos).square(), Coord::from(cursor).square()));
                            } else {
                                selected = Some(cursor);
                                valid_moves = game.valid_human_moves(cursor.into());
                            }
                            force_move = false;
                        }
                    } else {
                        selected = Some(cursor);
                        valid_moves = game.valid_human_moves(cursor.into());
                    },
                        Key::Char('u') => {
                            game.undo_move();
                            valid_moves.clear();
                            selected = None;
                        },
                        Key::Char('K') => {
                            move_cycle_index = (move_cycle_index + 1) % valid_moves.len();
                            if !valid_moves.is_empty() {
                                cursor = valid_moves[move_cycle_index].target().into();
                            }
                        },
                        Key::Char('J') => {
                            move_cycle_index = ((move_cycle_index as isize - 1).rem_euclid(valid_moves.len() as isize)) as usize;
                            if !valid_moves.is_empty() {
                                cursor = valid_moves[move_cycle_index].target().into();
                            }
                        },
                        Key::Char('p') => {
                            if debug {
                                mode = InputMode::Print;
                            }
                        },
                        Key::Char('o') => {
                            if debug {
                                mode = InputMode::Overlay;
                            }
                        },
                        Key::Char('r') => {
                            if debug && game.board.square[Coord::from(cursor)].piece_type() != Piece::KING {
                                mode = InputMode::Replace;
                            }
                        },
                        Key::Char('d') => {
                            if debug && game.board.square[Coord::from(cursor)].piece_type() != Piece::KING {
                                game.board.remove_piece(cursor.into());
                            }
                        },
                        Key::Char('m') => {
                            if debug {
                                selected = Some(cursor);
                                force_move = true;
                            }
                        },
                        Key::Char('M') => {
                            if debug {
                                selected = None;
                                valid_moves.clear();
                                overlayed_bitboard = None;
                                game.board.make_null_move(&game.zobrist);
                            }
                        },
                        Key::Esc => {
                            selected = None;
                            valid_moves.clear();
                            overlayed_bitboard = None;
                        },
                        _ => ()
                },

                InputMode::Print => {
                    match c.unwrap() {
                        Key::Char('f') => {
                            if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
                            write!(stdout, "{}{}\n\r", clear::CurrentLine, utils::fen::fen_from_position(&game.board)).unwrap();
                            printed_dbg_len = Some(1);
                        },
                        Key::Char('z') => {
                            if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
                            write!(stdout, "{}{}\n\r", clear::CurrentLine, &game.zobrist.calc_zobrist_key(&game.board)).unwrap();
                            printed_dbg_len = Some(1);
                        },
                        Key::Char('b') => {
                            if let Some(bb) = overlayed_bitboard {
                                if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
                                write!(stdout, "{}{}\n\r", clear::CurrentLine, bb.0).unwrap();
                                printed_dbg_len = Some(1);
                            }
                        },
                        Key::Char('s') => {
                            let sqr = Coord::from(cursor);
                            if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
                            write!(stdout, "{}{}\n\r", clear::CurrentLine, sqr.index()).unwrap();
                            printed_dbg_len = Some(1);
                        },
                        Key::Char('S') => {
                            let sqr = Coord::from(cursor);
                            if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
                            write!(stdout, "{}{}\n\r", clear::CurrentLine, sqr.inner_value()).unwrap();
                            printed_dbg_len = Some(1);
                        },

                        // NOTE: Used purely for debugging specific functions
                        Key::Char('e') => {
                            let sqr = Coord::from(cursor);
                            let mut eval = Evaluation::new(&game.board, &game.precomp, &game.magics);
                            if game.board.white_to_move { eval.init::<White, Black>() } else { eval.init::<Black, White>() };
                            let v = if game.board.white_to_move {
                                eval.bishop_pawns::<White, Black>()[sqr]
                            } else {
                                eval.bishop_pawns::<Black, White>()[sqr]
                            };

                            if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
                            write!(stdout, "{}{}\n\r", clear::CurrentLine, v).unwrap();
                            printed_dbg_len = Some(1);
                        },
                        _ => (),
                    }
                    mode = InputMode::Normal;
                },
                InputMode::Overlay => {
                    match c.unwrap() {
                        Key::Char('w') => {
                            mode = InputMode::SelectPieceOverlay(Piece::WHITE, Box::new(InputMode::Overlay))
                        },
                        Key::Char('b') => {
                            mode = InputMode::SelectPieceOverlay(Piece::BLACK, Box::new(InputMode::Overlay))
                        },
                        Key::Char('W') => {
                            overlayed_bitboard = Some(game.board.color_bitboards[Board::WHITE_INDEX]);
                            mode = InputMode::Normal;
                        },
                        Key::Char('B') => {
                            overlayed_bitboard = Some(game.board.color_bitboards[Board::BLACK_INDEX]);
                            mode = InputMode::Normal;
                        },
                        Key::Char('a') => {
                            overlayed_bitboard = Some(game.board.all_pieces_bitboard);
                            mode = InputMode::Normal;
                        },
                        Key::Char('o') => {
                            overlayed_bitboard = Some(game.board.friendly_orthogonal_sliders);
                            mode = InputMode::Normal;
                        },
                        Key::Char('O') => {
                            overlayed_bitboard = Some(game.board.enemy_orthogonal_sliders);
                            mode = InputMode::Normal;
                        },
                        Key::Char('d') => {
                            overlayed_bitboard = Some(game.board.friendly_diagonal_sliders);
                            mode = InputMode::Normal;
                        },
                        Key::Char('D') => {
                            overlayed_bitboard = Some(game.board.enemy_diagonal_sliders);
                            mode = InputMode::Normal;
                        },
                        Key::Char('m') => {
                            overlayed_bitboard = Some(game.magics.get_rook_attacks(cursor.into(), game.board.all_pieces_bitboard));
                            mode = InputMode::Normal;
                        },
                        Key::Char('M') => {
                            overlayed_bitboard = Some(game.magics.get_bishop_attacks(cursor.into(), game.board.all_pieces_bitboard));
                            mode = InputMode::Normal;
                        },
                        Key::Char('p') => {
                            overlayed_bitboard = Some(game.movegen.pin_rays);
                            mode = InputMode::Normal;
                        },
                        Key::Char('e') => {
                            mode = InputMode::SelectPrecompBitBoard;
                        },
                        _ => mode = InputMode::Normal,
                    }
                },
                InputMode::SelectPrecompBitBoard => {
                    match c.unwrap() {
                        Key::Char('s') => {
                            overlayed_bitboard = Some(game.precomp.white_pawn_support_mask[Coord::from(cursor)]);
                        },
                        Key::Char('S') => {
                            overlayed_bitboard = Some(game.precomp.black_pawn_support_mask[Coord::from(cursor)]);
                        },
                        Key::Char('p') => {
                            overlayed_bitboard = Some(game.precomp.white_pawn_attacks[Coord::from(cursor)]);
                        },
                        Key::Char('P') => {
                            overlayed_bitboard = Some(game.precomp.black_pawn_attacks[Coord::from(cursor)]);
                        },

                        // NOTE: Used purely for debugging
                        Key::Char('e') => {
                            let mut eval = Evaluation::new(&game.board, &game.precomp, &game.magics);
                            if game.board.white_to_move { eval.init::<White, Black>() } else { eval.init::<Black, White>() };
                            overlayed_bitboard = Some(if game.board.white_to_move {
                                eval.blockers_for_king::<White, Black>()
                            } else {
                                eval.blockers_for_king::<Black, White>()
                            });
                        },
                        Key::Char('E') => {
                            let mut eval = Evaluation::new(&game.board, &game.precomp, &game.magics);
                            if game.board.white_to_move { eval.init::<White, Black>() } else { eval.init::<Black, White>() };
                            overlayed_bitboard = Some(if game.board.white_to_move {
                                eval.pin_rays::<White, Black>().1
                            } else {
                                eval.pin_rays::<Black, White>().1
                            });
                        },
                        _ => ()
                    };
                    mode = InputMode::Normal;
                },
                InputMode::Replace => {
                    match c.unwrap() {
                        Key::Char('w') => {
                            mode = InputMode::SelectPieceReplace(Piece::WHITE, Box::new(InputMode::Replace))
                        },
                        Key::Char('b') => {
                            mode = InputMode::SelectPieceReplace(Piece::BLACK, Box::new(InputMode::Replace))
                        },
                        _ => mode = InputMode::Normal,
                    }
                },
                InputMode::SelectPieceOverlay(color, prev_mode) | InputMode::SelectPieceReplace(color, prev_mode)=> {
                    let mut set_piece = |p: Piece| match *prev_mode {
                        InputMode::Overlay => {
                            overlayed_bitboard = Some(game.board.piece_bitboards[p]);
                        },
                        InputMode::Replace => {
                            game.board.set_piece(p, cursor.into());
                        },
                        _ => unreachable!()
                    };

                    match c.unwrap() {
                        Key::Char('p') => {
                            set_piece(Piece::new(Piece::PAWN | color));
                        },
                        Key::Char('n') => {
                            set_piece(Piece::new(Piece::KNIGHT | color));
                        },
                        Key::Char('b') => {
                            set_piece(Piece::new(Piece::BISHOP | color));
                        },
                        Key::Char('r') => {
                            set_piece(Piece::new(Piece::ROOK | color));
                        },
                        Key::Char('q') => {
                            set_piece(Piece::new(Piece::QUEEN | color));
                        },
                        Key::Char('k') => {
                            set_piece(Piece::new(Piece::KING | color));
                        },
                        _ => ()
                    };
                    mode = InputMode::Normal;
                }
            }

            cursor.0 = cursor.0.clamp(0, 7);
            cursor.1 = cursor.1.clamp(0, 7);
            display_board(&mut stdout, &game.board, cursor, selected, &valid_moves, overlayed_bitboard, truecolor);
            stdout.flush().unwrap();
        }

        let is_terminal = if game_over { true } else { result.is_terminal() };

        while game.player_to_move == PlayerType::Computer && !is_terminal {
            if let Some(res) = game.try_make_computer_move() {
                result = res;
                write!(stdout, "{}{}", cursor::Up(17), clear::AfterCursor).unwrap();
                display_board(&mut stdout, &game.board, cursor, selected, &valid_moves, overlayed_bitboard, truecolor);

                stdout.flush().unwrap();
                break;
            }
        }

        if is_terminal && !game_over {
            let message = match result {
                GameResult::InProgress => unreachable!(),
                GameResult::WhiteMated => "Checkmate: Black Wins",
                GameResult::BlackMated => "Checkmate: White Wins",
                GameResult::WhiteTimeout => "Timeout: Black Wins",
                GameResult::BlackTimeout => "Timeout: White Wins",
                GameResult::Stalemate => "Draw by Stalemate",
                GameResult::Repetition => "Draw by Three-fold Repetition",
                GameResult::FiftyMoveRule => "Draw by Fifty Move Rule",
                GameResult::InsufficientMaterial => "Draw by Insufficient Material",
            };

            write!(stdout, "{}", cursor::Up(17)).unwrap();
            game_over = true;

            if let Some(lines) = printed_dbg_len { write!(stdout, "{}", cursor::Up(lines)).unwrap(); }
            write!(stdout, "{}{}{}{}\n\r", clear::CurrentLine, color::Fg(color::Yellow), message, color::Fg(color::Reset)).unwrap();
            printed_dbg_len = Some(1);

            display_board(&mut stdout, &game.board, cursor, selected, &valid_moves, overlayed_bitboard, truecolor);
            stdout.flush().unwrap();
        }
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}

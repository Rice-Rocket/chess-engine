use std::{fs, io::{stdout, Write}, path::PathBuf};

use engine::{board::{coord::Coord, moves::Move, piece::Piece, Board}, game::{Game, PlayerType}, result::GameResult, search::options::SearchOptions, utils::representation::{FILE_NAMES, RANK_NAMES}};
use external_uci::{ExternalUci, ExternalUciCapable};
use termion::{async_stdin, clear, color, cursor, event::Key, input::TermRead, raw::IntoRawMode};

use crate::{tui::display_board, CommandDisplayMethod};

pub async fn start(mut opponent: ExternalUci, positions: PathBuf, movetime: u32, display: CommandDisplayMethod) -> Result<(), String> {
    let fens = fs::read_to_string(positions.clone()).map_err(|_| format!("failed to read {}", positions.display()))?;
    let fens = fens.lines();

    let opts = SearchOptions {
        movetime: Some(movetime),
        depth: None,
    };
    let (mut wins, mut losses, mut draws) = (0, 0, 0);
    let mut opponent_in_search = false;

    let mut stdin = if display == CommandDisplayMethod::Tui {
        Some(async_stdin().keys())
    } else { None };
    let mut stdout = if display == CommandDisplayMethod::Tui {
        let mut stdout = stdout().into_raw_mode().unwrap();
        writeln!(stdout).unwrap();
        Some(stdout)
    } else { None };

    'main: for fen in fens {
        if fen.trim() == "" { continue };

        for i in 0..=1 {
            let white = if i == 0 { PlayerType::Computer } else { PlayerType::Human };
            let black = if i == 0 { PlayerType::Human } else { PlayerType::Computer };
            let mut game = Game::new(Some(fen.to_string()), opts, white, black);
            let mut result = GameResult::InProgress;

            if let Some(sout) = stdout.as_mut() {
                display_board(sout, &game.board, (-1, -1), None, &[], None, true, None);
                sout.flush().unwrap();
            }

            'game: loop {
                if result.is_terminal() {
                    if result.is_draw() {
                        draws += 1;
                    } else if result.is_white_win() {
                        if white == PlayerType::Computer {
                            wins += 1;
                        } else {
                            losses += 1;
                        }
                    } else if result.is_black_win() {
                        if black == PlayerType::Computer {
                            wins += 1;
                        } else {
                            losses += 1;
                        }
                    }
                    
                    break 'game;
                }

                if let CommandDisplayMethod::Tui = display {
                    if let Some(sin) = stdin.as_mut() {
                        if let Some(c) = sin.next() {
                            match c.unwrap() {
                                Key::Char('q') | Key::Ctrl('c') => break 'main,
                                _ => ()
                            }
                        }
                    }
                }

                let mut make_move_post = false;
                if game.player_to_move == PlayerType::Human {
                    if !opponent_in_search {
                        opponent.set_position_moves(fen, game.board.move_log.iter().map(|m| name_from_move(*m).unwrap()).collect()).await.unwrap();
                        opponent.go_time(movetime as usize).await.unwrap();
                        opponent_in_search = true;
                    }

                    if let Some(bestmove) = opponent.get_bestmove() {
                        opponent_in_search = false;
                        game.board.make_move(move_from_name(&game.board, &bestmove).unwrap(), false, &game.zobrist);
                        make_move_post = true;
                    } else {
                        continue 'game;
                    }
                } else if game.player_to_move == PlayerType::Computer {
                    if let Some(res) = game.try_make_computer_move() {
                        result = res;
                    } else {
                        continue 'game;
                    }
                }

                match display {
                    CommandDisplayMethod::None => {

                    },
                    CommandDisplayMethod::Tui => {
                        if let Some(sout) = stdout.as_mut() {
                            write!(sout, "{}{}", cursor::Up(18), clear::AfterCursor).unwrap();
                            display_board(sout, &game.board, (-1, -1), None, &[], None, true, None);
                            sout.flush().unwrap();
                        }
                    },
                    CommandDisplayMethod::Gui => {

                    }
                }

                if make_move_post {
                    result = game.make_move_post();
                }
            }

            match display {
                CommandDisplayMethod::None => {
                    print!(
                        "\r{}{}wins: {}{}  |  {}draws: {}{}  |  {}losses: {}{}",
                        clear::CurrentLine,
                        color::Fg(color::Green), wins, color::Fg(color::Reset),
                        color::Fg(color::Yellow), draws, color::Fg(color::Reset),
                        color::Fg(color::Red), losses, color::Fg(color::Reset),
                    );
                    std::io::stdout().flush().unwrap();
                },
                CommandDisplayMethod::Tui => {
                    if let Some(sout) = stdout.as_mut() {
                        write!(sout, "{}{}", cursor::Up(19), clear::CurrentLine).unwrap();
                        write!(
                            sout, "{}wins: {}{}  |  {}draws: {}{}  |  {}losses: {}{}\n\r",
                            color::Fg(color::Green), wins, color::Fg(color::Reset),
                            color::Fg(color::Yellow), draws, color::Fg(color::Reset),
                            color::Fg(color::Red), losses, color::Fg(color::Reset),
                        ).unwrap();
                        sout.flush().unwrap();
                    }
                },
                CommandDisplayMethod::Gui => {

                },
            }
        }
    }

    match display {
        CommandDisplayMethod::None => {
            println!();
        },
        CommandDisplayMethod::Tui => {
            if let Some(sout) = stdout.as_mut() {
                write!(sout, "{}{}", cursor::Up(18), clear::AfterCursor).unwrap();
            }
        },
        CommandDisplayMethod::Gui => {

        },
    }

    opponent.stop().await.unwrap();
    opponent.quit().await.unwrap();

    Ok(())
}


fn square_from_name(s: &str) -> Option<Coord> {
    let mut chars = s.chars();
    let file = chars.next()?;
    let rank = chars.next()?;

    Some(Coord::new(
        FILE_NAMES.chars().position(|x| x == file)? as i8,
        RANK_NAMES.chars().position(|x| x == rank)? as i8,
    ))
}

fn name_from_square(c: Coord) -> Option<String> {
    let mut s = String::new();
    s.push(FILE_NAMES.chars().nth(c.file() as usize)?);
    s.push(RANK_NAMES.chars().nth(c.rank() as usize)?);
    Some(s)
}

fn move_from_name(board: &Board, m: &str) -> Option<Move> {
    if m.len() < 4 { return None };
    let start_sqr = square_from_name(&m[0..2])?;
    let target_sqr = square_from_name(&m[2..4])?;

    let piece = board.square[start_sqr].piece_type();

    let flag = if piece == Piece::PAWN {
        if m.len() > 4 {
            match m.chars().last().unwrap() {
                'n' => Move::KNIGHT_PROMOTION,
                'b' => Move::BISHOP_PROMOTION,
                'r' => Move::ROOK_PROMOTION,
                'q' => Move::QUEEN_PROMOTION,
                _ => Move::NORMAL,
            }
        } else if (target_sqr.rank() - start_sqr.rank()).abs() == 2 {
            Move::PAWN_TWO_FORWARD
        } else if start_sqr.file() != target_sqr.file() && board.square[target_sqr].piece_type() == Piece::NONE {
            Move::EN_PASSANT_CAPTURE
        } else {
            Move::NORMAL
        }
    } else if piece == Piece::KING {
        if (start_sqr.file() - target_sqr.file()).abs() > 1 {
            Move::CASTLING
        } else {
            Move::NORMAL
        }
    } else {
        Move::NORMAL
    };

    Some(Move::from_start_end_flagged(start_sqr.square(), target_sqr.square(), flag))
}

fn name_from_move(m: Move) -> Option<String> {
    let start_sqr = name_from_square(m.start())?;
    let target_sqr = name_from_square(m.target())?;
    let mut name = format!("{}{}", start_sqr, target_sqr);

    match m.move_flag() {
        Move::KNIGHT_PROMOTION => name.push('n'),
        Move::BISHOP_PROMOTION => name.push('b'),
        Move::ROOK_PROMOTION => name.push('r'),
        Move::QUEEN_PROMOTION => name.push('q'),
        _ => ()
    }

    Some(name)
}

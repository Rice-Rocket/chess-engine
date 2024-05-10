use std::io::{stdin, stdout, Stdout, Write};
use termion::{clear, color, cursor, event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}};

use engine::{board::{coord::Coord, moves::Move, piece::Piece, Board}, game::Game};


// const BOARD_CHARACTERS_LIGHT: &str = "─│┌┐└┘├┤┬┴┼";
// const BOARD_CHARACTERS_HEAVY: &str = "━┃┏┓┗┛┣┫┳┻╋";

fn display_board(
    stdout: &mut RawTerminal<Stdout>,
    board: &Board,
    cursor: (i8, i8),
    selected: Option<(i8, i8)>,
    valid_moves: &[Move],
) {
    for mut sqr in Coord::iter_squares() {
        sqr = sqr.flip_rank();
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
                s = format!(" {} ", Piece::new(piece as u8));
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

        if (sqr.rank() + sqr.file()) % 2 == 0 {
            write!(stdout, "{}{}", color::Bg(color::Black), color::Fg(color::LightWhite)).unwrap();
        } else {
            write!(stdout, "{}{}", color::Bg(color::White), color::Fg(color::Black)).unwrap();
        }

        if let Some(p) = selected {
            if sqr.rank() == p.1 && sqr.file() == p.0 {
                write!(stdout, "{}{}", color::Bg(color::LightGreen), color::Fg(color::Black)).unwrap();
            }
        }

        for m in valid_moves.iter() {
            if m.target() == sqr {
                write!(stdout, "{}{}", color::Bg(color::LightRed), color::Fg(color::Black)).unwrap();
                break;
            }
        }

        if sqr.rank() == cursor.1 && sqr.file() == cursor.0 {
            write!(stdout, "{}{}", color::Bg(color::LightYellow), color::Fg(color::Black)).unwrap();
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


pub fn start(fen: String) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::new(Some(fen));
    let mut cursor = (1, 1);
    let mut selected: Option<(i8, i8)> = None;
    let mut valid_moves: Vec<Move> = vec![];
    let mut move_cycle_index = 0;

    write!(stdout, "{}", cursor::Hide).unwrap();
    display_board(&mut stdout, &game.board, cursor, None, &valid_moves);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}", cursor::Up(17), clear::AfterCursor).unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('j') | Key::Down => cursor.1 -= 1,
            Key::Char('k') | Key::Up => cursor.1 += 1,
            Key::Char('h') | Key::Left => cursor.0 -= 1,
            Key::Char('l') | Key::Right => cursor.0 += 1,
            Key::Char(' ') | Key::Char('o') => if let Some(_pos) = selected {
                if let Some(i) = valid_moves.iter().position(|m| m.target() == cursor.into()) {
                    game.make_move(valid_moves[i]);
                    valid_moves.clear();
                    selected = None;
                } else {
                    selected = Some(cursor);
                    valid_moves = game.valid_moves(cursor.into());
                }
            } else {
                selected = Some(cursor);
                valid_moves = game.valid_moves(cursor.into());
            },
            Key::Char('u') => {
                game.undo_move();
                valid_moves.clear();
                selected = None;
            },
            Key::Char('K') | Key::Char('n') => {
                move_cycle_index = (move_cycle_index + 1) % valid_moves.len();
                if !valid_moves.is_empty() {
                    cursor = valid_moves[move_cycle_index].target().into();
                }
            },
            Key::Char('J') | Key::Char('p') => {
                move_cycle_index = ((move_cycle_index as isize - 1).rem_euclid(valid_moves.len() as isize)) as usize;
                if !valid_moves.is_empty() {
                    cursor = valid_moves[move_cycle_index].target().into();
                }
            },
            Key::Esc => {
                selected = None;
                valid_moves.clear();
            },
            _ => ()
        }

        cursor.0 = cursor.0.clamp(0, 7);
        cursor.1 = cursor.1.clamp(0, 7);
        display_board(&mut stdout, &game.board, cursor, selected, &valid_moves);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}

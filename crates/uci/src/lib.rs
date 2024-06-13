use engine::{board::{coord::Coord, Board}, game::{Game, PlayerType}, search::options::SearchOptions, utils::fen};
use ucimove::move_from_name;

use crate::ucimove::name_from_move;

mod ucimove;
mod channel;

pub fn start() {
    let stdin = channel::spawn_stdin();
    let mut game = Game::new(None, SearchOptions::default(), PlayerType::Human, PlayerType::Human);
    let mut finished = true;

    'main: loop {
        if let Ok(line) = stdin.try_recv() {
            match line.split_whitespace().next() {
                None | Some("") => (),
                Some("uci") => {
                    println!("uciok")
                },
                Some("isready") => {
                    println!("readyok")
                },
                Some("ucinewgame") => {
                    game.searcher.abort();
                    game.board = Board::load_position(None, &mut game.zobrist);
                    finished = true;
                },
                Some("position") => {
                    position(&mut game, &line);
                },
                Some("go") => {
                    if go(&mut game, &line).is_some() {
                        finished = false;
                    }
                },
                Some("d") => {
                    display(&game);
                },
                Some("stop") => {
                    game.searcher.abort();
                },
                Some("quit") => {
                    game.searcher.abort();
                    break 'main;
                },
                Some(cmd) => {
                    println!("Unknown command: '{}'. Type help for more information.", cmd)
                }
            }
        }

        if !finished {
            if let Some(bestmove) = game.searcher.best_move() {
                println!("info depth {}", game.searcher.diagnostics.depth_searched);
                println!("bestmove {}", name_from_move(bestmove).unwrap());
                finished = true;
            }
        }
    }
}


pub fn position(game: &mut Game, cmd: &str) -> Option<()> {
    let args = cmd.split_whitespace();

    if args.clone().any(|s| s == "startpos") {
        game.board = Board::load_position(None, &mut game.zobrist);
    } else if let Some(i) = args.clone().position(|s| s == "fen") {
        let mut fen_args = args.clone().skip(i + 1);
        let mut fen = fen_args.next()?.to_string();
        for _ in 0..5 {
            fen.push_str(&format!(" {}", fen_args.next()?));
        }
        game.board = Board::load_position(Some(fen.to_string()), &mut game.zobrist);
    } else {
        return None;
    }

    if let Some(i) = args.clone().position(|s| s == "moves") {
        for m_name in args.skip(i + 1) {
            let m = move_from_name(&game.board, m_name)?;
            game.board.make_move(m, false, &game.zobrist);
        }
    }

    Some(())
}

pub fn go(game: &mut Game, cmd: &str) -> Option<()> {
    let args = cmd.split_whitespace();

    let mut opts = SearchOptions {
        movetime: None,
        depth: None,
    };

    if let Some(i) = args.clone().position(|s| s == "movetime") {
        let movetime = args.clone().nth(i + 1)?;
        opts.movetime = Some(movetime.parse().ok()?);
    } else if let Some(i) = args.clone().position(|s| s == "depth") {
        let depth = args.clone().nth(i + 1)?;
        opts.depth = Some(depth.parse().ok()?);
    }

    game.searcher.begin_search(opts, &mut game.board, &game.zobrist, &mut game.movegen);
    Some(())
}

pub fn display(game: &Game) {
    let mut s = String::new();
    s.push('\n');
    let last_move = if let Some(last) = game.board.move_log.last() { last.target().square() } else { -1 };

    for y in 0..8 {
        let rank = 7 - y;
        s.push_str("+---+---+---+---+---+---+---+---+\n");

        for file in 0..8 {
            let sqr = Coord::new(file, rank);
            let highlight = sqr.square() == last_move;
            let piece = game.board.square[sqr];

            if highlight {
                s.push_str(&format!("|({:#?})", piece));
            } else {
                s.push_str(&format!("| {:#?} ", piece));
            }

            if file == 7 {
                s.push_str(&format!("| {}\n", rank + 1));
            }
        }

        if y == 7 {
            s.push_str("+---+---+---+---+---+---+---+---+\n");
            s.push_str("  a   b   c   d   e   f   g   h  \n\n");

            s.push_str(&format!("Fen: {}\n", fen::fen_from_position(&game.board)));
            s.push_str(&format!("Key: {:X}\n", game.board.current_state.zobrist_key));
        }
    }

    print!("{}", s);
}

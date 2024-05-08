#![allow(unused)]

mod bitboard;
mod board;
mod move_gen;
mod utils;
mod game;
mod prelude;
mod cli;
mod eval;
mod color;
mod perft;

use std::{fmt, time::{Duration, Instant}};
use board::{zobrist::Zobrist, Board};
use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand, ValueEnum};
use termion::color as tcolor;
use game::Game;


#[derive(Parser)]
#[command(version, author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Eval {
        fen: String,

        #[arg(long, short, value_name = "DEPTH", default_value = "4")]
        depth: u16,
    },
    Perft {
        #[arg(long, short, value_name = "POSITION_NUMBER", default_value = "1")]
        position: u16,

        #[arg(long, short, value_name = "DEPTH")]
        depth: u16,

        /// Only test the single, given depth rather than testing all depths up to the given depth. 
        #[arg(long, short)]
        only_one_depth: bool,
    },
    Play {
        #[arg(long, short, value_name = "PLAYER_TYPE", default_value = "human")]
        white_player: PlayerType,

        #[arg(long, short, value_name = "PLAYER_TYPE", default_value = "human")]
        black_player: PlayerType,
    },
}


#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
enum PlayerType {
    #[default]
    Human,
    AI
}


fn main() {
    let cli_input = Cli::parse();

    match cli_input.command {
        Commands::Eval {
            fen,
            depth,
        } => {
            let mut zobrist = Zobrist::new();
            let board = Board::load_position(Some(fen), &mut zobrist);
            let eval = board.evaluate();
            println!("evaluation: {}", eval);
        },
        Commands::Perft {
            position,
            depth,
            only_one_depth,
        } => {
            let fen = match position {
                1 => "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                2 => "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
                3 => "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -",
                4 => "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                5 => "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                6 => "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                _ => {
                    let err = Cli::command().error(
                        ErrorKind::InvalidValue,
                        format!("position {} does not exist. valid positions are from 1-6", position),
                    );
                    err.print();
                    return;
                },
            };

            perft::expected_depth(position, depth);

            println!("testing position {}, fen = {}", position, fen);

            let mut test_once = |depth: u16| {
                let mut game = Game::new(Some(fen.to_string()));

                let start = Instant::now();
                let expected_nodes = perft::expected_nodes(position, depth);
                let nodes = perft::movegen_test(&mut game.board, &game.zobrist, &mut game.movegen, &game.precomp, &game.bbutils, &game.magics, depth);
                let time_spent = Instant::now().duration_since(start);

                if expected_nodes == nodes {
                    println!(
                        "{}test passed {}|{} depth = {}{}{},{} took {}{:.4}s{},{} found {}{}{} nodes",
                        tcolor::Fg(tcolor::Green), tcolor::Fg(tcolor::LightBlack), tcolor::Fg(tcolor::Reset),
                        tcolor::Fg(tcolor::Yellow), depth, tcolor::Fg(tcolor::LightBlack), tcolor::Fg(tcolor::Reset),
                        tcolor::Fg(tcolor::Yellow), time_spent.as_secs_f32(), tcolor::Fg(tcolor::LightBlack), tcolor::Fg(tcolor::Reset),
                        tcolor::Fg(tcolor::Yellow), nodes, tcolor::Fg(tcolor::Reset),
                    );
                } else {
                    println!(
                        "{}test failed {}|{} depth = {}{}{},{} found {}{}{},{} expected {}{}{} nodes",
                        tcolor::Fg(tcolor::Red), tcolor::Fg(tcolor::LightBlack), tcolor::Fg(tcolor::Reset),
                        tcolor::Fg(tcolor::Yellow), depth, tcolor::Fg(tcolor::LightBlack), tcolor::Fg(tcolor::Reset),
                        tcolor::Fg(tcolor::Yellow), nodes, tcolor::Fg(tcolor::LightBlack), tcolor::Fg(tcolor::Reset),
                        tcolor::Fg(tcolor::Yellow), expected_nodes, tcolor::Fg(tcolor::Reset),
                    );
                }
            };

            if only_one_depth {
                test_once(depth);
            } else {
                for i in 1..=depth {
                    test_once(i);
                }
            }
        },
        Commands::Play {
            white_player,
            black_player,
        } => {
            cli::start();
        }
    }
}

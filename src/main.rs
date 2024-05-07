#![allow(unused)]

mod bitboard;
mod board;
mod move_gen;
mod utils;
mod game;
mod prelude;
mod cli;

use std::fmt;

use clap::{Parser, Subcommand, CommandFactory, ValueEnum};


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
            todo!();
        },
        Commands::Play {
            white_player,
            black_player,
        } => {
            cli::start();
        }
    }
}

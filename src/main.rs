use engine::{board::{zobrist::Zobrist, Board}, eval::Evaluation, game::PlayerType, move_gen::magics::MagicBitBoards, precomp::Precomputed};
use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand, ValueEnum};
use engine::game::Game;

mod perft;
mod tui;


#[derive(Parser)]
#[command(version, author, about)]
pub struct Cli {
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

        #[arg(long, short, value_name = "FEN")]
        fen: Option<String>,

        #[arg(long, short, value_name = "DEPTH")]
        depth: u16,

        /// Test all depths up to and including the given depth rather than just testing the given
        /// depth. Note that this automatically disables `compare` as it is not supported. 
        #[arg(long, short)]
        all: bool,

        #[arg(long, short)]
        expand_branches: bool,

        #[arg(long, short = 'r')]
        test_recursive: bool,

        /// Compares the expanded branches with data given by stockfish. Requires 
        /// stockfish to be in your PATH. Note that this automatically enables 
        /// `expand-branches` as it is required for this to work. 
        #[arg(long, short)]
        compare: bool,
    },
    Play {
        #[arg(long, short, value_name = "FEN", default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
        fen: String,

        #[arg(long, short = 'c')]
        no_truecolor: bool,

        #[arg(long, short, value_name = "PLAYER_TYPE", default_value = "human")]
        white_player: CommandPlayerType,

        #[arg(long, short, value_name = "PLAYER_TYPE", default_value = "human")]
        black_player: CommandPlayerType,

        #[arg(long, short)]
        debug: bool,
    },
}


#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
enum CommandPlayerType {
    #[default]
    Human,
    Computer,
}

impl From<CommandPlayerType> for PlayerType {
    fn from(val: CommandPlayerType) -> Self {
        match val {
            CommandPlayerType::Human => PlayerType::Human,
            CommandPlayerType::Computer => PlayerType::Computer,
        }
    }
}


#[tokio::main]
async fn main() {
    let cli_input = Cli::parse();

    match cli_input.command {
        Commands::Eval {
            fen,
            depth: _,
        } => {
            let precomp = Precomputed::new();
            let magics = MagicBitBoards::default();
            let mut zobrist = Zobrist::new();
            let board = Board::load_position(Some(fen), &mut zobrist);
            let mut eval = Evaluation::new(&board, &precomp, &magics);
            println!("evaluation: {}", eval.evaluate::<engine::color::White, engine::color::Black>());
        },
        Commands::Perft {
            position,
            fen: fen_str,
            depth,
            all,
            mut expand_branches,
            test_recursive,
            compare,
        } => {
            let fen = if let Some(f) = &fen_str {
                f.to_string()
            } else {
                match position {
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
                        let _ = err.print();
                        return;
                    },
                }.to_string()
            };

            match perft::expected_depth(position, depth) {
                Ok(_) => (),
                Err(_) => return,
            }

            if fen_str.is_some() {
                println!("testing fen = {}", fen);
            } else {
                println!("testing position {}, fen = {}", position, fen);
            }

            if test_recursive {
                let mut game = Game::new(Some(fen), PlayerType::Human, PlayerType::Human);
                let mut cmd = std::process::Command::new("stockfish").spawn();
                match &mut cmd {
                    Ok(proc) => proc.kill().unwrap(),
                    Err(_) => {
                        let err = Cli::command().error(
                            ErrorKind::Io,
                            "stockfish executable not found. make sure stockfish is installed and in your PATH"
                        );
                        let _ = err.print();
                        return;
                    }
                };

                perft::test_perft_recursive(&mut game.board, &game.zobrist, &mut game.movegen, &game.precomp, &game.magics, depth).await;
            } else if all {
                for i in 1..=depth {
                    match perft::test_perft(position, i, &fen, expand_branches, false).await {
                        Ok(_) => (),
                        Err(_) => return
                    }
                }
            } else {
                if compare { expand_branches = true };
                match perft::test_perft(position, depth, &fen, expand_branches, compare).await {
                    Ok(_) => (),
                    Err(_) => return,
                }
            }
        },
        Commands::Play {
            fen,
            no_truecolor,
            white_player,
            black_player,
            debug,
        } => {
            tui::start(fen, white_player.into(), black_player.into(), !no_truecolor, debug);
        }
    }
}

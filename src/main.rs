use std::{ffi::OsString, path::PathBuf};

use engine::{board::{zobrist::Zobrist, Board}, color::{Black, White}, eval::Evaluation, game::PlayerType, move_gen::magics, precomp, search::options::SearchOptions};
use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand, ValueEnum};
use engine::game::Game;
use external_uci::ExternalUci;

mod perft;
mod tui;
mod faceoff;


#[derive(Parser)]
#[command(version, author, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Evaluate the given chess position.
    Eval {
        fen: String,

        #[arg(long, short, value_name = "DEPTH", default_value = "4")]
        depth: u16,

        #[arg(long)]
        material: bool,

        #[arg(long)]
        psqt: bool,

        #[arg(long)]
        imbalance: bool,

        #[arg(long)]
        pawns: bool,

        #[arg(long)]
        pieces: bool,

        #[arg(long)]
        mobility: bool,

        #[arg(long)]
        threats: bool,

        #[arg(long)]
        passed: bool,

        #[arg(long)]
        space: bool,

        #[arg(long)]
        king: bool,
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

        /// Whether or not to also run the static evaluation function on each position. This helps bench
        /// the evaluation speed.
        #[arg(long, short)]
        eval: bool,

        #[arg(long, short = 'b')]
        expand_branches: bool,

        #[arg(long, short = 'r')]
        test_recursive: bool,

        /// Compares the expanded branches with data given by stockfish. Requires 
        /// stockfish to be in your PATH. Note that this automatically enables 
        /// `expand-branches` as it is required for this to work. 
        #[arg(long, short)]
        compare: bool,
    },
    /// Launch the TUI.
    Play {
        /// The FEN position to begin the game at.
        #[arg(long, short, value_name = "FEN", default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
        fen: String,

        /// Whether or not to use truecolor.
        #[arg(long, short = 'c')]
        no_truecolor: bool,

        /// The player type of the white player.
        #[arg(long, short, value_name = "PLAYER_TYPE", default_value = "human")]
        white_player: CommandPlayerType,

        /// The player type of the black player.
        #[arg(long, short, value_name = "PLAYER_TYPE", default_value = "human")]
        black_player: CommandPlayerType,

        /// Run the TUI in debug mode.
        #[arg(long, short)]
        debug: bool,
    },
    /// Faceoff against a different chess engine that implements UCI.
    Faceoff {
        /// The path to the other chess engine executable.
        engine_uci: OsString,

        /// The path to a file containing the positions that the engines will play.
        /// The file should contain a list of FEN positions separated by line breaks.
        positions: PathBuf,

        /// Arguments to be passed to the other chess engine.
        #[arg(long)]
        args: Vec<String>,

        /// Time in milliseconds that the engines have to move.
        #[arg(long, short = 't', default_value = "100")]
        movetime: u32,

        #[arg(long, short, value_name = "DISPLAY_METHOD", default_value = "none")]
        display: CommandDisplayMethod,
    },
    /// Launch the UCI.
    Uci,
}


#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
enum CommandDisplayMethod {
    #[default]
    None,
    Tui,
    Gui,
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
            depth:_,
            material,
            psqt,
            imbalance,
            pawns,
            pieces,
            mobility,
            threats,
            passed,
            space,
            king,
        } => {
            precomp::initialize();
            magics::initialize();
            let mut zobrist = Zobrist::new();
            let board = Board::load_position(Some(fen), &mut zobrist);
            let mut eval = Evaluation::new(&board);

            println!("main evaluation: {}", eval.evaluate::<White, Black>());
            println!("mg evaluation: {}", eval.middle_game_eval::<White, Black>());
            println!("eg evaluation: {}", eval.end_game_eval::<White, Black>());

            if material {
                let w = (eval.piece_value_mg::<White, Black>(), eval.piece_value_eg::<White, Black>());
                let b = (eval.piece_value_mg::<Black, White>(), eval.piece_value_eg::<Black, White>());
                println!("material evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("material evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }

            if psqt {
                let w = (eval.psqt_mg::<White, Black>(), eval.psqt_eg::<White, Black>());
                let b = (eval.psqt_mg::<Black, White>(), eval.psqt_eg::<Black, White>());
                println!("psqt evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("psqt evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }

            if imbalance {
                println!("imbalance evaluation (white): {}", eval.imbalance_total::<White, Black>());
                println!("imbalance evaluation (black): {}", eval.imbalance_total::<Black, White>());
            }

            if pawns {
                let w = eval.pawns::<White, Black>();
                let b = eval.pawns::<Black, White>();
                println!("pawns evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("pawns evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }

            if pieces {
                let w = eval.pieces::<White, Black>();
                let b = eval.pieces::<Black, White>();
                println!("pieces evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("pieces evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }

            if mobility {
                let w = eval.mobility_bonus::<White, Black>();
                let b = eval.mobility_bonus::<Black, White>();
                println!("mobility evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("mobility evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            } 

            if threats {
                let w = eval.threats::<White, Black>();
                let b = eval.threats::<Black, White>();
                println!("threats evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("threats evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }

            if passed {
                let w = eval.passed::<White, Black>();
                let b = eval.passed::<Black, White>();
                println!("passed evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("passed evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }

            if space {
                println!("space evaluation (white): {}", eval.space::<White, Black>());
                println!("space evaluation (black): {}", eval.space::<Black, White>());
            }

            if king {
                let w = eval.king::<White, Black>();
                let b = eval.king::<Black, White>();
                println!("king evaluation (white): (mg = {}, eg = {})", w.0, w.1);
                println!("king evaluation (black): (mg = {}, eg = {})", b.0, b.1);
            }
        },
        Commands::Perft {
            position,
            fen: fen_str,
            depth,
            all,
            eval,
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
                let mut game = Game::new(Some(fen), SearchOptions::default(), PlayerType::Human, PlayerType::Human);
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

                perft::test_perft_recursive(&mut game.board, &game.zobrist, &mut game.movegen, depth).await;
            } else if all {
                for i in 1..=depth {
                    match perft::test_perft(position, i, &fen, expand_branches, false, eval).await {
                        Ok(_) => (),
                        Err(_) => return
                    }
                }
            } else {
                if compare { expand_branches = true };
                match perft::test_perft(position, depth, &fen, expand_branches, compare, eval).await {
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
        },
        Commands::Faceoff {
            engine_uci: engine,
            args,
            positions,
            movetime,
            display,
        } => {
            let mut cmd = std::process::Command::new(engine.clone()).args(args.clone()).spawn();
            let opponent = ExternalUci::new_with_args(engine.to_str().unwrap(), args).await.unwrap();
            match &mut cmd {
                Ok(proc) => {
                    proc.kill().unwrap();
                    match faceoff::start(opponent, positions, movetime, display).await {
                        Ok(()) => (),
                        Err(e) => {
                            let err = Cli::command().error(
                                ErrorKind::Io,
                                &e,
                            );
                            let _ = err.print();
                            return
                        }
                    }
                },
                Err(_) => {
                    let err = Cli::command().error(
                        ErrorKind::Io,
                        "engine path executable not found"
                    );
                    let _ = err.print();
                    return;
                }
            };
        },
        Commands::Uci => {
            uci::start();
        }
    }
}

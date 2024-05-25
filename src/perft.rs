use std::{future::Future, pin::Pin, time::Instant};
use crate::Cli;

use clap::{error::ErrorKind, CommandFactory};
use engine::{board::{moves::Move, zobrist::Zobrist, Board}, game::{Game, PlayerType}, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator}, precomp::Precomputed, utils::{fen, representation}};
use external_uci::{ExternalUci, ExternalUciCapable, UciPerftResults};
use termion::color as tcolor;

pub fn movegen_test(
    board: &mut Board,
    zobrist: &Zobrist,
    movegen: &mut MoveGenerator,
    precomp: &Precomputed,
    magics: &MagicBitBoards,
    depth: u16,
) -> u64 {
    if depth == 0 { return 1 };

    movegen.generate_moves(board, precomp, magics, false);
    let mut nodes = 0;

    for m in movegen.moves.clone().into_iter() {
        board.make_move(m, false, zobrist);
        let n = movegen_test(board, zobrist, movegen, precomp, magics, depth - 1);
        nodes += n;
        board.unmake_move(m, false);
    }

    nodes
}

pub fn movegen_test_expand(
    board: &mut Board,
    zobrist: &Zobrist,
    movegen: &mut MoveGenerator,
    precomp: &Precomputed,
    magics: &MagicBitBoards,
    depth: u16,
) -> (u64, Vec<(Move, u64)>) {
    if depth == 0 { return (1, vec![]) };

    movegen.generate_moves(board, precomp, magics, false);
    let mut nodes = 0;
    let move_nodes = vec![(Move::NULL, 0); movegen.moves.len()];

    for m in movegen.moves.clone().into_iter() {
        board.make_move(m, false, zobrist);
        let n = movegen_test_expand(board, zobrist, movegen, precomp, magics, depth - 1).0;
        nodes += n;
        board.unmake_move(m, false);
    }

    (nodes, move_nodes)
}

pub fn parse_compare_data(perfts: Vec<UciPerftResults>) -> Result<Vec<(Move, u64)>, clap::Error> {
    let mut move_nodes = Vec::new();

    for perft in perfts.iter() {
        let m = &perft.m;

        let (start, target) = match (
            representation::coord_from_name(&m[0..2]),
            representation::coord_from_name(&m[2..4])
        ) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                let err = Cli::command().error(
                    ErrorKind::Io,
                    "failed to parse comparison data".to_string(),
                );
                err.print()?;
                return Err(err);
            }
        };

        let flag = if let Some(c) = &m.chars().nth(4) {
            match c {
                'q' | 'Q' => Move::QUEEN_PROMOTION,
                'r' | 'R' => Move::ROOK_PROMOTION,
                'b' | 'B' => Move::BISHOP_PROMOTION,
                'n' | 'N' => Move::KNIGHT_PROMOTION,
                _ => Move::NORMAL,
            }
        } else { Move::NORMAL };

        move_nodes.push((Move::from_start_end_flagged(start.square(), target.square(), flag), perft.nodes));
    }
    
    Ok(move_nodes)
}


pub fn expected_depth(position: u16, depth: u16) -> Result<(), clap::Error> {
    let e = match position {
        1 => if depth > 13 { Some(format!("depth {} does not exist for position {}. valid depths are from 0-13", depth, position)) } else { None },
        2 => if depth > 6 { Some(format!("depth {} does not exist for position {}. valid depths are from 0-6", depth, position)) } else { None },
        3 => if depth > 8 { Some(format!("depth {} does not exist for position {}. valid depths are from 0-8", depth, position)) } else { None },
        4 => if depth > 6 { Some(format!("depth {} does not exist for position {}. valid depths are from 0-6", depth, position)) } else { None },
        5 => if depth > 5 { Some(format!("depth {} does not exist for position {}. valid depths are from 0-5", depth, position)) } else { None },
        6 => if depth > 9 { Some(format!("depth {} does not exist for position {}. valid depths are from 0-9", depth, position)) } else { None },
        _ => None,
    };

    match e {
        Some(s) => {
            let err = Cli::command().error(
                ErrorKind::InvalidValue,
                s
            );
            err.print()?;
            Err(err)
        },
        None => { Ok(()) },
    }
}

pub fn expected_nodes(position: u16, depth: u16) -> u64 {
    match position {
        1 => match depth {
            0 => 1, 
            1 => 20,
            2 => 400,
            3 => 8902,
            4 => 197281,
            5 => 4865609,
            6 => 119060324,
            7 => 3195901860,
            8 => 84998978956,
            9 => 2439530234167,
            10 => 69352859712417,
            11 => 2097651003696806,
            12 => 62854969236701747,
            13 => 1981066775000396239,
            _ => unreachable!(),
        },
        2 => match depth {
            0 => 1, 
            1 => 48,
            2 => 2039,
            3 => 97862,
            4 => 4085603,
            5 => 193690690,
            6 => 8031647685,
            _ => unreachable!(),
        },
        3 => match depth {
            0 => 1,
            1 => 14,
            2 => 191,
            3 => 2812,
            4 => 43238,
            5 => 674624,
            6 => 11030083,
            7 => 178633661,
            8 => 3009794393,
            _ => unreachable!(),
        },
        4 => match depth {
            0 => 1,
            1 => 6,
            2 => 264,
            3 => 9467,
            4 => 422333,
            5 => 15833292,
            6 => 706045033,
            _ => unreachable!(),
        },
        5 => match depth {
            0 => 1,
            1 => 44,
            2 => 1486,
            3 => 62379,
            4 => 2103487,
            5 => 89941194,
            _ => unreachable!(),
        },
        6 => match depth {
            0 => 1,
            1 => 46,
            2 => 2079,
            3 => 89890,
            4 => 3894594,
            5 => 164075551,
            6 => 6923051137,
            7 => 287188994746,
            8 => 11923589843526,
            9 => 490154852788714,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}


pub async fn test_perft(position: u16, depth: u16, fen: &str, expand_branch_nodes: bool, cmp: bool) -> Result<(), clap::Error> {
    let mut game = Game::new(Some(fen.to_string()), PlayerType::Human, PlayerType::Human);
    let expected_nodes = expected_nodes(position, depth);

    let start = Instant::now();
    let (nodes, mut move_nodes) = if expand_branch_nodes {
        movegen_test_expand(&mut game.board, &game.zobrist, &mut game.movegen, &game.precomp, &game.magics, depth)
    } else {
        (movegen_test(&mut game.board, &game.zobrist, &mut game.movegen, &game.precomp, &game.magics, depth), vec![])
    };
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

    if expand_branch_nodes {
        move_nodes.sort_by_key(|(m, _)| {
            (m.start().file() as usize * 8 + m.start().rank() as usize) * 64
                + (m.target().file() as usize * 8 + m.target().file() as usize)
        });
        if cmp {
            let stockfish_values = async {
                if let Ok(mut ext_uci) = ExternalUci::new("stockfish").await {
                    ext_uci.start_uci().await.unwrap();
                    ext_uci.new_game().await.unwrap();
                    ext_uci.set_position(fen).await.unwrap();
                    ext_uci.go_perft(4).await.unwrap();
                    ext_uci.get_perfts_block().await.ok()
                } else {
                    None
                }
            }.await;
            
            let Some(stockfish_values) = stockfish_values else {
                let err = Cli::command().error(
                    ErrorKind::Io,
                    "stockfish executable not found. make sure it is installed and in your PATH"
                );
                err.print()?;
                return Err(err);
            };

            let mut cmp_move_nodes = match parse_compare_data(stockfish_values) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            for (m, n) in move_nodes {
                let Some(matching_idx) = cmp_move_nodes.iter().position(|(m1, _)| m1.same_move_and_prom(m)) else {
                    println!("+++ {}: {}", m.name(), n);
                    continue;
                };
                let (_cmp_m, cmp_n) = cmp_move_nodes.remove(matching_idx);

                if n == cmp_n {
                    println!("    {}: {}", m.name(), n);
                } else {
                    println!("~~~ {}: {} | expected {}", m.name(), n, cmp_n);
                }
            }

            for (m, n) in cmp_move_nodes {
                println!("--- {}: {}", m.name(), n);
            }
        } else {
            for (m, n) in move_nodes {
                println!("    {}: {}", m.name(), n);
            }
        }
    }

    Ok(())
}


pub fn test_perft_recursive<'a>(
    board: &'a mut Board,
    zobrist: &'a Zobrist,
    movegen: &'a mut MoveGenerator,
    precomp: &'a Precomputed,
    magics: &'a MagicBitBoards,
    depth: u16,
) -> Pin<Box<dyn Future<Output = u64> + Send + 'a>> {
    Box::pin(async move {
        if depth == 0 { return 1 };

        let current_fen = fen::fen_from_position(board);
        let stockfish_nodes = async {
            if let Ok(mut ext_uci) = ExternalUci::new("stockfish").await {
                ext_uci.start_uci().await.unwrap();
                ext_uci.new_game().await.unwrap();
                ext_uci.set_position(&current_fen).await.unwrap();
                ext_uci.go_perft(4).await.unwrap();
                ext_uci.get_perfts_block().await.ok()
            } else {
                None
            }
        };

        movegen.generate_moves(board, precomp, magics, false);
        let mut nodes = 0;
        let mut move_nodes = vec![(Move::NULL, 0); movegen.moves.len()];

        for (i, m) in movegen.moves.clone().into_iter().enumerate() {
            board.make_move(m, false, zobrist);
            let n = test_perft_recursive(board, zobrist, movegen, precomp, magics, depth - 1).await;
            nodes += n;
            move_nodes[i] = (m, n);
            board.unmake_move(m, false);
        }

        move_nodes.sort_by_key(|(m, _)| {
            (m.start().file() as usize * 8 + m.start().rank() as usize) * 64
                + (m.target().file() as usize * 8 + m.target().file() as usize)
        });

        let mut expected_nodes = parse_compare_data(stockfish_nodes.await.unwrap()).unwrap();

        for (m, _n) in move_nodes {
            let Some(matching_idx) = expected_nodes.iter().position(|(m1, _)| m1.same_move_and_prom(m)) else {
                println!("    found problematic move {:?} | fen = {}", m, current_fen);
                continue;
            };

            expected_nodes.remove(matching_idx);
        }

        for (m, _n) in expected_nodes {
            println!("    found missing move {:?} | fen = {}", m, current_fen);
        }

        nodes
    })
}

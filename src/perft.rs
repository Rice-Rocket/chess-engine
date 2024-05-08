use clap::{error::ErrorKind, CommandFactory};

use crate::{bitboard::bbutils::BitBoardUtils, board::{zobrist::Zobrist, Board}, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData}, Cli};

pub fn movegen_test(
    board: &mut Board,
    zobrist: &Zobrist,
    movegen: &mut MoveGenerator,
    precomp: &PrecomputedMoveData,
    bbutils: &BitBoardUtils,
    magics: &MagicBitBoards,
    depth: u16,
) -> u64 {
    if depth == 0 { return 1 };

    movegen.generate_moves(board, precomp, bbutils, magics, false);
    let mut n_positions = 0;

    for m in movegen.moves.clone() {
        board.make_move(m, false, zobrist);
        n_positions += movegen_test(board, zobrist, movegen, precomp, bbutils, magics, depth - 1);
        board.unmake_move(m, false);
    }

    n_positions
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

use crate::{board::{piece::*, Board}, utils::representation};


pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn ptype_from_symbol(symbol: &str) -> u8 {
    match symbol {
        "k" => Piece::KING,
        "p" => Piece::PAWN,
        "n" => Piece::KNIGHT,
        "b" => Piece::BISHOP,
        "r" => Piece::ROOK,
        "q" => Piece::QUEEN,
        _ => Piece::NONE
    }
}


pub struct LoadedPositionInfo {
    pub squares: [u8; 64],
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub ep_file: i8,
    pub white_to_move: bool,
    pub fifty_move_ply_count: u8,
    pub move_count: u32,
}

impl LoadedPositionInfo {
    pub fn default() -> Self {
        Self {
            squares: [0; 64],
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            ep_file: 0,
            white_to_move: true,
            fifty_move_ply_count: 0,
            move_count: 0
        }
    }
}


pub fn position_from_fen(fen: String) -> LoadedPositionInfo {
    let mut loaded_pos_info = LoadedPositionInfo::default();
    let sections: Vec<&str> = fen.split(' ').collect();

    let mut file: u32 = 0;
    let mut rank: u32 = 7;

    for symbol in sections[0].chars() {
        if symbol == '/' {
            file = 0;
            rank -= 1;
        } else if symbol.is_ascii_digit() {
            file += symbol.to_digit(10).unwrap();
        } else {
            let pcolor = if symbol.is_uppercase() { Piece::WHITE } else { Piece::BLACK };
            let ptype = ptype_from_symbol(&symbol.to_lowercase().to_string());
            loaded_pos_info.squares[rank as usize * 8 + file as usize] = ptype | pcolor;
            file += 1;
        }
    }

    loaded_pos_info.white_to_move = sections[1] == "w";
    let castling_rights = if sections.len() > 2 { sections[2] } else { "KQkq" };
    loaded_pos_info.white_castle_kingside = castling_rights.contains('K');
    loaded_pos_info.white_castle_queenside = castling_rights.contains('Q');
    loaded_pos_info.black_castle_kingside = castling_rights.contains('k');
    loaded_pos_info.black_castle_queenside = castling_rights.contains('q');

    if sections.len() > 3 {
        let en_passant_filename = sections[3].chars().next().unwrap();
        if representation::FILE_NAMES.contains(en_passant_filename) {
            loaded_pos_info.ep_file = representation::FILE_NAMES.chars().position(|f| f == en_passant_filename).unwrap() as i8 + 1;
        }
    }

    if sections.len() > 4 {
        loaded_pos_info.fifty_move_ply_count = sections[4].parse().unwrap();
    }

    if sections.len() > 5 {
        loaded_pos_info.move_count = sections[5].parse().unwrap();
    }
    loaded_pos_info
}


pub fn fen_from_position(board: &Board) -> String {
    let mut fen = String::from("");

    for rank in (0..8).rev() {
        let mut n_empty_files = 0;
        for file in 0..8 {
            let i = rank * 8 + file;
            let piece = board.square[i as usize];
            if piece != Piece::NULL {
                if n_empty_files != 0 {
                    fen += &format!("{}", n_empty_files);
                    n_empty_files = 0;
                }
                let is_black = piece.is_color(Piece::BLACK);
                let ptype = piece.piece_type();
                let piece_char = match ptype {
                    Piece::PAWN => 'P',
                    Piece::KNIGHT => 'N',
                    Piece::BISHOP => 'B',
                    Piece::ROOK => 'R',
                    Piece::QUEEN => 'Q',
                    Piece::KING => 'K',
                    _ => '!',
                };
                let lower = &piece_char.to_string().to_lowercase();
                let upper = &piece_char.to_string();
                fen += if is_black { lower } else { upper };
            } else {
                n_empty_files += 1;
            }
        }
        if n_empty_files != 0 {
            fen += &format!("{}", n_empty_files);
        }
        if rank != 0 {
            fen += "/";
        }
    }
    
    fen += " ";
    fen += if board.white_to_move { "w" } else { "b" };

    let white_kingside = board.current_state.has_kingside_castle_right(true);
    let white_queenside = board.current_state.has_queenside_castle_right(true);
    let black_kingside = board.current_state.has_kingside_castle_right(false);
    let black_queenside = board.current_state.has_queenside_castle_right(false);

    fen += " ";
    fen += if white_kingside { "K" } else { "" };
    fen += if white_queenside { "Q" } else { "" };
    fen += if black_kingside { "k" } else { "" };
    fen += if black_queenside { "q" } else { "" };
    fen += if !white_kingside && !white_queenside && !black_kingside && !black_queenside { "-" } else { "" };

    fen += " ";
    let ep_file = board.current_state.en_passant_file;
    if ep_file == 0 {
        fen += "-";
    } else {
        let filename = representation::FILE_NAMES.as_bytes()[ep_file as usize - 1].to_string();
        let ep_rank = if board.white_to_move { "6" } else { "3" };
        fen += &(filename + ep_rank);
    }

    fen += " ";
    fen += &format!("{}", board.current_state.fifty_move_counter);

    fen += " ";
    fen += &format!("{}", (board.plycount / 2) + 1);

    fen
}

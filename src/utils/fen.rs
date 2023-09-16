use crate::{board::piece::*, game::representation};


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
    pub ep_file: u32,
    pub white_to_move: bool,
    pub ply_count: u32,
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
            ply_count: 0
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
        } else {
            if symbol.is_digit(10) {
                file += symbol.to_digit(10).unwrap();
            } else {
                let pcolor = if symbol.is_uppercase() { Piece::WHITE } else { Piece::BLACK };
                let ptype = ptype_from_symbol(&symbol.to_lowercase().to_string());
                loaded_pos_info.squares[rank as usize * 8 + file as usize] = ptype | pcolor;
                file += 1;
            }
        }
    }

    loaded_pos_info.white_to_move = sections[1] == "w";
    let castling_rights = if sections.len() > 2 { sections[2] } else { "KQkq" };
    loaded_pos_info.white_castle_kingside = castling_rights.contains("K");
    loaded_pos_info.white_castle_queenside = castling_rights.contains("Q");
    loaded_pos_info.black_castle_kingside = castling_rights.contains("k");
    loaded_pos_info.black_castle_queenside = castling_rights.contains("q");

    if sections.len() > 3 {
        let en_passant_filename = sections[3].chars().nth(0).unwrap();
        if representation::FILE_NAMES.contains(en_passant_filename) {
            loaded_pos_info.ep_file = representation::FILE_NAMES.chars().position(|f| f == en_passant_filename).unwrap() as u32 + 1;
        }
    }

    if sections.len() > 4 {
        loaded_pos_info.ply_count = sections[4].parse().unwrap();
    }
    return loaded_pos_info;
}
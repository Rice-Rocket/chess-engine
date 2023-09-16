use bevy::prelude::*;
use super::{
    piece, piece_list::PieceList,
    moves::{self, Move}, zobrist::Zobrist
};
use crate::{
    game::representation,
    fen,
};


pub const WHITE_INDEX: u32 = 0;
pub const BLACK_INDEX: u32 = 1;

const WHITE_CASTLE_KINGSIDE_MASK: u32 = 0b1111111111111110;
const WHITE_CASTLE_QUEENSIDE_MASK: u32 = 0b1111111111111101;
const BLACK_CASTLE_KINGSIDE_MASK: u32 = 0b1111111111111011;
const BLACK_CASTLE_QUEENSIDE_MASK: u32 = 0b1111111111110111;

const WHITE_CASTLE_MASK: u32 = WHITE_CASTLE_KINGSIDE_MASK & WHITE_CASTLE_QUEENSIDE_MASK;
const BLACK_CASTLE_MASK: u32 = BLACK_CASTLE_KINGSIDE_MASK & BLACK_CASTLE_QUEENSIDE_MASK;


#[derive(Resource, Clone)]
pub struct Board {
    pub square: [u32; 64],
    pub white_to_move: bool,
    pub color_to_move: u32,
    pub opponent_color: u32,
    pub color_to_move_idx: u32,

    game_state_history: Vec<u32>, 
    pub current_game_state: u32,

    pub plycount: u32,
    pub fifty_move_counter: u32,

    pub zobrist_key: u64,
    pub zobrist: Zobrist,
    pub repeat_position_history: Vec<u64>,

    pub king_square: [u32; 2],
    all_pieces: Vec<PieceList>,
}

impl Default for Board {
    fn default() -> Self {
        let knights = vec![PieceList::new(10); 2];
        let pawns = vec![PieceList::new(8); 2];
        let rooks = vec![PieceList::new(10); 2];
        let bishops = vec![PieceList::new(10); 2];
        let queens = vec![PieceList::new(9); 2];
        let empty_list = PieceList::new(0);
        Self {
            square: [0; 64],
            white_to_move: true,
            color_to_move: piece::WHITE,
            opponent_color: piece::BLACK,
            color_to_move_idx: WHITE_INDEX,

            game_state_history: Vec::new(),
            current_game_state: 0,

            plycount: 0,
            fifty_move_counter: 0,

            zobrist_key: 0,
            zobrist: Zobrist::new(),
            repeat_position_history: Vec::new(),

            king_square: [0; 2],
            all_pieces: vec![
                empty_list.clone(),
				empty_list.clone(),
				pawns[WHITE_INDEX as usize].clone(),
				knights[WHITE_INDEX as usize].clone(),
				empty_list.clone(),
				bishops[WHITE_INDEX as usize].clone(),
				rooks[WHITE_INDEX as usize].clone(),
				queens[WHITE_INDEX as usize].clone(),
				empty_list.clone(),
				empty_list.clone(),
				pawns[BLACK_INDEX as usize].clone(),
				knights[BLACK_INDEX as usize].clone(),
				empty_list,
				bishops[BLACK_INDEX as usize].clone(),
				rooks[BLACK_INDEX as usize].clone(),
				queens[BLACK_INDEX as usize].clone(),
            ],
        }
    }
}

impl Board {
    pub fn get_piece_list(&self, ptype: u32, color_idx: u32) -> &PieceList {
        return &self.all_pieces[(color_idx * 8 + ptype) as usize];
    }
    pub fn get_piece_list_mut(&mut self, ptype: u32, color_idx: u32) -> &mut PieceList {
        return &mut self.all_pieces[(color_idx * 8 + ptype) as usize];
    }
}

#[derive(Event)]
pub struct BoardUnmakeMove {
    pub mov: moves::Move,
    pub in_search: bool,
}

#[derive(Event)]
pub struct BoardLoadPosition {
    fen_str: Option<String>,
}


impl Board {
    pub fn make_move(&mut self, mov: Move, in_search: bool) {
        let color_to_move_idx = self.color_to_move_idx;

        let old_en_passant_file = (self.current_game_state >> 4) & 15;
        let original_castle_state = self.current_game_state & 15;
        let mut new_castle_state = original_castle_state;
        self.current_game_state = 0;
    
        let opponent_color_idx: u32 = 1 - self.color_to_move_idx;
        let move_from: u32 = mov.start();
        let move_to: u32 = mov.target();
    
        let captured_ptype = piece::piece_type(self.square[move_to as usize]);
        let move_piece = self.square[move_from as usize];
        let move_ptype = piece::piece_type(move_piece);
    
        let move_flag: u32 = mov.move_flag();
        let is_promotion: bool = mov.is_promotion();
        let is_en_passant: bool = move_flag == moves::EN_PASSANT_CAPTURE;
    
        self.current_game_state |= captured_ptype << 8;
        if captured_ptype != 0 && !is_en_passant {
            self.zobrist_key ^= self.zobrist.pieces_array[move_to as usize][opponent_color_idx as usize][captured_ptype as usize];
            self.get_piece_list_mut(captured_ptype, opponent_color_idx).remove_piece(move_to);
        }
    
        if move_ptype == piece::KING {
            self.king_square[color_to_move_idx as usize] = move_to;
            new_castle_state &= if self.white_to_move { WHITE_CASTLE_MASK } else { BLACK_CASTLE_MASK }
        } else {
            self.get_piece_list_mut(move_ptype, color_to_move_idx).move_piece(move_from, move_to);
        }
    
        let mut piece_on_target_sqr = move_piece;
    
        if is_promotion {
            let promotion_type = match move_flag {
                moves::QUEEN_PROMOTION => {
                    self.get_piece_list_mut(piece::QUEEN, color_to_move_idx).add_piece(move_to);
                    piece::QUEEN
                },
                moves::ROOK_PROMOTION => {
                    self.get_piece_list_mut(piece::ROOK, color_to_move_idx).add_piece(move_to);
                    piece::ROOK
                },
                moves::BISHOP_PROMOTION => {
                    self.get_piece_list_mut(piece::BISHOP, color_to_move_idx).add_piece(move_to);
                    piece::BISHOP
                },
                moves::KNIGHT_PROMOTION => {
                    self.get_piece_list_mut(piece::KNIGHT, color_to_move_idx).add_piece(move_to);
                    piece::KNIGHT
                },
                _ => piece::NONE
            };
            piece_on_target_sqr = promotion_type | self.color_to_move;
            self.get_piece_list_mut(piece::PAWN, color_to_move_idx).remove_piece(move_to);
        } else {
            match move_flag {
                moves::EN_PASSANT_CAPTURE => {
                    let ep_pawn_sqr = move_to + (if self.color_to_move == piece::WHITE { self.square.len() as u32 - 8 } else { 8 });
                    self.current_game_state |= self.square[ep_pawn_sqr as usize] << 8;
                    self.square[ep_pawn_sqr as usize] = 0;
                    self.get_piece_list_mut(piece::PAWN, opponent_color_idx).remove_piece(ep_pawn_sqr);
                    self.zobrist_key ^= self.zobrist.pieces_array[ep_pawn_sqr as usize][opponent_color_idx as usize][piece::PAWN as usize];
                },
                moves::CASTLING => {
                    let kingside = move_to == representation::G1 || move_to == representation::G8;
                    let castle_rook_from_idx = if kingside { move_to + 1 } else { move_to - 2};
                    let castle_rook_to_idx = if kingside { move_to - 1 } else { move_to + 1 };
    
                    self.square[castle_rook_from_idx as usize] = piece::NONE;
                    self.square[castle_rook_to_idx as usize] = piece::ROOK | self.color_to_move;
    
                    self.get_piece_list_mut(piece::ROOK, color_to_move_idx).move_piece(castle_rook_from_idx, castle_rook_to_idx);
                    self.zobrist_key ^= self.zobrist.pieces_array[castle_rook_from_idx as usize][color_to_move_idx as usize][piece::ROOK as usize];
                    self.zobrist_key ^= self.zobrist.pieces_array[castle_rook_to_idx as usize][self.color_to_move_idx as usize][piece::ROOK as usize];
                },
                _ => ()
            }
        }
    
        self.square[move_to as usize] = piece_on_target_sqr;
        self.square[move_from as usize] = 0;
    
        if move_flag == moves::PAWN_TWO_FORWARD {
            let file = representation::file_idx(move_from) + 1;
            self.current_game_state |= file << 4;
            self.zobrist_key ^= self.zobrist.en_passant_file[file as usize];
        }
        
        if original_castle_state != 0 {
            if move_to == representation::H1 || move_from == representation::H1 {
                new_castle_state &= WHITE_CASTLE_KINGSIDE_MASK;
            } else if move_to == representation::A1 || move_from == representation::A1 {
                new_castle_state &= WHITE_CASTLE_QUEENSIDE_MASK;
            }
            if move_to == representation::H8 || move_from == representation::H8 {
                new_castle_state &= BLACK_CASTLE_KINGSIDE_MASK;
            } else if move_to == representation::A8 || move_from == representation::A8 {
                new_castle_state &= BLACK_CASTLE_QUEENSIDE_MASK;
            }
        }
    
        self.zobrist_key ^= self.zobrist.side_to_move;
        self.zobrist_key ^= self.zobrist.pieces_array[move_from as usize][self.color_to_move_idx as usize][move_ptype as usize];
        self.zobrist_key ^= self.zobrist.pieces_array[move_to as usize][self.color_to_move_idx as usize][piece::piece_type(piece_on_target_sqr) as usize];
    
        if old_en_passant_file != 0 {
            self.zobrist_key ^= self.zobrist.en_passant_file[old_en_passant_file as usize];
        }
        if new_castle_state != original_castle_state {
            self.zobrist_key ^= self.zobrist.castling_rights[original_castle_state as usize];
            self.zobrist_key ^= self.zobrist.castling_rights[new_castle_state as usize];
        }
    
        self.current_game_state |= new_castle_state;
        self.current_game_state |= self.fifty_move_counter << 14;
        let self_current_game_state = self.current_game_state;
        self.game_state_history.push(self_current_game_state);
    
        self.white_to_move = !self.white_to_move;
        self.color_to_move = if self.white_to_move { piece::WHITE } else { piece::BLACK };
        self.opponent_color = if self.white_to_move { piece::BLACK } else { piece::WHITE };
        self.color_to_move_idx = 1 - self.color_to_move_idx;
        self.plycount += 1;
        self.fifty_move_counter += 1;
    
        if !in_search {
            if move_ptype == piece::PAWN || captured_ptype != piece::NONE {
                self.repeat_position_history.clear();
                self.fifty_move_counter = 0;
            } else {
                let board_zobrist_key = self.zobrist_key;
                self.repeat_position_history.push(board_zobrist_key);
            }
        }
    }
    
    pub fn unmake_move(&mut self, mov: Move, in_search: bool) {
        let color_to_move_idx = self.color_to_move_idx;

        let opponent_color_idx = self.color_to_move_idx;
        let undoing_white_move = opponent_color_idx == piece::WHITE;
        self.color_to_move = self.opponent_color;
        self.opponent_color = if undoing_white_move { piece::BLACK } else { piece::WHITE };
        self.color_to_move_idx = 1 - self.color_to_move_idx;
        self.white_to_move = !self.white_to_move;
    
        let original_castle_state = self.current_game_state & 0b1111;
        let captured_ptype = (self.current_game_state >> 8) & 63;
        let captured_piece = if captured_ptype == 0 { 0 } else { captured_ptype | self.opponent_color };
    
        let move_from = mov.start();
        let move_to = mov.target();
        let move_flags = mov.move_flag();
        let is_en_passant = move_flags == moves::EN_PASSANT_CAPTURE;
        let is_promotion = mov.is_promotion();
    
        let to_sqr_ptype = piece::piece_type(self.square[move_to as usize]);
        let move_ptype = if is_promotion { piece::PAWN } else { to_sqr_ptype };
    
        self.zobrist_key ^= self.zobrist.side_to_move;
        self.zobrist_key ^= self.zobrist.pieces_array[move_from as usize][self.color_to_move_idx as usize][move_ptype as usize];
        self.zobrist_key ^= self.zobrist.pieces_array[move_to as usize][self.color_to_move_idx as usize][to_sqr_ptype as usize];
    
        let old_en_passant_file = (self.current_game_state >> 4) & 15;
        if old_en_passant_file != 0 {
            self.zobrist_key ^= self.zobrist.en_passant_file[old_en_passant_file as usize];
        }
        if captured_ptype != 0 && !is_en_passant {
            self.zobrist_key ^= self.zobrist.pieces_array[move_to as usize][opponent_color_idx as usize][captured_ptype as usize];
            self.get_piece_list_mut(captured_ptype, opponent_color_idx).add_piece(move_to);
        }
        if move_ptype == piece::KING {
            self.king_square[color_to_move_idx as usize] = move_from;
        } else if !is_promotion {
            self.get_piece_list_mut(move_ptype, color_to_move_idx).move_piece(move_to, move_from);
        }
    
        self.square[move_from as usize] = move_ptype | self.color_to_move;
        self.square[move_to as usize] = captured_piece;
    
        if is_promotion {
            self.get_piece_list_mut(piece::PAWN, color_to_move_idx).add_piece(move_from);
            match move_flags {
                moves::QUEEN_PROMOTION => {
                    self.get_piece_list_mut(piece::QUEEN, color_to_move_idx).remove_piece(move_to);
                },
                moves::KNIGHT_PROMOTION => {
                    self.get_piece_list_mut(piece::KNIGHT, color_to_move_idx).remove_piece(move_to);
                },
                moves::ROOK_PROMOTION => {
                    self.get_piece_list_mut(piece::ROOK, color_to_move_idx).remove_piece(move_to);
                },
                moves::BISHOP_PROMOTION => {
                    self.get_piece_list_mut(piece::BISHOP, color_to_move_idx).remove_piece(move_to);
                },
                _ => ()
            }
        } else if is_en_passant {
            let ep_idx = move_to + (if self.color_to_move == piece::WHITE { self.square.len() as u32 - 8 } else { 8 });
            self.square[move_to as usize] = 0;
            self.square[ep_idx as usize] = captured_piece;
            self.get_piece_list_mut(piece::PAWN, opponent_color_idx).add_piece(ep_idx);
            self.zobrist_key ^= self.zobrist.pieces_array[ep_idx as usize][opponent_color_idx as usize][piece::PAWN as usize];
        } else if move_flags == moves::CASTLING {
            let kingside = move_to == 6 || move_to == 62;
            let castling_rook_from_idx = if kingside { move_to + 1 } else { move_to - 2 };
            let castling_rook_to_idx = if kingside { move_to - 1 } else { move_to + 1 };
    
            self.square[castling_rook_from_idx as usize] = 0;
            self.square[castling_rook_to_idx as usize] = piece::ROOK | self.color_to_move;
            
            self.get_piece_list_mut(piece::ROOK, color_to_move_idx).move_piece(castling_rook_to_idx, castling_rook_from_idx);
            self.zobrist_key ^= self.zobrist.pieces_array[castling_rook_from_idx as usize][color_to_move_idx as usize][piece::ROOK as usize];
            self.zobrist_key ^= self.zobrist.pieces_array[castling_rook_to_idx as usize][self.color_to_move_idx as usize][piece::ROOK as usize];
        }
    
        self.game_state_history.pop();
        self.current_game_state = self.game_state_history.get(self.game_state_history.len() - 1).unwrap().clone();
    
        self.fifty_move_counter = (self.current_game_state & 0b11111111111111111111111111111111) >> 14;
        let new_en_passant_file = (self.current_game_state >> 4) & 15;
        if new_en_passant_file != 0 {
            self.zobrist_key = self.zobrist.en_passant_file[new_en_passant_file as usize];
        }
        let new_castle_state = self.current_game_state & 0b1111;
        if new_castle_state != original_castle_state {
            self.zobrist_key ^= self.zobrist.castling_rights[original_castle_state as usize];
            self.zobrist_key ^= self.zobrist.castling_rights[new_castle_state as usize];
        }
        self.plycount -= 1;
        if !in_search && self.repeat_position_history.len() > 0 {
            self.repeat_position_history.pop();
        }
    }
    
    pub fn load_position(&mut self, fen_str: Option<String>) {
        let loaded_pos = match fen_str {
            Some(str) => fen::position_from_fen(str),
            None => fen::position_from_fen(String::from(fen::START_FEN))
        };

        for sqr_idx in 0u32..64u32 {
            let piece = loaded_pos.squares[sqr_idx as usize];
            self.square[sqr_idx as usize] = piece;
    
            if piece != piece::NONE {
                let ptype = piece::piece_type(piece);
                let pcolor_idx = if piece::is_color(piece, piece::WHITE) { WHITE_INDEX } else { BLACK_INDEX };
                if piece::is_sliding_piece(piece) {
                    if ptype == piece::QUEEN {
                        self.get_piece_list_mut(piece::QUEEN, pcolor_idx).add_piece(sqr_idx);
                    } else if ptype == piece::ROOK {
                        self.get_piece_list_mut(piece::ROOK, pcolor_idx).add_piece(sqr_idx);
                    } else if ptype == piece::BISHOP {
                        self.get_piece_list_mut(piece::BISHOP, pcolor_idx).add_piece(sqr_idx);
                    }
                } else if ptype == piece::KNIGHT {
                    self.get_piece_list_mut(piece::KNIGHT, pcolor_idx).add_piece(sqr_idx);
                } else if ptype == piece::PAWN {
                    self.get_piece_list_mut(piece::PAWN, pcolor_idx).add_piece(sqr_idx);
                } else if ptype == piece::KING {
                    self.king_square[pcolor_idx as usize] = sqr_idx;
                }
            }
        }
    
        self.white_to_move = loaded_pos.white_to_move;
        self.color_to_move = if self.white_to_move { piece::WHITE } else { piece::BLACK };
        self.opponent_color = !self.color_to_move;
        self.color_to_move_idx = if self.white_to_move { 0 } else { 1 };
    
        let white_castle = (if loaded_pos.white_castle_kingside { 1 << 0 } else { 0 }) | (if loaded_pos.white_castle_queenside { 1 << 1 } else { 0 });
        let black_castle = (if loaded_pos.black_castle_kingside { 1 << 2 } else { 0 }) | (if loaded_pos.black_castle_queenside { 1 << 3 } else { 0 });
        let ep_state = loaded_pos.ep_file << 4;
        let initial_game_state = white_castle | black_castle | ep_state;
        self.game_state_history.push(initial_game_state);
        self.current_game_state = initial_game_state;
        self.plycount = loaded_pos.ply_count;
        let board_clone = self.clone();
        self.zobrist_key = self.zobrist.calc_zobrist_key(board_clone);
    }
}

pub fn spawn_main_board(
    mut commands: Commands,
) {
    let loaded_pos = fen::position_from_fen(String::from(fen::START_FEN));
    let empty_piece_list = PieceList::new(0);
    let rooks = vec![PieceList::new(10), PieceList::new(10)];
    let bishops = vec![PieceList::new(10), PieceList::new(10)];
    let queens = vec![PieceList::new(9), PieceList::new(9)];
    let knights = vec![PieceList::new(10), PieceList::new(10)];
    let pawns = vec![PieceList::new(8), PieceList::new(8)];
    let mut board = Board {
        square: [0; 64],
        white_to_move: true,
        color_to_move: 0,
        opponent_color: 0,
        color_to_move_idx: 0,

        game_state_history: Vec::new(), 
        current_game_state: 0,

        plycount: 0,
        fifty_move_counter: 0,

        zobrist_key: 0,
        zobrist: Zobrist::new(),
        repeat_position_history: Vec::new(),

        king_square: [0; 2],
        all_pieces: vec![
            empty_piece_list.clone(),
            empty_piece_list.clone(),
            pawns[WHITE_INDEX as usize].clone(),
            knights[WHITE_INDEX as usize].clone(),
            empty_piece_list.clone(),
            bishops[WHITE_INDEX as usize].clone(),
            rooks[WHITE_INDEX as usize].clone(),
            queens[WHITE_INDEX as usize].clone(),
            empty_piece_list.clone(),
            empty_piece_list.clone(),
            pawns[BLACK_INDEX as usize].clone(),
            knights[BLACK_INDEX as usize].clone(),
            empty_piece_list,
            bishops[BLACK_INDEX as usize].clone(),
            rooks[BLACK_INDEX as usize].clone(),
            queens[BLACK_INDEX as usize].clone(),
        ],
    };

    for sqr_idx in 0u32..64u32 {
        let piece = loaded_pos.squares[sqr_idx as usize];
        board.square[sqr_idx as usize] = piece;

        if piece != piece::NONE {
            let ptype = piece::piece_type(piece);
            let pcolor_idx = if piece::is_color(piece, piece::WHITE) { WHITE_INDEX } else { BLACK_INDEX };
            if piece::is_sliding_piece(piece) {
                if ptype == piece::QUEEN {
                    board.get_piece_list_mut(piece::QUEEN, pcolor_idx).add_piece(sqr_idx);
                } else if ptype == piece::ROOK {
                    board.get_piece_list_mut(piece::ROOK, pcolor_idx).add_piece(sqr_idx);
                } else if ptype == piece::BISHOP {
                    board.get_piece_list_mut(piece::BISHOP, pcolor_idx).add_piece(sqr_idx);
                }
            } else if ptype == piece::KNIGHT {
                board.get_piece_list_mut(piece::KNIGHT, pcolor_idx).add_piece(sqr_idx);
            } else if ptype == piece::PAWN {
                board.get_piece_list_mut(piece::PAWN, pcolor_idx).add_piece(sqr_idx);
            } else if ptype == piece::KING {
                board.king_square[pcolor_idx as usize] = sqr_idx;
            } else {
            }
        }
    }

    board.white_to_move = loaded_pos.white_to_move;
    board.color_to_move = if board.white_to_move { piece::WHITE } else { piece::BLACK };
    board.opponent_color = !board.color_to_move;
    board.color_to_move_idx = if board.white_to_move { 0 } else { 1 };

    let white_castle = (if loaded_pos.white_castle_kingside { 1 << 0 } else { 0 }) | (if loaded_pos.white_castle_queenside { 1 << 1 } else { 0 });
    let black_castle = (if loaded_pos.black_castle_kingside { 1 << 2 } else { 0 }) | (if loaded_pos.black_castle_queenside { 1 << 3 } else { 0 });
    let ep_state = loaded_pos.ep_file << 4;
    let initial_game_state = white_castle | black_castle | ep_state;
    board.game_state_history.push(initial_game_state);
    board.current_game_state = initial_game_state;
    board.plycount = loaded_pos.ply_count;
    let board_clone = board.clone();
    board.zobrist_key = board.zobrist.calc_zobrist_key(board_clone);

    commands.insert_resource(board);
}

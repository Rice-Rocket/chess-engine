#[derive(Clone, Copy)]
pub struct GameState {
    pub captured_ptype: u8,
    pub en_passant_file: i8,
    pub castling_rights: u8,
    pub fifty_move_counter: u8,
    pub zobrist_key: u64,
}

impl GameState {
    pub const CLEAR_WHITE_KINGSIDE_MASK: u8 = 0b1110;
    pub const CLEAR_WHITE_QUEENSIDE_MASK: u8 = 0b1101;
    pub const CLEAR_BLACK_KINGSIDE_MASK: u8 = 0b1011;
    pub const CLEAR_BLACK_QUEENSIDE_MASK: u8 = 0b0111;

    pub fn has_kingside_castle_right(&self, white: bool) -> bool {
        let mask = if white { 1 } else { 4 };
        (self.castling_rights & mask) != 0
    }
    pub fn has_queenside_castle_right(&self, white: bool) -> bool {
        let mask = if white { 2 } else { 8 };
        (self.castling_rights & mask) != 0
    }
}

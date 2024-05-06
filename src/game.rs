use crate::{bitboard::{bbutils::BitBoardUtils, precomp_bits::PrecomputedBits}, board::{zobrist::Zobrist, Board}, move_gen::{magics::MagicBitBoards, move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData}};

pub struct Game {
    pub board: Board,
    pub precomp: PrecomputedMoveData,
    pub bbutils: BitBoardUtils,
    pub zobrist: Zobrist,
    pub magics: MagicBitBoards,
    pub precomp_bits: PrecomputedBits,
    pub movegen: MoveGenerator,
}

impl Game {
    pub fn new() -> Self {
        let precomp = PrecomputedMoveData::new();
        let bbutils = BitBoardUtils::new();
        let mut zobrist = Zobrist::new();
        let board = Board::load_position(None, &mut zobrist);
        let magics = MagicBitBoards::default();
        let precomp_bits = PrecomputedBits::new(&bbutils);
        let movegen = MoveGenerator::default();

        Self {
            board,
            precomp,
            bbutils,
            zobrist,
            magics,
            precomp_bits,
            movegen,
        }
    }
}

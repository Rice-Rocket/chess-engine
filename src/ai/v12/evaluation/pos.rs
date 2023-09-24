use crate::{board::{board::Board, coord::Coord, piece::Piece}, move_gen::{move_generator::MoveGenerator, precomp_move_data::PrecomputedMoveData, bitboard::bb::BitBoard, magics::MagicBitBoards}};
use super::{perspective::Perspective, attack::AttackEvaluationData, material::MaterialEvaluationData};


pub fn sum_sqrs(func: fn(&PositionEvaluation, Perspective, Coord) -> i32, pos: &PositionEvaluation, perspective: Perspective) -> i32 {
    let mut sum = 0;
    for sqr in Coord::iterate_squares() {
        sum += func(pos, perspective, sqr);
    }
    return sum;
}
pub fn total(func: fn(&PositionEvaluation, Perspective) -> i32, pos: &PositionEvaluation) -> i32 {
    func(pos, Perspective::White) - func(pos, Perspective::Black)
}


pub struct PositionEvaluation<'a> {
    pub board: &'a Board,
    pub move_gen: &'a MoveGenerator,
    pub move_data: &'a PrecomputedMoveData,
    pub magic: &'a MagicBitBoards,

    white_pin_rays: BitBoard,
    black_pin_rays: BitBoard,

    attack_data: Option<AttackEvaluationData>,
    material_data: Option<MaterialEvaluationData>,
}

impl <'a>PositionEvaluation<'a> {
    pub fn new(board: &'a Board, move_gen: &'a MoveGenerator, move_data: &'a PrecomputedMoveData, magic: &'a MagicBitBoards) -> Self {
        let mut pos = Self {
            board,
            move_gen,
            move_data,
            magic,
            
            white_pin_rays: BitBoard(0),
            black_pin_rays: BitBoard(0),

            attack_data: None,
            material_data: None,
        };
        pos.initialize();
        pos
    }

    fn initialize(&mut self) {
        if self.move_gen.white_to_move {
            self.white_pin_rays = self.move_gen.pin_rays;
            self.black_pin_rays = self.calc_friendly_attack_data(Perspective::Black);
        } else {
            self.black_pin_rays = self.move_gen.pin_rays;
            self.white_pin_rays = self.calc_friendly_attack_data(Perspective::White);
        }

        self.attack_data = Some(AttackEvaluationData::new(&self.shallow_clone()));
        self.material_data = Some(MaterialEvaluationData::new(&self.shallow_clone()));
    }

    fn calc_friendly_attack_data(&mut self, per: Perspective) -> BitBoard {
        let mut pin_rays = BitBoard(0);
        
        let mut start_dir_idx = 0;
        let mut end_dir_idx = 8;

        // Don't calculate unecessary directions
        if self.board.get_piece_list(Piece::new(Piece::QUEEN | per.other().color())).count() == 0 {
            start_dir_idx = if self.board.get_piece_list(Piece::new(Piece::ROOK | per.other().color())).count() > 0 { 0 } else { 4 };
            end_dir_idx = if self.board.get_piece_list(Piece::new(Piece::BISHOP | per.other().color())).count() > 0 { 8 } else { 4 };
        }

        for dir in start_dir_idx..end_dir_idx {
            let is_diagonal = dir > 3;
            let slider = if is_diagonal { self.enemy_diagonal_sliders(per) } else { self.enemy_orthogonal_sliders(per) };
            if (self.move_data.dir_ray_mask[self.friendly_king_sqr(per).index()][dir] & slider).0 == 0 { continue; }

            let n = self.move_data.num_sqrs_to_edge[self.friendly_king_sqr(per).index()][dir];
            let dir_offset = self.move_data.direction_offsets[dir];
            let mut is_friendly_piece_along_ray = false;
            let mut ray_mask = BitBoard(0);

            for i in 0..n {
                let sqr = self.friendly_king_sqr(per) + dir_offset * (i + 1);
                ray_mask |= sqr.to_bitboard();
                let piece = self.board.square[sqr.index()];

                if piece != Piece::NULL {
                    if piece.is_color(per.color()) {
                        if !is_friendly_piece_along_ray {
                            is_friendly_piece_along_ray = true;
                        } else { break; }
                    } else {
                        if (is_diagonal && piece.is_bishop_or_queen()) || (!is_diagonal && piece.is_rook_or_queen()) {
                            if is_friendly_piece_along_ray {
                                pin_rays |= ray_mask;
                            }
                            break;
                        } else { break; }
                    }
                }
            }
        };

        // let mut opponent_knight_attacks = BitBoard(0);
        // let mut knights = board.piece_bitboards[Piece::new(Piece::KNIGHT | self.enemy_color).index()];
        // let friendly_king_bitboard = board.piece_bitboards[Piece::new(Piece::KING | self.friendly_color).index()];

        // while knights.0 != 0 {
        //     let knight_sqr = knights.pop_lsb();
        //     let knight_attacks = bbutils.knight_attacks[knight_sqr as usize];
        //     opponent_knight_attacks |= knight_attacks;

        //     if (knight_attacks & friendly_king_bitboard).0 != 0 {
        //         self.in_double_check = self.in_check;
        //         self.in_check = true;
        //         self.check_ray_bitmask |= 1 << knight_sqr;
        //     }
        // }

        // let enemy_pawns_bitboard = board.piece_bitboards[Piece::new(Piece::PAWN | self.enemy_color).index()];
        // self.enemy_pawn_attack_map = BitBoardUtils::pawn_attacks(enemy_pawns_bitboard, !self.white_to_move);
        // if self.enemy_pawn_attack_map.contains_square(self.friendly_king_sqr.square()) {
        //     self.in_double_check = self.in_check;
        //     self.in_check = true;
        //     let possible_pawn_attack_origins = if board.white_to_move { bbutils.white_pawn_attacks[self.friendly_king_sqr.index()] } else {
        //         bbutils.black_pawn_attacks[self.friendly_king_sqr.index()]};
        //     let pawn_check_map = enemy_pawns_bitboard & possible_pawn_attack_origins;
        //     self.check_ray_bitmask |= pawn_check_map;
        // }

        // let enemy_king_sqr = board.king_square[self.enemy_idx];
        // self.enemy_attack_map_no_pawns = self.enemy_sliding_attack_map | opponent_knight_attacks | bbutils.king_moves[enemy_king_sqr.index()];
        // self.enemy_attack_map = self.enemy_attack_map_no_pawns | self.enemy_pawn_attack_map;

        // if !self.in_check {
        //     self.check_ray_bitmask = BitBoard::ALL;
        // }

        // println!("{:?}", pin_rays);

        return pin_rays;
    }

    pub fn square(&self, coord: Coord) -> Piece {
        if coord.is_valid() {
            self.board.square[coord.index()]
        } else {
            Piece::new(Piece::OUT_OF_BOUNDS)
        }
    }

    pub fn dist_to_edge(&self, sqr: Coord, dir: usize) -> i8 {
        self.move_data.num_sqrs_to_edge[sqr.index()][dir]
    }

    pub fn all_pieces_bb(&self) -> BitBoard {
        self.board.all_pieces_bitboard
    }
    pub fn friendly_color_bb(&self, per: Perspective) -> BitBoard {
        self.board.color_bitboards[per.color_idx()]
    }
    pub fn enemy_color_bb(&self, per: Perspective) -> BitBoard {
        self.friendly_color_bb(per.other())
    }
    pub fn friendly_piece_bb(&self, per: Perspective, ptype: u8) -> BitBoard {
        self.board.piece_bitboards[Piece::new(ptype | per.color()).index()]
    }
    pub fn enemy_piece_bb(&self, per: Perspective, ptype: u8) -> BitBoard {
        self.friendly_piece_bb(per.other(), ptype)
    }
    pub fn piece_bb(&self, ptype: u8) -> BitBoard {
        self.friendly_piece_bb(Perspective::White, ptype) | self.friendly_piece_bb(Perspective::Black, ptype)
    }

    pub fn friendly_king_sqr(&self, per: Perspective) -> Coord {
        if per.is_color(Piece::WHITE) {
            self.board.king_square[Board::WHITE_INDEX]
        } else {
            self.board.king_square[Board::BLACK_INDEX]
        }
    }

    pub fn friendly_diagonal_sliders(&self, per: Perspective) -> BitBoard {
        if self.board.move_color == per.color() {
            self.board.friendly_diagonal_sliders
        } else {
            self.board.enemy_diagonal_sliders
        }
    }
    pub fn friendly_orthogonal_sliders(&self, per: Perspective) -> BitBoard {
        if self.board.move_color == per.color() {
            self.board.friendly_orthogonal_sliders
        } else {
            self.board.enemy_orthogonal_sliders
        }
    }
    pub fn enemy_diagonal_sliders(&self, per: Perspective) -> BitBoard {
        if self.board.move_color == per.color() {
            self.board.enemy_diagonal_sliders
        } else {
            self.board.friendly_diagonal_sliders
        }
    }
    pub fn enemy_orthogonal_sliders(&self, per: Perspective) -> BitBoard {
        if self.board.move_color == per.color() {
            self.board.enemy_orthogonal_sliders
        } else {
            self.board.friendly_orthogonal_sliders
        }
    }

    /// Returns the rays that pin friendly pieces
    pub fn friendly_pin_rays(&self, per: Perspective) -> BitBoard {
        if per.is_white() {
            self.white_pin_rays
        } else {
            self.black_pin_rays
        }
    }
    /// Returns the rays that pin enemy pieces
    pub fn enemy_pin_rays(&self, per: Perspective) -> BitBoard {
        if per.is_white() {
            self.black_pin_rays
        } else {
            self.white_pin_rays
        }
    }
    /// Returns all pin rays
    pub fn all_pin_rays(&self) -> BitBoard {
        self.white_pin_rays | self.black_pin_rays
    }

    pub fn attack_data(&self) -> &AttackEvaluationData {
        &self.attack_data.as_ref().unwrap()
    }

    pub fn material_data(&self) -> &MaterialEvaluationData {
        &self.material_data.as_ref().unwrap()
    }

    pub fn shallow_clone(&self) -> Self {
        Self {
            board: self.board,
            move_data: self.move_data,
            move_gen: self.move_gen,
            magic: self.magic,
            white_pin_rays: self.white_pin_rays,
            black_pin_rays: self.black_pin_rays,
            attack_data: None,
            material_data: None,
        }
    }
}
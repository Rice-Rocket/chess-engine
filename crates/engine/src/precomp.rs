use crate::{
    board::{Board, coord::*},
    bitboard::bb::BitBoard,
};


pub struct PrecomputedData {
    // Order doesn't matter
    pub align_mask: [[BitBoard; 64]; 64],

    /// A mask of all the squares to the edge of the board from a given square and going in the
    /// given direction.
    pub dir_ray_mask: [[BitBoard; 8]; 64],
    
    pub direction_offsets: [i8; 8],
    pub dir_offsets_2d: [Coord; 8],

    /// The number of squares until we hit the edge of the board, given a direction. 
    ///
    /// Index with [square][direction]
    pub num_sqrs_to_edge: [[i8; 8]; 64],
    
    /// All pseudo-legal night moves from a given square. 
    pub knight_moves: [BitBoard; 64],

    /// All pseudo-legal king moves from a given square.
    pub king_moves: [BitBoard; 64],

    /// The two directions from which a pawn of the given color can attack. 
    pub pawn_attack_dirs: [[Coord; 2]; 2],

    /// All pseudo-legal white pawn moves from a given square.
    pub white_pawn_attacks: [BitBoard; 64],

    /// All pseudo-legal black pawn moves from a given square. 
    pub black_pawn_attacks: [BitBoard; 64],

    pub orthogonal_directions: [BitBoard; 64],
    pub diagonal_directions: [BitBoard; 64],
    
    pub rook_xray_moves: [BitBoard; 64],
    pub bishop_xray_moves: [BitBoard; 64],
    pub queen_xray_moves: [BitBoard; 64],

    /// The manhattan distance between two given squares. 
    pub manhattan_distance: [[u32; 64]; 64],

    /// The distance between two given squares, using the max of the difference between the rank
    /// and file. 
    pub king_distance: [[u32; 64]; 64],

    /// The manhattan distance from the given square to the center of the board.
    pub center_manhattan_distance: [u32; 64],

    pub forward_ranks: [[BitBoard; 8]; 2],
    pub forward_files: [[BitBoard; 64]; 2],
    pub pawn_attack_span: [[BitBoard; 64]; 2],


    pub white_passed_pawn_mask: [BitBoard; 64],
    pub black_passed_pawn_mask: [BitBoard; 64],
    pub white_pawn_support_mask: [BitBoard; 64],
    pub black_pawn_support_mask: [BitBoard; 64],
    pub adjacent_file_mask: [BitBoard; 8],
    pub king_safety_mask: [BitBoard; 64],
    pub white_forward_file_mask: [BitBoard; 64],
    pub black_forward_file_mask: [BitBoard; 64],
    pub triple_file_mask: [BitBoard; 8],
    pub king_ring: [BitBoard; 64],
}

impl PrecomputedData {
    pub const WHITE_KINGSIDE_MASK: BitBoard = BitBoard(1u64 << Coord::F1.index() | 1u64 << Coord::G1.index());
    pub const BLACK_KINGSIDE_MASK: BitBoard = BitBoard(1u64 << Coord::F8.index() | 1u64 << Coord::G8.index());
    pub const WHITE_QUEENSIDE_MASK_2: BitBoard = BitBoard(1u64 << Coord::D1.index() | 1u64 << Coord::C1.index());
    pub const BLACK_QUEENSIDE_MASK_2: BitBoard = BitBoard(1u64 << Coord::D8.index() | 1u64 << Coord::C8.index());
    pub const WHITE_QUEENSIDE_MASK: BitBoard = BitBoard(Self::WHITE_QUEENSIDE_MASK_2.0 | 1u64 << Coord::B1.index());
    pub const BLACK_QUEENSIDE_MASK: BitBoard = BitBoard(Self::BLACK_QUEENSIDE_MASK_2.0 | 1u64 << Coord::B8.index());

    const WHITE: usize = Board::WHITE_INDEX;
    const BLACK: usize = Board::BLACK_INDEX;

    pub fn pawn_attacks(b: BitBoard, is_white: bool) -> BitBoard {
        if is_white {
            return ((b << 9) & !BitBoard::FILE_A) | ((b << 7) & !BitBoard::FILE_H);
        }
        ((b >> 7) & !BitBoard::FILE_A) | ((b >> 9) & !BitBoard::FILE_H)
    }
}

impl Default for PrecomputedData {
    fn default() -> Self {
        Self {
            align_mask: [[BitBoard(0); 64]; 64],
            dir_ray_mask: [[BitBoard(0); 8]; 64],
            direction_offsets: [8, -8, -1, 1, 7, -7, 9, -9],
            dir_offsets_2d: [
                Coord::new(0, 1), Coord::new(0, -1),
                Coord::new(-1, 0), Coord::new(1, 0),
                Coord::new(-1, 1), Coord::new(1, -1),
                Coord::new(1, 1), Coord::new(-1, -1)],
            num_sqrs_to_edge: [[0; 8]; 64],
            knight_moves: [BitBoard(0); 64],
            king_moves: [BitBoard(0); 64],
            pawn_attack_dirs: [[Coord::new(1, 1), Coord::new(-1, 1)], [Coord::new(1, -1), Coord::new(-1, -1)]],
            white_pawn_attacks: [BitBoard(0); 64],
            black_pawn_attacks: [BitBoard(0); 64],
            orthogonal_directions: [BitBoard(0); 64],
            diagonal_directions: [BitBoard(0); 64],
            rook_xray_moves: [BitBoard(0); 64],
            bishop_xray_moves: [BitBoard(0); 64],
            queen_xray_moves: [BitBoard(0); 64],
            manhattan_distance: [[0; 64]; 64],
            king_distance: [[0; 64]; 64],
            center_manhattan_distance: [0; 64],
            forward_ranks: [[BitBoard(0); 8]; 2],
            forward_files: [[BitBoard(0); 64]; 2],
            pawn_attack_span: [[BitBoard(0); 64]; 2],
            white_passed_pawn_mask: [BitBoard(0); 64],
            black_passed_pawn_mask: [BitBoard(0); 64],
            white_pawn_support_mask: [BitBoard(0); 64],
            black_pawn_support_mask: [BitBoard(0); 64],
            adjacent_file_mask: [BitBoard(0); 8],
            king_safety_mask: [BitBoard(0); 64],
            white_forward_file_mask: [BitBoard(0); 64],
            black_forward_file_mask: [BitBoard(0); 64],
            triple_file_mask: [BitBoard(0); 8],
            king_ring: [BitBoard(0); 64],
        }
    }
}

impl PrecomputedData {
    pub fn num_rook_moves_to_sqr(&self, start_sqr: u32, target_sqr: u32) -> u32 {
        self.manhattan_distance[start_sqr as usize][target_sqr as usize]
    }

    pub fn num_king_moves_to_sqr(&self, start_sqr: u32, target_sqr: u32) -> u32 {
        self.king_distance[start_sqr as usize][target_sqr as usize]
    }

    fn valid_index(x: i8, y: i8) -> Option<i8> {
        match (0..8).contains(&x) && (0..8).contains(&y) {
            true => Some(y * 8 + x),
            false => None
        }
    }

    fn calc_num_sqrs_to_edge(&mut self) {
        for sqr_idx in 0..64 {
            let y = sqr_idx / 8;
            let x = sqr_idx - y * 8;
            let sqr = Coord::from_idx(sqr_idx);

            let north = 7 - y;
            let south = y;
            let west = x;
            let east = 7 - x;
            self.num_sqrs_to_edge[sqr_idx as usize][0] = north;
            self.num_sqrs_to_edge[sqr_idx as usize][1] = south;
            self.num_sqrs_to_edge[sqr_idx as usize][2] = west;
            self.num_sqrs_to_edge[sqr_idx as usize][3] = east;
            self.num_sqrs_to_edge[sqr_idx as usize][4] = north.min(west);
            self.num_sqrs_to_edge[sqr_idx as usize][5] = south.min(east);
            self.num_sqrs_to_edge[sqr_idx as usize][6] = north.min(east);
            self.num_sqrs_to_edge[sqr_idx as usize][7] = south.min(west);
        }
    }

    fn calc_pseudo_legal_moves(&mut self) {
        let all_knight_jumps: [i8; 8] = [15, 17, -17, -15, 10, -6, 6, -10];
        let ortho_dir: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let diag_dir: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
        let knight_jumps: [(i8, i8); 8] = [(-2, -1), (-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2)];

        for sqr_idx in 0..64 {
            let y = sqr_idx / 8;
            let x = sqr_idx - y * 8;
            let sqr = Coord::from_idx(sqr_idx);

            for dir_idx in 0..4 {
                let ortho_x = x + ortho_dir[dir_idx].0;
                let ortho_y = y + ortho_dir[dir_idx].1;
                let diag_x = x + diag_dir[dir_idx].0;
                let diag_y = y + diag_dir[dir_idx].1;

                if let Some(ortho_target_idx) = Self::valid_index(ortho_x, ortho_y) {
                    self.king_moves[sqr] |= 1 << ortho_target_idx;
                }
                if let Some(diag_target_idx) = Self::valid_index(diag_x, diag_y) {
                    self.king_moves[sqr] |= 1 << diag_target_idx;
                }

                for knight_jump in knight_jumps.iter() {
                    let knight_x = x + knight_jump.0;
                    let knight_y = y + knight_jump.1;
                    if let Some(knight_target_idx) = Self::valid_index(knight_x, knight_y) {
                        self.knight_moves[sqr] |= 1 << knight_target_idx;
                    }
                }

                if let Some(white_pawn_right) = Self::valid_index(x + 1, y + 1) {
                    self.white_pawn_attacks[sqr] |= 1 << white_pawn_right;
                }
                if let Some(white_pawn_left) = Self::valid_index(x - 1, y + 1) {
                    self.white_pawn_attacks[sqr] |= 1 << white_pawn_left;
                }
                if let Some(black_pawn_right) = Self::valid_index(x + 1, y - 1) {
                    self.black_pawn_attacks[sqr] |= 1 << black_pawn_right;
                }
                if let Some(black_pawn_left) = Self::valid_index(x - 1, y - 1) {
                    self.black_pawn_attacks[sqr] |= 1 << black_pawn_left;
                }
            }

            for (direction_idx, cur_dir_offset) in self.direction_offsets.iter().enumerate().take(4) {
                for n in 0..self.num_sqrs_to_edge[sqr_idx as usize][direction_idx] {
                    let target_sqr = sqr_idx + cur_dir_offset * (n + 1);
                    self.rook_xray_moves[sqr_idx as usize] |= 1u64 << target_sqr;
                }
            }

            for (direction_idx, cur_dir_offset) in self.direction_offsets.iter().enumerate().skip(4) {
                for n in 0..self.num_sqrs_to_edge[sqr_idx as usize][direction_idx] {
                    let target_sqr = sqr_idx + cur_dir_offset * (n + 1);
                    self.bishop_xray_moves[sqr_idx as usize] |= 1u64 << target_sqr;
                }
            }

            self.queen_xray_moves[sqr_idx as usize] = self.rook_xray_moves[sqr_idx as usize] | self.bishop_xray_moves[sqr_idx as usize];
        }
    }

    fn calc_relative_directions(&mut self) {
        for sqr in Coord::iter_squares() {
            for (i, dir) in self.dir_offsets_2d.iter().enumerate() {
                let ortho = i < 4;
                let s = Coord::new_unchecked(sqr.file() + dir.file(), sqr.rank() + dir.rank());
                if s.is_valid() {
                    if ortho {
                        self.orthogonal_directions[sqr] |= s.to_bitboard();
                    } else {
                        self.diagonal_directions[sqr] |= s.to_bitboard();
                    }
                }
            }
        }
    }

    fn calc_manhattan_distance(&mut self) {
        for sqr_a in Coord::iter_squares() {
            let file_center_dst = (3 - sqr_a.file() as i32).max(sqr_a.file() as i32 - 4) as u32;
            let rank_center_dst = (3 - sqr_a.rank() as i32).max(sqr_a.rank() as i32 - 4) as u32;
            self.center_manhattan_distance[sqr_a] = file_center_dst + rank_center_dst;
            for sqr_b in Coord::iter_squares() {
                let file_dst = (sqr_a.file() as i32 - sqr_b.file() as i32).abs();
                let rank_dst = (sqr_a.rank() as i32 - sqr_b.rank() as i32).abs();
                self.manhattan_distance[sqr_a][sqr_b] = (file_dst + rank_dst) as u32;
                self.king_distance[sqr_a][sqr_b] = file_dst.max(rank_dst) as u32;
            }
        };
    }

    fn calc_align_mask(&mut self) {
        for sqr_a in Coord::iter_squares() {
            for sqr_b in Coord::iter_squares() {
                let delta = sqr_a.delta(sqr_b);
                let dir = Coord::new(delta.file().signum(), delta.rank().signum());
                for i in -8..8 {
                    let c = sqr_a + dir * i;
                    if c.is_valid() {
                        self.align_mask[sqr_a][sqr_b] |= 1 << c.index();
                    }
                }
            }
        }
    }

    fn calc_dir_ray_mask(&mut self) {
        for (dir_idx, offset_2d) in self.dir_offsets_2d.iter().enumerate() {
            for sqr in Coord::iter_squares() {
                for i in 0..8 {
                    let c = sqr + *offset_2d * i;
                    if c.is_valid() {
                        self.dir_ray_mask[sqr][dir_idx] |= 1 << c.index();
                    }
                }
            }
        }
    }

    fn calc_pawn_structure_masks(&mut self) {
        for i in 0..8 {
            let left = if i > 0 { BitBoard::FILE_A << (i - 1) } else { BitBoard(0) };
            let right = if i < 7 { BitBoard::FILE_A << (i + 1) } else { BitBoard(0) };
            self.adjacent_file_mask[i] = left | right;
        };

        for (i, mask) in self.triple_file_mask.iter_mut().enumerate() {
            let clamped_file = i.clamp(1, 6);
            *mask = BitBoard::FILES[clamped_file] | self.adjacent_file_mask[clamped_file];
        }

        for sqr in Coord::iter_squares() {
            let file = sqr.file();
            let rank = sqr.rank();
            let adjacent_files = BitBoard::FILE_A.0 << (file - 1).max(0) | BitBoard::FILE_A.0 << (file + 1).max(7);

            let white_forward_mask = BitBoard(!(u64::MAX >> (64 - 8 * (rank + 1))));
            let black_forward_mask = BitBoard((1 << (8 * rank)) - 1);

            self.white_passed_pawn_mask[sqr] = (BitBoard::FILE_A << file as usize | adjacent_files) & white_forward_mask;
            self.black_passed_pawn_mask[sqr] = (BitBoard::FILE_A << file as usize | adjacent_files) & black_forward_mask;

            let adjacent = ((
                if sqr.index() == 0 { BitBoard(0) } else { BitBoard(1 << (sqr.index() - 1)) }
            ) | (if sqr.index() == 63 { 0 } else { 1 << (sqr.index() + 1) })) & adjacent_files;
            self.white_pawn_support_mask[sqr] = adjacent | adjacent.shifted(-8);
            self.black_pawn_support_mask[sqr] = adjacent | adjacent.shifted(8);

            self.white_forward_file_mask[sqr] = white_forward_mask & BitBoard::FILES[file as usize];
            self.black_forward_file_mask[sqr] = black_forward_mask & BitBoard::FILES[rank as usize];
        };

        for (i, mask) in self.king_safety_mask.iter_mut().enumerate() {
            *mask = self.king_moves[i] | (1 << i);
        }
    }

    fn calc_king_ring(&mut self) {
        for sqr in Coord::iter_squares() {
            for ix in -2..=2 {
                for iy in -2..=2 {
                    if ((-1..=1).contains(&ix) || sqr.file() == 0 || sqr.file() == 7)
                    && ((-1..=1).contains(&iy) || sqr.rank() == 0 || sqr.rank() == 7) {
                        self.king_ring[sqr].set_square(Coord::new_clamp(sqr.file() + ix, sqr.rank() + iy).square());
                    }
                }
            }
        }
    }
    
    fn calc_forward_ranks_files_span(&mut self) {
        for r in 0..7 {
            self.forward_ranks[Self::BLACK][r + 1] = self.forward_ranks[Self::BLACK][r] | BitBoard::RANKS[r];
            self.forward_ranks[Self::WHITE][r] = !self.forward_ranks[Self::BLACK][r + 1];
        }

        for c in 0..2 {
            for s in Coord::iter_squares() {
                self.forward_files[c][s] = self.forward_ranks[c][s.rank() as usize] & BitBoard::FILES[s.file() as usize];
                self.pawn_attack_span[c][s] = self.forward_ranks[c][s.rank() as usize] & self.adjacent_file_mask[s.file() as usize];
            }
        }
    }

    pub fn new() -> Self {
        let mut data = Self::default();

        data.calc_num_sqrs_to_edge();
        data.calc_pseudo_legal_moves();
        data.calc_relative_directions();
        data.calc_manhattan_distance();
        data.calc_dir_ray_mask();
        data.calc_align_mask();
        data.calc_pawn_structure_masks();
        data.calc_king_ring();
        data.calc_forward_ranks_files_span();

        data
    }
}

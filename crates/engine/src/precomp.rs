use crate::{
    board::{Board, coord::*},
    bitboard::bb::BitBoard,
};

// Order doesn't matter
static mut ALIGN_MASK: [[BitBoard; 64]; 64] = [[BitBoard(0); 64]; 64];

/// A mask of all the squares to the edge of the board from a given square and going in the
/// given direction.
static mut DIR_RAY_MASK: [[BitBoard; 8]; 64] = [[BitBoard(0); 8]; 64];

const DIRECTION_OFFSETS: [i8; 8] = [8, -8, -1, 1, 7, -7, 9, -9];
const DIR_OFFSETS_2D: [Coord; 8] = [
    Coord::new(0, 1), Coord::new(0, -1),
    Coord::new(-1, 0), Coord::new(1, 0),
    Coord::new(-1, 1), Coord::new(1, -1),
    Coord::new(1, 1), Coord::new(-1, -1)
];

/// The number of squares until we hit the edge of the board, given a direction. 
///
/// Index with [square][direction]
static mut NUM_SQRS_TO_EDGE: [[i8; 8]; 64] = [[0; 8]; 64];

/// All pseudo-legal night moves from a given square. 
static mut KNIGHT_MOVES: [BitBoard; 64] = [BitBoard(0); 64];

/// All pseudo-legal king moves from a given square.
static mut KING_MOVES: [BitBoard; 64] = [BitBoard(0); 64];

/// The two directions from which a pawn of the given color can attack. 
const PAWN_ATTACK_DIRS: [[Coord; 2]; 2] = [[Coord::new(1, 1), Coord::new(-1, 1)], [Coord::new(1, -1), Coord::new(-1, -1)]];

/// All pseudo-legal white pawn moves from a given square.
static mut WHITE_PAWN_ATTACKS: [BitBoard; 64] = [BitBoard(0); 64];

/// All pseudo-legal black pawn moves from a given square. 
static mut BLACK_PAWN_ATTACKS: [BitBoard; 64] = [BitBoard(0); 64];

static mut ORTHOGONAL_DIRECTIONS: [BitBoard; 64] = [BitBoard(0); 64];
static mut DIAGONAL_DIRECTIONS: [BitBoard; 64] = [BitBoard(0); 64];

static mut ROOK_XRAY_MOVES: [BitBoard; 64] = [BitBoard(0); 64];
static mut BISHOP_XRAY_MOVES: [BitBoard; 64] = [BitBoard(0); 64];
static mut QUEEN_XRAY_MOVES: [BitBoard; 64] = [BitBoard(0); 64];

/// The manhattan distance between two given squares. 
static mut MANHATTAN_DISTANCE: [[u32; 64]; 64] = [[0; 64]; 64];

/// The distance between two given squares, using the max of the difference between the rank
/// and file. 
static mut KING_DISTANCE: [[u32; 64]; 64] = [[0; 64]; 64];

/// The manhattan distance from the given square to the center of the board.
static mut CENTER_MANHATTAN_DISTANCE: [u32; 64] = [0; 64];

static mut FORWARD_RANKS: [[BitBoard; 8]; 2] = [[BitBoard(0); 8]; 2];
static mut FORWARD_FILES: [[BitBoard; 64]; 2] = [[BitBoard(0); 64]; 2];
static mut PAWN_ATTACK_SPAN: [[BitBoard; 64]; 2] = [[BitBoard(0); 64]; 2];

static mut DIAGONAL_SQUARES: [[BitBoard; 64]; 8] = [[BitBoard(0); 64]; 8];
static mut ORTHOGONAL_SQUARES: [[BitBoard; 64]; 8] = [[BitBoard(0); 64]; 8];

static mut WHITE_PASSED_PAWN_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut BLACK_PASSED_PAWN_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut WHITE_PAWN_SUPPORT_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut BLACK_PAWN_SUPPORT_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut ADJACENT_FILE_MASK: [BitBoard; 8] = [BitBoard(0); 8];
static mut KING_SAFETY_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut WHITE_FORWARD_FILE_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut BLACK_FORWARD_FILE_MASK: [BitBoard; 64] = [BitBoard(0); 64];
static mut TRIPLE_FILE_MASK: [BitBoard; 8] = [BitBoard(0); 8];
static mut KING_RING: [BitBoard; 64] = [BitBoard(0); 64];


fn valid_index(x: i8, y: i8) -> Option<i8> {
    match (0..8).contains(&x) && (0..8).contains(&y) {
        true => Some(y * 8 + x),
        false => None
    }
}

fn calc_num_sqrs_to_edge() {
    for sqr_idx in 0..64 {
        let y = sqr_idx / 8;
        let x = sqr_idx - y * 8;
        let sqr = Coord::from_idx(sqr_idx);

        let north = 7 - y;
        let south = y;
        let west = x;
        let east = 7 - x;

        unsafe {
            NUM_SQRS_TO_EDGE[sqr_idx as usize][0] = north;
            NUM_SQRS_TO_EDGE[sqr_idx as usize][1] = south;
            NUM_SQRS_TO_EDGE[sqr_idx as usize][2] = west;
            NUM_SQRS_TO_EDGE[sqr_idx as usize][3] = east;
            NUM_SQRS_TO_EDGE[sqr_idx as usize][4] = north.min(west);
            NUM_SQRS_TO_EDGE[sqr_idx as usize][5] = south.min(east);
            NUM_SQRS_TO_EDGE[sqr_idx as usize][6] = north.min(east);
            NUM_SQRS_TO_EDGE[sqr_idx as usize][7] = south.min(west);
        }
    }
}

fn calc_pseudo_legal_moves() {
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

            if let Some(ortho_target_idx) = valid_index(ortho_x, ortho_y) {
                unsafe { KING_MOVES[sqr] |= 1 << ortho_target_idx };
            }
            if let Some(diag_target_idx) = valid_index(diag_x, diag_y) {
                unsafe { KING_MOVES[sqr] |= 1 << diag_target_idx };
            }

            for knight_jump in knight_jumps.iter() {
                let knight_x = x + knight_jump.0;
                let knight_y = y + knight_jump.1;
                if let Some(knight_target_idx) = valid_index(knight_x, knight_y) {
                    unsafe { KNIGHT_MOVES[sqr] |= 1 << knight_target_idx };
                }
            }

            if let Some(white_pawn_right) = valid_index(x + 1, y + 1) {
                unsafe { WHITE_PAWN_ATTACKS[sqr] |= 1 << white_pawn_right };
            }
            if let Some(white_pawn_left) = valid_index(x - 1, y + 1) {
                unsafe { WHITE_PAWN_ATTACKS[sqr] |= 1 << white_pawn_left };
            }
            if let Some(black_pawn_right) = valid_index(x + 1, y - 1) {
                unsafe { BLACK_PAWN_ATTACKS[sqr] |= 1 << black_pawn_right };
            }
            if let Some(black_pawn_left) = valid_index(x - 1, y - 1) {
                unsafe { BLACK_PAWN_ATTACKS[sqr] |= 1 << black_pawn_left };
            }
        }

        for (direction_idx, cur_dir_offset) in DIRECTION_OFFSETS.iter().enumerate().take(4) {
            for n in 0..unsafe { NUM_SQRS_TO_EDGE[sqr_idx as usize][direction_idx] } {
                let target_sqr = sqr_idx + cur_dir_offset * (n + 1);
                unsafe { ROOK_XRAY_MOVES[sqr_idx as usize] |= 1u64 << target_sqr };
            }
        }

        for (direction_idx, cur_dir_offset) in DIRECTION_OFFSETS.iter().enumerate().skip(4) {
            for n in 0..unsafe { NUM_SQRS_TO_EDGE[sqr_idx as usize][direction_idx] } {
                let target_sqr = sqr_idx + cur_dir_offset * (n + 1);
                unsafe { BISHOP_XRAY_MOVES[sqr_idx as usize] |= 1u64 << target_sqr };
            }
        }

        unsafe { QUEEN_XRAY_MOVES[sqr_idx as usize] = ROOK_XRAY_MOVES[sqr_idx as usize] | BISHOP_XRAY_MOVES[sqr_idx as usize] };
    }
}

fn calc_relative_directions() {
    for sqr in Coord::iter_squares() {
        for (i, dir) in DIR_OFFSETS_2D.iter().enumerate() {
            let ortho = i < 4;
            let s = Coord::new_unchecked(sqr.file() + dir.file(), sqr.rank() + dir.rank());
            if s.is_valid() {
                if ortho {
                    unsafe { ORTHOGONAL_DIRECTIONS[sqr] |= s.to_bitboard() };
                } else {
                    unsafe { DIAGONAL_DIRECTIONS[sqr] |= s.to_bitboard() };
                }
            }
        }
    }
}

fn calc_manhattan_distance() {
    for sqr_a in Coord::iter_squares() {
        let file_center_dst = (3 - sqr_a.file() as i32).max(sqr_a.file() as i32 - 4) as u32;
        let rank_center_dst = (3 - sqr_a.rank() as i32).max(sqr_a.rank() as i32 - 4) as u32;
        unsafe { CENTER_MANHATTAN_DISTANCE[sqr_a] = file_center_dst + rank_center_dst };
        for sqr_b in Coord::iter_squares() {
            let file_dst = (sqr_a.file() as i32 - sqr_b.file() as i32).abs();
            let rank_dst = (sqr_a.rank() as i32 - sqr_b.rank() as i32).abs();
            unsafe { MANHATTAN_DISTANCE[sqr_a][sqr_b] = (file_dst + rank_dst) as u32 };
            unsafe { KING_DISTANCE[sqr_a][sqr_b] = file_dst.max(rank_dst) as u32 };
        }
    };
}

fn calc_align_mask() {
    for sqr_a in Coord::iter_squares() {
        for sqr_b in Coord::iter_squares() {
            let delta = sqr_a.delta(sqr_b);
            let dir = Coord::new(delta.file().signum(), delta.rank().signum());
            for i in -8..8 {
                let c = sqr_a + dir * i;
                if c.is_valid() {
                    unsafe { ALIGN_MASK[sqr_a][sqr_b] |= 1 << c.index() };
                }
            }
        }
    }
}

fn calc_dir_ray_mask() {
    for (dir_idx, offset_2d) in DIR_OFFSETS_2D.iter().enumerate() {
        for sqr in Coord::iter_squares() {
            for i in 0..8 {
                let c = sqr + *offset_2d * i;
                if c.is_valid() {
                    unsafe { DIR_RAY_MASK[sqr][dir_idx] |= 1 << c.index() };
                }
            }
        }
    }
}

fn calc_pawn_structure_masks() {
    for (i, mask) in unsafe { ADJACENT_FILE_MASK.iter_mut().enumerate() } {
        let left = if i > 0 { BitBoard::FILE_A << (i - 1) } else { BitBoard(0) };
        let right = if i < 7 { BitBoard::FILE_A << (i + 1) } else { BitBoard(0) };
        *mask = left | right;
    };

    for (i, mask) in unsafe { TRIPLE_FILE_MASK.iter_mut().enumerate() } {
        let clamped_file = i.clamp(1, 6);
        *mask = BitBoard::FILES[clamped_file] | unsafe { ADJACENT_FILE_MASK[clamped_file] };
    }

    for sqr in Coord::iter_squares() {
        let file = sqr.file();
        let rank = sqr.rank();
        let adjacent_files = BitBoard::FILE_A.0 << (file - 1).max(0) | BitBoard::FILE_A.0 << (file + 1).max(7);

        let white_forward_mask = BitBoard(!(u64::MAX >> (64 - 8 * (rank + 1))));
        let black_forward_mask = BitBoard((1 << (8 * rank)) - 1);

        unsafe {
            WHITE_PASSED_PAWN_MASK[sqr] = (BitBoard::FILE_A << file as usize | adjacent_files) & white_forward_mask;
            BLACK_PASSED_PAWN_MASK[sqr] = (BitBoard::FILE_A << file as usize | adjacent_files) & black_forward_mask;
        }

        let adjacent = ((
            if sqr.index() == 0 { BitBoard(0) } else { BitBoard(1 << (sqr.index() - 1)) }
        ) | (if sqr.index() == 63 { 0 } else { 1 << (sqr.index() + 1) })) & adjacent_files;

        unsafe {
            WHITE_PAWN_SUPPORT_MASK[sqr] = adjacent | adjacent.shifted(-8);
            BLACK_PAWN_SUPPORT_MASK[sqr] = adjacent | adjacent.shifted(8);

            WHITE_FORWARD_FILE_MASK[sqr] = white_forward_mask & BitBoard::FILES[file as usize];
            BLACK_FORWARD_FILE_MASK[sqr] = black_forward_mask & BitBoard::FILES[rank as usize];
        }
    };

    for (i, mask) in unsafe { KING_SAFETY_MASK.iter_mut().enumerate() } {
        *mask = unsafe { KING_MOVES[i] | (1 << i) };
    }
}

fn calc_king_ring() {
    for sqr in Coord::iter_squares() {
        for ix in -2..=2 {
            for iy in -2..=2 {
                if ((-1..=1).contains(&ix) || sqr.file() == 0 || sqr.file() == 7)
                && ((-1..=1).contains(&iy) || sqr.rank() == 0 || sqr.rank() == 7) {
                    unsafe { KING_RING[sqr].set_square(Coord::new_clamp(sqr.file() + ix, sqr.rank() + iy).square()) };
                }
            }
        }
    }
}

fn calc_forward_ranks_files_span() {
    for r in 0..7 {
        unsafe {
            FORWARD_RANKS[Board::BLACK_INDEX][r + 1] = FORWARD_RANKS[Board::BLACK_INDEX][r] | BitBoard::RANKS[r];
            FORWARD_RANKS[Board::WHITE_INDEX][r] = !FORWARD_RANKS[Board::BLACK_INDEX][r + 1];
        }
    }

    for c in 0..2 {
        for s in Coord::iter_squares() {
            unsafe {
                FORWARD_FILES[c][s] = FORWARD_RANKS[c][s.rank() as usize] & BitBoard::FILES[s.file() as usize];
                PAWN_ATTACK_SPAN[c][s] = FORWARD_RANKS[c][s.rank() as usize] & ADJACENT_FILE_MASK[s.file() as usize];
            }
        }
    }
}

fn calc_diagonal_orthogonal_sqrs() {
    for sqr in Coord::iter_squares() {
        for depth in 1..=8 {
            for (dir, offset) in DIRECTION_OFFSETS.iter().enumerate() {
                let is_diagonal = dir > 3;
                let n = unsafe { NUM_SQRS_TO_EDGE[sqr][dir].min(depth) };
                for i in 0..n {
                    let s = sqr + offset * (i + 1);
                    if is_diagonal {
                        unsafe { DIAGONAL_SQUARES[depth as usize - 1][sqr] |= s.to_bitboard() };
                    } else {
                        unsafe { ORTHOGONAL_SQUARES[depth as usize - 1][sqr] |= s.to_bitboard() };
                    }
                }
            }
        }
    }
}

pub fn initialize() {
    calc_num_sqrs_to_edge();
    calc_pseudo_legal_moves();
    calc_relative_directions();
    calc_manhattan_distance();
    calc_dir_ray_mask();
    calc_align_mask();
    calc_pawn_structure_masks();
    calc_king_ring();
    calc_forward_ranks_files_span();
    calc_diagonal_orthogonal_sqrs();
}


pub struct Precomputed;

impl Precomputed {
    pub const WHITE_KINGSIDE_MASK: BitBoard = BitBoard(1u64 << Coord::F1.index() | 1u64 << Coord::G1.index());
    pub const BLACK_KINGSIDE_MASK: BitBoard = BitBoard(1u64 << Coord::F8.index() | 1u64 << Coord::G8.index());
    pub const WHITE_QUEENSIDE_MASK_2: BitBoard = BitBoard(1u64 << Coord::D1.index() | 1u64 << Coord::C1.index());
    pub const BLACK_QUEENSIDE_MASK_2: BitBoard = BitBoard(1u64 << Coord::D8.index() | 1u64 << Coord::C8.index());
    pub const WHITE_QUEENSIDE_MASK: BitBoard = BitBoard(Self::WHITE_QUEENSIDE_MASK_2.0 | 1u64 << Coord::B1.index());
    pub const BLACK_QUEENSIDE_MASK: BitBoard = BitBoard(Self::BLACK_QUEENSIDE_MASK_2.0 | 1u64 << Coord::B8.index());

    const WHITE: usize = Board::WHITE_INDEX;
    const BLACK: usize = Board::BLACK_INDEX;

    #[inline]
    pub fn pawn_attacks(b: BitBoard, is_white: bool) -> BitBoard {
        if is_white {
            return ((b << 9) & !BitBoard::FILE_A) | ((b << 7) & !BitBoard::FILE_H);
        }
        ((b >> 7) & !BitBoard::FILE_A) | ((b >> 9) & !BitBoard::FILE_H)
    }

    #[inline]
    pub fn num_rook_moves_to_sqr(start_sqr: u32, target_sqr: u32) -> u32 {
        unsafe { MANHATTAN_DISTANCE[start_sqr as usize][target_sqr as usize] }
    }

    #[inline]
    pub fn num_king_moves_to_sqr(start_sqr: u32, target_sqr: u32) -> u32 {
        unsafe { KING_DISTANCE[start_sqr as usize][target_sqr as usize] }
    }

    // Order doesn't matter
    #[inline]
    pub fn align_mask(a: Coord, b: Coord) -> BitBoard {
        unsafe { ALIGN_MASK[a][b] }
    }

    /// A mask of all the squares to the edge of the board from a given square and going in the
    /// given direction.
    #[inline]
    pub fn dir_ray_mask(sqr: Coord, dir: usize) -> BitBoard {
        unsafe { DIR_RAY_MASK[sqr][dir] }
    }

    #[inline]
    pub fn direction_offsets(d: usize) -> i8 {
        DIRECTION_OFFSETS[d]
    }

    #[inline]
    pub fn dir_offsets_2d(d: usize) -> Coord {
        DIR_OFFSETS_2D[d]
    }

    /// The number of squares until we hit the edge of the board, given a direction. 
    ///
    /// Index with [square][direction]
    #[inline]
    pub fn num_sqrs_to_edge(sqr: Coord, dir: usize) -> i8 {
        unsafe { NUM_SQRS_TO_EDGE[sqr][dir] }
    }

    /// All pseudo-legal night moves from a given square. 
    #[inline]
    pub fn knight_moves(s: Coord) -> BitBoard {
        unsafe { KNIGHT_MOVES[s] }
    }

    /// All pseudo-legal king moves from a given square.
    #[inline]
    pub fn king_moves(s: Coord) -> BitBoard {
        unsafe { KING_MOVES[s] }
    }

    /// The two directions from which a pawn of the given color can attack. 
    #[inline]
    pub fn pawn_attack_dirs(color: usize) -> [Coord; 2] {
        PAWN_ATTACK_DIRS[color]
    }

    /// All pseudo-legal white pawn moves from a given square.
    #[inline]
    pub fn white_pawn_attacks(s: Coord) -> BitBoard {
        unsafe { WHITE_PAWN_ATTACKS[s] }
    }

    /// All pseudo-legal black pawn moves from a given square. 
    #[inline]
    pub fn black_pawn_attacks(s: Coord) -> BitBoard {
        unsafe { BLACK_PAWN_ATTACKS[s] }
    }

    #[inline]
    pub fn orthogonal_directions(s: Coord) -> BitBoard { // [BitBoard(0); 64];
        unsafe { ORTHOGONAL_DIRECTIONS[s] }
    }

    #[inline]
    pub fn diagonal_directions(s: Coord) -> BitBoard {
        unsafe { DIAGONAL_DIRECTIONS[s] }
    }

    #[inline]
    pub fn rook_xray_moves(s: Coord) -> BitBoard {
        unsafe { ROOK_XRAY_MOVES[s] }
    }

    #[inline]
    pub fn bishop_xray_moves(s: Coord) -> BitBoard {
        unsafe { BISHOP_XRAY_MOVES[s] }
    }

    #[inline]
    pub fn queen_xray_moves(s: Coord) -> BitBoard {
        unsafe { QUEEN_XRAY_MOVES[s] }
    }

    /// The manhattan distance between two given squares. 
    #[inline]
    pub fn manhattan_distance(a: Coord, b: Coord) -> u32 {
        unsafe { MANHATTAN_DISTANCE[a][b] }
    }

    /// The distance between two given squares, using the max of the difference between the rank
    /// and file. 
    #[inline]
    pub fn king_distance(a: Coord, b: Coord) -> u32 {
        unsafe { KING_DISTANCE[a][b] }
    }

    /// The manhattan distance from the given square to the center of the board.
    #[inline]
    pub fn center_manhattan_distance(s: Coord) -> u32 {
        unsafe { CENTER_MANHATTAN_DISTANCE[s] }
    }

    #[inline]
    pub fn forward_ranks(color: usize, r: usize) -> BitBoard {
        unsafe { FORWARD_RANKS[color][r] }
    }

    #[inline]
    pub fn forward_files(color: usize, s: Coord) -> BitBoard {
        unsafe { FORWARD_FILES[color][s] }
    }

    #[inline]
    pub fn pawn_attack_span(color: usize, s: Coord) -> BitBoard {
        unsafe { PAWN_ATTACK_SPAN[color][s] }
    }

    #[inline]
    pub fn diagonal_squares(dir: usize, s: Coord) -> BitBoard {
        unsafe { DIAGONAL_SQUARES[dir][s] }
    }

    #[inline]
    pub fn orthogonal_squares(dir: usize, s: Coord) -> BitBoard {
        unsafe { ORTHOGONAL_SQUARES[dir][s] }
    }

    #[inline]
    pub fn white_passed_pawn_mask(s: Coord) -> BitBoard {
        unsafe { WHITE_PASSED_PAWN_MASK[s] }
    }

    #[inline]
    pub fn black_passed_pawn_mask(s: Coord) -> BitBoard {
        unsafe { BLACK_PASSED_PAWN_MASK[s] }
    }

    #[inline]
    pub fn white_pawn_support_mask(s: Coord) -> BitBoard {
        unsafe { WHITE_PAWN_SUPPORT_MASK[s] }
    }

    #[inline]
    pub fn black_pawn_support_mask(s: Coord) -> BitBoard {
        unsafe { BLACK_PAWN_SUPPORT_MASK[s] }
    }

    #[inline]
    pub fn adjacent_file_mask(f: usize) -> BitBoard {
        unsafe { ADJACENT_FILE_MASK[f] }
    }

    #[inline]
    pub fn king_safety_mask(s: Coord) -> BitBoard {
        unsafe { KING_SAFETY_MASK[s] }
    }

    #[inline]
    pub fn white_forward_file_mask(s: Coord) -> BitBoard {
        unsafe { WHITE_FORWARD_FILE_MASK[s] }
    }

    #[inline]
    pub fn black_forward_file_mask(s: Coord) -> BitBoard {
        unsafe { BLACK_FORWARD_FILE_MASK[s] }
    }

    #[inline]
    pub fn triple_file_mask(f: usize) -> BitBoard {
        unsafe { TRIPLE_FILE_MASK[f] }
    }

    #[inline]
    pub fn king_ring(s: Coord) -> BitBoard {
        unsafe { KING_RING[s] }
    }
}

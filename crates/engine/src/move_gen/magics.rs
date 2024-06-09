use crate::bitboard::masks::{FILE_A, FILE_H, RANK_1, RANK_8};
use crate::board::coord::Coord;

use crate::bitboard::bb::BitBoard;

use super::prng::PRNG;


const ROOK_MAGIC_SIZE: usize = 102_400;
static mut ROOK_MAGICS: [Magic; 64] = [Magic::new(); 64];
static mut ROOK_TABLE: [u64; ROOK_MAGIC_SIZE] = [0; ROOK_MAGIC_SIZE];

const BISHOP_MAGIC_SIZE: usize = 5248;
static mut BISHOP_MAGICS: [Magic; 64] = [Magic::new(); 64];
static mut BISHOP_TABLE: [u64; BISHOP_MAGIC_SIZE] = [0; BISHOP_MAGIC_SIZE];

const SEEDS: [[u64; 8]; 2] = [
    [8977, 44_560, 54_343, 38_998, 5731, 95_205, 104_912, 17_020],
    [728, 10_316, 55_013, 32_803, 12_281, 15_100, 16_645, 255],
];


#[derive(Debug)]
pub struct Magics;

impl Magics {
    // pub const ROOK_SHIFTS: [u32; 64] = [52, 52, 52, 52, 52, 52, 52, 52, 53, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 52, 54, 53, 53, 53, 53, 54, 53, 52, 53, 54, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52];
    // pub const BISHOP_SHIFTS: [u32; 64] = [58, 60, 59, 59, 59, 59, 60, 58, 60, 59, 59, 59, 59, 59, 59, 60, 59, 59, 57, 57, 57, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59, 60, 60, 59, 59, 59, 59, 60, 60, 58, 60, 59, 59, 59, 59, 59, 58];
    //
    // pub const ROOK_MAGICS: [u64; 64] = [468374916371625120, 18428729537625841661, 2531023729696186408, 6093370314119450896, 13830552789156493815, 16134110446239088507, 12677615322350354425, 5404321144167858432, 2111097758984580, 18428720740584907710, 17293734603602787839, 4938760079889530922, 7699325603589095390, 9078693890218258431, 578149610753690728, 9496543503900033792, 1155209038552629657, 9224076274589515780, 1835781998207181184, 509120063316431138, 16634043024132535807, 18446673631917146111, 9623686630121410312, 4648737361302392899, 738591182849868645, 1732936432546219272, 2400543327507449856, 5188164365601475096, 10414575345181196316, 1162492212166789136, 9396848738060210946, 622413200109881612, 7998357718131801918, 7719627227008073923, 16181433497662382080, 18441958655457754079, 1267153596645440, 18446726464209379263, 1214021438038606600, 4650128814733526084, 9656144899867951104, 18444421868610287615, 3695311799139303489, 10597006226145476632, 18436046904206950398, 18446726472933277663, 3458977943764860944, 39125045590687766, 9227453435446560384, 6476955465732358656, 1270314852531077632, 2882448553461416064, 11547238928203796481, 1856618300822323264, 2573991788166144, 4936544992551831040, 13690941749405253631, 15852669863439351807, 18302628748190527413, 12682135449552027479, 13830554446930287982, 18302628782487371519, 7924083509981736956, 4734295326018586370];
    // pub const BISHOP_MAGICS: [u64; 64] = [16509839532542417919, 14391803910955204223, 1848771770702627364, 347925068195328958, 5189277761285652493, 3750937732777063343, 18429848470517967340, 17870072066711748607, 16715520087474960373, 2459353627279607168, 7061705824611107232, 8089129053103260512, 7414579821471224013, 9520647030890121554, 17142940634164625405, 9187037984654475102, 4933695867036173873, 3035992416931960321, 15052160563071165696, 5876081268917084809, 1153484746652717320, 6365855841584713735, 2463646859659644933, 1453259901463176960, 9808859429721908488, 2829141021535244552, 576619101540319252, 5804014844877275314, 4774660099383771136, 328785038479458864, 2360590652863023124, 569550314443282, 17563974527758635567, 11698101887533589556, 5764964460729992192, 6953579832080335136, 1318441160687747328, 8090717009753444376, 16751172641200572929, 5558033503209157252, 17100156536247493656, 7899286223048400564, 4845135427956654145, 2368485888099072, 2399033289953272320, 6976678428284034058, 3134241565013966284, 8661609558376259840, 17275805361393991679, 15391050065516657151, 11529206229534274423, 9876416274250600448, 16432792402597134585, 11975705497012863580, 11457135419348969979, 9763749252098620046, 16960553411078512574, 15563877356819111679, 14994736884583272463, 9441297368950544394, 14537646123432199168, 9888547162215157388, 18140215579194907366, 18374682062228545019];

    // 64 - shift_amount
    // pub const ROOK_N_BITS: [u32; 64] = [64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 53, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 54, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 54, 64 - 54, 64 - 53, 64 - 52, 64 - 54, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 52, 64 - 53, 64 - 54, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 54, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 52, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 52];
    // pub const BISHOP_N_BITS: [u32; 64] = [64 - 58, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 60, 64 - 58, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 60, 64 - 59, 64 - 59, 64 - 57, 64 - 57, 64 - 57, 64 - 57, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 57, 64 - 55, 64 - 55, 64 - 57, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 57, 64 - 55, 64 - 55, 64 - 57, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 57, 64 - 57, 64 - 57, 64 - 57, 64 - 59, 64 - 59, 64 - 60, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 60, 64 - 60, 64 - 58, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 58];
    
    // 1 << n_bits
    // pub const ROOK_SIZES: [usize; 64] = [1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 52), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 52), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 52), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 52)];
    // pub const BISHOP_SIZES: [usize; 64] = [1 << (64 - 58), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 58), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 55), 1 << (64 - 55), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 55), 1 << (64 - 55), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 60), 1 << (64 - 58), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 58)];

    pub const MIN_ROOK_LOOKUP_SIZE: usize = 4096;
    pub const MIN_BISHOP_LOOKUP_SIZE: usize = 512;

    #[inline]
    pub fn slider_attacks(square: Coord, blockers: BitBoard, ortho: bool) -> BitBoard {
        if ortho { Self::rook_attacks(square, blockers) } else { Self::bishop_attacks(square, blockers) }
    }

    #[inline]
    pub fn rook_attacks(square: Coord, blockers: BitBoard) -> BitBoard {
        let mut blockers = blockers.0;
        let entry = unsafe { ROOK_MAGICS.get_unchecked(square.index()) };
        blockers &= entry.mask;
        blockers = blockers.wrapping_mul(entry.magic);
        blockers = blockers.wrapping_shr(entry.shift);
        BitBoard(unsafe { *(entry.offset as *const u64).add(blockers as usize) })
    }

    #[inline]
    pub fn bishop_attacks(square: Coord, blockers: BitBoard) -> BitBoard {
        let mut blockers = blockers.0;
        let entry = unsafe { BISHOP_MAGICS.get_unchecked(square.index()) };
        blockers &= entry.mask;
        blockers = blockers.wrapping_mul(entry.magic);
        blockers = blockers.wrapping_shr(entry.shift);
        BitBoard(unsafe { *(entry.offset as *const u64).add(blockers as usize) })
    }

    pub fn create_blocker_bitboards(move_mask: BitBoard) -> Vec<BitBoard> {
        let mut move_sqr_idx = Vec::new();
        for i in 0..64 {
            if ((move_mask >> i) & 1).0 == 1 {
                move_sqr_idx.push(i);
            }
        }
        let n_patterns = 1 << move_sqr_idx.len();
        let mut blocker_bitboards = vec![BitBoard(0); n_patterns];

        for (pattern_idx, blocker_bitboard) in blocker_bitboards.iter_mut().enumerate().take(n_patterns) {
            for (bit_idx, move_sqr_i) in move_sqr_idx.iter().enumerate() {
                let bit = BitBoard::from((pattern_idx >> bit_idx) & 1);
                *blocker_bitboard |= bit << *move_sqr_i;
            };
        };
        blocker_bitboards
    }

    pub fn create_movement_mask(start_coord: Coord, ortho: bool) -> BitBoard {
        let mut mask = BitBoard(0);
        let directions = if ortho { Coord::ROOK_DIRECTIONS } else { Coord::BISHOP_DIRECTIONS };

        for dir in directions {
            for dst in 1..8 {
                let coord = start_coord + dir * dst;
                let next_coord = start_coord + dir * (dst + 1);
                if next_coord.is_valid() {
                    mask.set_square(coord.square());
                } else { break; }
            }
        }
        
        let edges = 
            ((FILE_A | FILE_H) & if ortho { !BitBoard::from_file(start_coord.file()).0 } else { u64::MAX }) | 
            ((RANK_1 | RANK_8) & if ortho { !BitBoard::from_rank(start_coord.rank()).0 } else { u64::MAX });
        
        mask & !edges
    }

    pub fn legal_move_bitboard_from_blockers(start_sqr: Coord, blockers: BitBoard, ortho: bool) -> BitBoard {
        let mut bitboard = BitBoard(0);
        let directions = if ortho { Coord::ROOK_DIRECTIONS } else { Coord::BISHOP_DIRECTIONS };

        for dir in directions {
            for dst in 1..8 {
                let coord = start_sqr + dir * dst;
                if coord.is_valid() {
                    bitboard.set_square(coord.square());
                    if blockers.contains_square(coord.square()) {
                        break;
                    }
                } else { break; }
            }
        }
        bitboard
    }

    fn create_table(square: Coord, ortho: bool, magic: u64, left_shift: u32) -> Vec<BitBoard> {
        let n_bits = 64 - left_shift;
        let lookup_size = 1 << n_bits;
        let mut table = vec![BitBoard(0); lookup_size];
        let move_mask = Self::create_movement_mask(square, ortho);
        let blockers = Self::create_blocker_bitboards(move_mask);

        for pattern in blockers {
            let index = (pattern.0.wrapping_mul(magic)).wrapping_shr(left_shift);
            let moves = Self::legal_move_bitboard_from_blockers(square, pattern, ortho);
            table[index as usize] = moves;
        };
        table
    }
}


#[derive(Clone, Copy)]
struct Magic {
    offset: usize,
    mask: u64,
    magic: u64,
    shift: u32,
}

impl Magic {
    pub const fn new() -> Self {
        Magic {
            offset: 0,
            mask: 0,
            magic: 0,
            shift: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct PreMagic {
    start: usize,
    len: usize,
    mask: u64,
    magic: u64,
    shift: u32,
}

impl PreMagic {
    pub fn new() -> PreMagic {
        PreMagic {
            start: 0,
            len: 0,
            mask: 0,
            magic: 0,
            shift: 0,
        }
    }

    pub fn next_idx(&self) -> usize {
        self.start + self.len
    }
}


#[cold]
pub fn initialize() {
    unsafe {
        gen_magics(
            BISHOP_MAGIC_SIZE,
            &[7, 9, -9, -7],
            BISHOP_MAGICS.as_mut_ptr(),
            BISHOP_TABLE.as_mut_ptr(),
        );
        gen_magics(
            ROOK_MAGIC_SIZE,
            &[8, 1, -8, -1],
            ROOK_MAGICS.as_mut_ptr(),
            ROOK_TABLE.as_mut_ptr(),
        );
    }
}

unsafe fn gen_magics(
    table_size: usize,
    deltas: &[i8; 4],
    smagics: *mut Magic,
    attacks: *mut u64
) {
    let mut pre_sqr_table = [PreMagic::new(); 64];

    let mut blockers: [u64; 4096] = [0; 4096];
    let mut reference: [u64; 4096] = [0; 4096];
    let mut age: [i32; 4096] = [0; 4096];

    let mut size: usize;
    let mut b: u64;
    let mut current: i32 = 0;
    let mut i: usize;

    pre_sqr_table[0].start = 0;

    for s in Coord::iter_squares() {
        let mut magic: u64;

        let edges = ((BitBoard::RANK_1 | BitBoard::RANK_8) & !BitBoard::from_rank(s.rank())).0
            | ((BitBoard::FILE_A | BitBoard::FILE_H) & !BitBoard::from_file(s.file())).0;
        let mask = sliding_attack(deltas, s.square() as u8, 0) & !edges;
        let shift = 64 - mask.count_ones();

        b = 0;
        size = 0;

        'bit: loop {
            blockers[size] = b;
            reference[size] = sliding_attack(deltas, s.square() as u8, b);
            size += 1;
            b = (b.wrapping_sub(mask)) & mask;
            if b == 0 {
                break 'bit;
            }
        }

        pre_sqr_table[s].len = size;
        
        if s.square() < 63 {
            pre_sqr_table[s.index() + 1].start = pre_sqr_table[s].next_idx();
        }

        let mut rng = PRNG::new(SEEDS[1][s.rank() as usize]);

        'outer: loop {
            'inner: loop {
                magic = rng.sparse_rand();
                if magic.wrapping_mul(mask).wrapping_shr(56).count_ones() >= 6 {
                    break 'inner;
                }
            }

            current += 1;
            i = 0;
            
            while i < size {
                let index = ((blockers[i] & mask).wrapping_mul(magic)).wrapping_shr(shift) as usize;

                if age[index] < current {
                    age[index] = current;
                    *attacks.add(pre_sqr_table[s].start + index) = reference[i];
                } else if *attacks.add(pre_sqr_table[s].start + index) != reference[i] {
                    break;
                }
                
                i += 1;
            }

            if i >= size {
                break 'outer;
            }
        }

        pre_sqr_table[s].magic = magic;
        pre_sqr_table[s].mask = mask;
        pre_sqr_table[s].shift = shift;
    }

    let mut size = 0;

    for (i, table) in pre_sqr_table.iter().enumerate() {
        let begin_offset = attacks.add(size);
        let static_ptr: *mut Magic = smagics.add(i);

        let table_i = Magic {
            offset: begin_offset as usize,
            mask: table.mask,
            magic: table.magic,
            shift: table.shift,
        };

        std::ptr::copy::<Magic>(&table_i, static_ptr, 1);
        size += table.len;
    }
}

fn sliding_attack(deltas: &[i8; 4], sqr: u8, blockers: u64) -> u64 {
    let mut attack = 0;
    let square = sqr as i16;

    for delta in deltas.iter().take(4) {
        let mut s = (square + (*delta as i16)) as u8;
        'inner: while s < 64 && Coord::from_idx(s as i8).distance(Coord::from_idx(((s as i16) - (*delta as i16)) as i8)) == 1 {
            attack |= 1u64.wrapping_shl(s as u32);
            if blockers & 1u64.wrapping_shl(s as u32) != 0 {
                break 'inner;
            }
            s = ((s as i16) + (*delta as i16)) as u8;
        }
    }

    attack
}

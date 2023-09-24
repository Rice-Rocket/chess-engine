use crate::board::{coord::Coord, piece::Piece};

use super::{perspective::Perspective, pos::PositionEvaluation, scores::*};



pub const PIECE_VALUE_BONUSES_MG: [i32; 5] = [124, 781, 825, 1276, 2538];
pub const PIECE_VALUE_BONUSES_EG: [i32; 5] = [206, 854, 915, 1380, 2682];


const PIECE_SQUARE_TABLE_BONUSES_MIDGAME: [[[i32; 4]; 8]; 5] = [
    [[-175,-92,-74,-73],[-77,-41,-27,-15],[-61,-17,6,12],[-35,8,40,49],[-34,13,44,51],[-9,22,58,53],[-67,-27,4,37],[-201,-83,-56,-26]],
    [[-53,-5,-8,-23],[-15,8,19,4],[-7,21,-5,17],[-5,11,25,39],[-12,29,22,31],[-16,6,1,11],[-17,-14,5,0],[-48,1,-14,-23]],
    [[-31,-20,-14,-5],[-21,-13,-8,6],[-25,-11,-1,3],[-13,-5,-4,-6],[-27,-15,-4,3],[-22,-2,6,12],[-2,12,16,18],[-17,-19,-1,9]],
    [[3,-5,-5,4],[-3,5,8,12],[-3,6,13,7],[4,5,9,8],[0,14,12,5],[-4,10,6,8],[-5,6,10,8],[-2,-2,1,-2]],
    [[271,327,271,198],[278,303,234,179],[195,258,169,120],[164,190,138,98],[154,179,105,70],[123,145,81,31],[88,120,65,33],[59,89,45,-1]]
];
const PIECE_SQUARE_TABLE_BONUSES_ENDGAME: [[[i32; 4]; 8]; 5] = [
    [[-96,-65,-49,-21],[-67,-54,-18,8],[-40,-27,-8,29],[-35,-2,13,28],[-45,-16,9,39],[-51,-44,-16,17],[-69,-50,-51,12],[-100,-88,-56,-17]],
    [[-57,-30,-37,-12],[-37,-13,-17,1],[-16,-1,-2,10],[-20,-6,0,17],[-17,-1,-14,15],[-30,6,4,6],[-31,-20,-1,1],[-46,-42,-37,-24]],
    [[-9,-13,-10,-9],[-12,-9,-1,-2],[6,-8,-2,-6],[-6,1,-9,7],[-5,8,7,-6],[6,1,-7,10],[4,5,20,-5],[18,0,19,13]],
    [[-69,-57,-47,-26],[-55,-31,-22,-4],[-39,-18,-9,3],[-23,-3,13,24],[-29,-6,9,21],[-38,-18,-12,1],[-50,-27,-24,-8],[-75,-52,-43,-36]],
    [[1,45,85,76],[53,100,133,135],[88,130,169,175],[103,156,172,172],[96,166,199,199],[92,172,184,191],[47,121,116,131],[11,59,73,78]]
];
const PAWN_SQUARE_BONUSES_MIDGAME: [[i32; 8]; 8] = [
    [0,0,0,0,0,0,0,0], [3,3,10,19,16,19,7,-5], [-9,-15,11,15,32,22,5,-22], [-4,-23,6,20,40,17,4,-8],
    [13,0,-13,1,11,-2,-13,5], [5,-12,-7,22,-8,-5,-15,-8], [-7,7,-3,-13,5,-16,10,-8], [0,0,0,0,0,0,0,0]
];
const PAWN_SQUARE_BONUSES_ENDGAME: [[i32; 8]; 8] = [
    [0,0,0,0,0,0,0,0],[-10,-6,10,0,14,7,-5,-19],[-10,-10,-10,4,4,3,-6,-4],[6,-2,-8,-4,-13,-12,-10,-9],
    [10,5,4,-5,-5,-5,14,9],[28,20,21,28,30,7,6,13],[0,-11,12,21,25,19,4,7],[0,0,0,0,0,0,0,0]
];


pub struct MaterialEvaluationData {
    pub piece_counts: [[u8; 6]; 2],
    pub pcount_white: u8,
    pub pcount_black: u8,
    pub pcount_total: u8,

    pub imbalance_total: i32,
    pub white_material: (i32, i32),
    pub black_material: (i32, i32),
    pub white_non_pawn_material: i32,
    pub black_non_pawn_material: i32,
    pub white_psqt_bonuses: (i32, i32),
    pub black_psqt_bonuses: (i32, i32),
}

impl MaterialEvaluationData {
    pub fn new(pos: &PositionEvaluation) -> Self {
        let mut eval_data = Self {
            piece_counts: [[0; 6]; 2],
            pcount_white: 0,
            pcount_black: 0,
            pcount_total: 0,
            imbalance_total: 0,
            white_material: (0, 0),
            black_material: (0, 0),
            white_non_pawn_material: 0,
            black_non_pawn_material: 0,
            white_psqt_bonuses: (0, 0),
            black_psqt_bonuses: (0, 0),
        };
        eval_data.initialize(pos);
        eval_data
    }

    fn initialize(&mut self, pos: &PositionEvaluation) {
        self.piece_counts = [[
            (pos.board.get_piece_list(Piece::new(Piece::WHITE_BISHOP)).count() > 0) as u8,
            pos.board.get_piece_list(Piece::new(Piece::WHITE_PAWN)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::WHITE_KNIGHT)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::WHITE_BISHOP)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::WHITE_ROOK)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::WHITE_QUEEN)).count() as u8,
            ], [
            (pos.board.get_piece_list(Piece::new(Piece::BLACK_BISHOP)).count() > 0) as u8,
            pos.board.get_piece_list(Piece::new(Piece::BLACK_PAWN)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::BLACK_KNIGHT)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::BLACK_BISHOP)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::BLACK_ROOK)).count() as u8,
            pos.board.get_piece_list(Piece::new(Piece::BLACK_QUEEN)).count() as u8,
        ]];

        self.pcount_white = 0;
        self.pcount_black = 0;
        self.piece_counts[0][1..6].iter().for_each(|x| self.pcount_white += x);
        self.piece_counts[1][1..6].iter().for_each(|x| self.pcount_black += x);
        self.pcount_total = self.pcount_white + self.pcount_black;

        self.white_material = (self.piece_value_bonus(Perspective::White, true), self.piece_value_bonus(Perspective::White, false));
        self.black_material = (self.piece_value_bonus(Perspective::Black, true), self.piece_value_bonus(Perspective::Black, false));

        self.white_non_pawn_material = self.non_pawn_material(Perspective::White);
        self.black_non_pawn_material = self.non_pawn_material(Perspective::Black);

        self.white_psqt_bonuses = (self.psqt_bonus(pos, Perspective::White, true), self.psqt_bonus(pos, Perspective::White, false));
        self.black_psqt_bonuses = (self.psqt_bonus(pos, Perspective::Black, true), self.psqt_bonus(pos, Perspective::Black, false));

        self.imbalance_total = (self.imbalance(Perspective::White) - self.imbalance(Perspective::Black)) / 16;
    }

    /// Gets the number of pieces on the board based on perspective and piece type. 
    /// If the piece type is 0, the bishop pair value of the perspective will be returned instead. 
    /// If the piece type is king or higher, 0 will be returned. 
    pub fn pcount(&self, per: Perspective, ptype: u8) -> u8 {
        if ptype > 5 { return 0; }
        self.piece_counts[per.color_idx()][ptype as usize]
    }

    pub fn friendly_pcount(&self, per: Perspective) -> u8 {
        if per.is_white() {
            self.pcount_white
        } else {
            self.pcount_black
        }
    }

    pub fn get_non_pawn_material(&self, per: Perspective) -> i32 {
        if per.is_white() {
            self.white_non_pawn_material
        } else {
            self.black_non_pawn_material
        }
    }

    fn piece_value_bonus(&self, per: Perspective, mg: bool) -> i32 {
        let mut sum = 0;
        let bonus = if mg { PIECE_VALUE_BONUSES_MG } else { PIECE_VALUE_BONUSES_EG };

        for pt in 1..6 {
            let count = self.pcount(per, pt);
            if count == 0 { continue; }
            sum += bonus[pt as usize - 1] * count as i32;
        }

        return sum;
    }

    fn non_pawn_material(&self, per: Perspective) -> i32 {
        let mut sum = 0;
        let bonus = PIECE_VALUE_BONUSES_MG;

        for pt in 2..6 {
            let count = self.pcount(per, pt);
            if count == 0 { continue; }
            sum += bonus[pt as usize - 1] * count as i32;
        }

        return sum;
    }   
     
    fn psqt_bonus(&self, pos: &PositionEvaluation, per: Perspective, mg: bool) -> i32 {
        let mut sum = 0;
        let bonuses = if mg { PIECE_SQUARE_TABLE_BONUSES_MIDGAME } else { PIECE_SQUARE_TABLE_BONUSES_ENDGAME };
        let p_bonuses = if mg { PAWN_SQUARE_BONUSES_MIDGAME } else { PAWN_SQUARE_BONUSES_ENDGAME };

        let mut friendly_sqrs = pos.friendly_color_bb(per);

        while friendly_sqrs.0 != 0 {
            let sq_idx = friendly_sqrs.pop_lsb() as i8;
            let c = Coord::from_idx(sq_idx);
            let ptype = pos.square(c).piece_type();

            if ptype == Piece::PAWN {
                sum += if per.is_white() {
                    p_bonuses[c.rank() as usize][c.file() as usize]
                } else {
                    p_bonuses[7 - c.rank() as usize][c.file() as usize]
                };
            } else {
                let file = c.file().min(7 - c.file()) as usize;
                sum += if per.is_white() {
                    bonuses[ptype as usize - 2][c.rank() as usize][file]
                } else {
                    bonuses[ptype as usize - 2][7 - c.rank() as usize][file]
                }
            }
        }
        return sum;
    }

    fn imbalance(&self, per: Perspective) -> i32 {
        let mut sum = 0;

        for pt1 in 0..6 {
            let count = self.pcount(per, pt1);
            if count == 0 { continue; }
            let mut v = 0;

            for pt2 in 0..6 {
                if pt2 > pt1 { continue; }
                v += QUADRATIC_OURS[pt1 as usize][pt2 as usize] * self.pcount(per, pt2) as i32
                    + QUADRATIC_THEIRS[pt1 as usize][pt2 as usize] * self.pcount(per.other(), pt2) as i32;
            }

            sum += count as i32 * v;
        }
        return sum;
    }
}
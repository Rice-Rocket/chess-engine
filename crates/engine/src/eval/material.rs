use proc_macro_utils::evaluation_fn;

use crate::{board::{coord::Coord, piece::Piece}, color::{Black, Color, White}};
use super::Evaluation;


impl<'a> Evaluation<'a> {
    pub fn non_pawn_material<W: Color, B: Color>(&self) -> i32 {
        Self::PIECE_VALUE_BONUS_MG[1] * self.knight_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[2] * self.bishop_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[3] * self.rook_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[4] * self.queen_count::<W, B>()
    }

    const PIECE_VALUE_BONUS_MG: [i32; 5] = [124, 781, 825, 1276, 2538];
    const PIECE_VALUE_BONUS_EG: [i32; 5] = [206, 854, 915, 1380, 2682];

    pub fn piece_value_mg<W: Color, B: Color>(&self) -> i32 {
        Self::PIECE_VALUE_BONUS_MG[0] * self.pawn_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[1] * self.knight_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[2] * self.bishop_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[3] * self.rook_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_MG[4] * self.queen_count::<W, B>()
    }

    pub fn piece_value_eg<W: Color, B: Color>(&self) -> i32 {
        Self::PIECE_VALUE_BONUS_EG[0] * self.pawn_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_EG[1] * self.knight_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_EG[2] * self.bishop_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_EG[3] * self.rook_count::<W, B>()
            + Self::PIECE_VALUE_BONUS_EG[4] * self.queen_count::<W, B>()
    }

    const PSQT_BONUS_MG: [[[i32; 4]; 8]; 5] = [
        [[-175, -92, -74, -73], [-77, -41, -27, -15], [-61, -17, 6,    12], [-35,   8, 40,  49], [-34,  13, 44,  51], [-9,   22, 58, 53], [-67, -27, 4,  37], [-201, -83, -56, -26]],
        [[-53,   -5, -8,  -23], [-15,   8, 19,    4], [-7,   21, -5,   17], [-5,   11, 25,  39], [-12,  29, 22,  31], [-16,   6, 1,  11], [-17, -14, 5,   0], [-48,    1, -14, -23]],
        [[-31,  -20, -14,  -5], [-21, -13, -8,    6], [-25, -11, -1,    3], [-13,  -5, -4,  -6], [-27, -15, -4,   3], [-22,  -2, 6,  12], [-2,   12, 16, 18], [-17,  -19, -1,    9]],
        [[3,     -5, -5,    4], [-3,    5, 8,    12], [-3,    6, 13,    7], [4,     5, 9,    8], [0,    14, 12,   5], [-4,   10, 6,   8], [-5,    6, 10,  8], [-2,    -2, 1,    -2]],
        [[271,  327, 271, 198], [278, 303, 234, 179], [195, 258, 169, 120], [164, 190, 138, 98], [154, 179, 105, 70], [123, 145, 81, 31], [88,  120, 65, 33], [59,    89, 45,   -1]]
    ];
    const PSQT_BONUS_EG: [[[i32; 4]; 8]; 5] = [
        [[-96, -65, -49, -21], [-67, -54, -18,   8], [-40, -27, -8,   29], [-35,  -2, 13,   28], [-45, -16, 9,    39], [-51, -44, -16,  17], [-69, -50, -51,  12], [-100, -88, -56, -17]],
        [[-57, -30, -37, -12], [-37, -13, -17,   1], [-16,  -1, -2,   10], [-20,  -6, 0,    17], [-17,  -1, -14,  15], [-30,   6, 4,     6], [-31, -20, -1,    1], [-46,  -42, -37, -24]],
        [[-9,  -13, -10,  -9], [-12,  -9, -1,   -2], [6,    -8, -2,   -6], [-6,    1, -9,    7], [-5,    8, 7,    -6], [6,     1, -7,   10], [4,     5, 20,   -5], [18,     0, 19,   13]],
        [[-69, -57, -47, -26], [-55, -31, -22,  -4], [-39, -18, -9,    3], [-23,  -3, 13,   24], [-29,  -6, 9,    21], [-38, -18, -12,   1], [-50, -27, -24,  -8], [-75,  -52, -43, -36]],
        [[1,    45, 85,   76], [53,  100, 133, 135], [88,  130, 169, 175], [103, 156, 172, 172], [96,  166, 199, 199], [92,  172, 184, 191], [47,  121, 116, 131], [11,    59, 73,   78]]
    ];

    const PSQT_BONUS_MG_PAWN: [[i32; 8]; 8] = [
        [0,    0, 0,     0, 0,    0, 0,     0],
        [3,    3, 10,   19, 16,  19, 7,    -5],
        [-9, -15, 11,   15, 32,  22, 5,   -22],
        [-4, -23, 6,    20, 40,  17, 4,    -8],
        [13,   0, -13,   1, 11,  -2, -13,   5],
        [5,  -12, -7,   22, -8,  -5, -15,  -8],
        [-7,   7, -3,  -13, 5,  -16, 10,   -8],
        [0,    0, 0,     0, 0,    0, 0,     0]
    ];

    const PSQT_BONUS_EG_PAWN: [[i32; 8]; 8] = [
        [0,     0, 0,    0, 0,     0, 0,     0],
        [-10,  -6, 10,   0, 14,    7, -5,  -19],
        [-10, -10, -10,  4, 4,     3, -6,   -4],
        [6,    -2, -8,  -4, -13, -12, -10,  -9],
        [10,    5, 4,   -5, -5,   -5, 14,    9],
        [28,   20, 21,  28, 30,    7, 6,    13],
        [0,   -11, 12,  21, 25,   19, 4,     7],
        [0,     0, 0,    0, 0,     0, 0,     0]
    ];

    pub fn psqt_mg<W: Color, B: Color>(&self) -> i32 {
        let mut v = 0;
        let mut pieces = self.board.color_bitboards[W::index()];

        while pieces.0 != 0 {
            let sqr = Coord::from_idx(pieces.pop_lsb() as i8);
            let ptype = self.board.square[sqr].piece_type();
            v += if ptype == Piece::PAWN {
                Self::PSQT_BONUS_MG_PAWN[W::rank(sqr.rank()) as usize][sqr.file() as usize]
            } else {
                Self::PSQT_BONUS_MG[ptype as usize - 2][W::rank(sqr.rank()) as usize][sqr.file().min(7 - sqr.file()) as usize]
            };
        }

        v
    }

    pub fn psqt_eg<W: Color, B: Color>(&self, sqr: Coord) -> i32 {
        let mut v = 0;
        let mut pieces = self.board.color_bitboards[W::index()];

        while pieces.0 != 0 {
            let sqr = Coord::from_idx(pieces.pop_lsb() as i8);
            let ptype = self.board.square[sqr].piece_type();
            v += if ptype == Piece::PAWN {
                Self::PSQT_BONUS_EG_PAWN[W::rank(sqr.rank()) as usize][sqr.file() as usize]
            } else {
                Self::PSQT_BONUS_EG[ptype as usize - 2][W::rank(sqr.rank()) as usize][sqr.file().min(7 - sqr.file()) as usize]
            };
        }

        v
    }
}


#[cfg(test)]
mod tests {
    use crate::eval::test_prelude::*;
    use super::*;

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_non_pawn_material() {
        assert_eval!(- non_pawn_material, 11335, 11577, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_piece_value_bonus() {
        assert_eval!(- piece_value_mg, 12203, 12197, eval);
    }

    #[test]
    #[evaluation_test("1r3q1R/p1p1n2n/n2k1pR1/pQ3P1B/1bP2qpr/QP3n1P/P1P1P3/2B1N1RK w kq - 9 6")]
    fn test_psqt_bonus() {
        assert_eval!(- psqt_mg, 146, 32, eval);
    }
}


pub const ENDGAME_LIMIT: i32 = 3915;
pub const MIDGAME_LIMIT: i32 = 15258;

pub const PAWN_BONUS: i32 = 100;
pub const KNIGHT_BONUS: i32 = 350;
pub const BISHOP_BONUS: i32 = 351;
pub const ROOK_BONUS: i32 = 500;
pub const QUEEN_BONUS: i32 = 900;

pub const PAWN_BONUS_MG: i32 = 171;
pub const KNIGHT_BONUS_MG: i32 = 764;
pub const BISHOP_BONUS_MG: i32 = 826;
pub const ROOK_BONUS_MG: i32 = 1282;
pub const QUEEN_BONUS_MG: i32 = 2526;

pub const PAWN_BONUS_EG: i32 = 240;
pub const KNIGHT_BONUS_EG: i32 = 848;
pub const BISHOP_BONUS_EG: i32 = 891;
pub const ROOK_BONUS_EG: i32 = 1373;
pub const QUEEN_BONUS_EG: i32 = 2646;


pub const PHASE_ENDGAME: u16 = 0;
pub const PHASE_MIDGAME: u16 = 128;


pub const SCALE_FACTOR_DRAW: u8 = 0;
pub const SCALE_FACTOR_ONEPAWN: u8 = 48;
pub const SCALE_FACTOR_NORMAL: u8 = 64;
pub const SCALE_FACTOR_MAX: u8 = 128;
pub const SCALE_FACTOR_NONE: u8 = 255;


pub const QUADRATIC_OURS: [[i32; 6]; 6] = [
    [1667, 0, 0, 0, 0, 0],          
    [40, 0, 0, 0, 0, 0],            
    [32, 255, -3, 0, 0, 0],         
    [0, 104, 4, 0, 0, 0],           
    [-26, -2, 47, 105, -149, 0],    
    [-189, 24, 117, 133, -134, -10],
]; 

pub const QUADRATIC_THEIRS: [[i32; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],          
    [36, 0, 0, 0, 0, 0],         
    [9, 63, 0, 0, 0, 0],         
    [59, 65, 42, 0, 0, 0],       
    [46, 39, 24, -24, 0, 0],     
    [97, 100, -42, 137, 268, 0], 
];
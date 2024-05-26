use engine::board::{coord::Coord, moves::Move, piece::Piece, Board};

const FILE_NAMES: &str = "abcdefgh";
const RANK_NAMES: &str = "12345678";

fn square_from_name(s: &str) -> Option<Coord> {
    let mut chars = s.chars();
    let file = chars.next()?;
    let rank = chars.next()?;

    Some(Coord::new(
        FILE_NAMES.chars().position(|x| x == file)? as i8,
        RANK_NAMES.chars().position(|x| x == rank)? as i8,
    ))
}

fn name_from_square(c: Coord) -> Option<String> {
    let mut s = String::new();
    s.push(FILE_NAMES.chars().nth(c.file() as usize)?);
    s.push(RANK_NAMES.chars().nth(c.rank() as usize)?);
    Some(s)
}


pub fn move_from_name(board: &Board, m: &str) -> Option<Move> {
    if m.len() < 4 { return None };
    let start_sqr = square_from_name(&m[0..2])?;
    let target_sqr = square_from_name(&m[2..4])?;

    let piece = board.square[start_sqr].piece_type();

    let flag = if piece == Piece::PAWN {
        if m.len() > 4 {
            match m.chars().last().unwrap() {
                'n' => Move::KNIGHT_PROMOTION,
                'b' => Move::BISHOP_PROMOTION,
                'r' => Move::ROOK_PROMOTION,
                'q' => Move::QUEEN_PROMOTION,
                _ => Move::NORMAL,
            }
        } else if (target_sqr.rank() - start_sqr.rank()).abs() == 2 {
            Move::PAWN_TWO_FORWARD
        } else if start_sqr.file() != target_sqr.file() && board.square[target_sqr].piece_type() == Piece::NONE {
            Move::EN_PASSANT_CAPTURE
        } else {
            Move::NORMAL
        }
    } else if piece == Piece::KING {
        if (start_sqr.file() - target_sqr.file()).abs() > 1 {
            Move::CASTLING
        } else {
            Move::NORMAL
        }
    } else {
        Move::NORMAL
    };

    Some(Move::from_start_end_flagged(start_sqr.square(), target_sqr.square(), flag))
}


pub fn name_from_move(m: Move) -> Option<String> {
    let start_sqr = name_from_square(m.start())?;
    let target_sqr = name_from_square(m.target())?;
    let mut name = format!("{}{}", start_sqr, target_sqr);

    match m.move_flag() {
        Move::KNIGHT_PROMOTION => name.push('n'),
        Move::BISHOP_PROMOTION => name.push('b'),
        Move::ROOK_PROMOTION => name.push('r'),
        Move::QUEEN_PROMOTION => name.push('q'),
        _ => ()
    }

    Some(name)
}

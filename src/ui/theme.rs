use bevy::prelude::*;
use crate::board::piece::*;


#[derive(Clone, Copy, PartialEq)]
pub enum SquareColorTypes {
    Normal,
    Legal,
    Selected,
    MoveFromHighlight,
    MoveToHighlight,
}

pub struct SquareColors {
    pub normal: Color,
    pub legal: Color,
    pub selected: Color,
    pub move_from_highlight: Color,
    pub move_to_highlight: Color,
}

#[derive(Resource)]
pub struct BoardTheme {
    pub light_squares: SquareColors,
    pub dark_squares: SquareColors,
}

impl Default for BoardTheme {
    fn default() -> Self {
        BoardTheme {
            light_squares: SquareColors {
                normal: Color::rgb(0.93333334, 0.84705883, 0.7529412),
                legal: Color::rgb(0.8666667, 0.34901962, 0.34901962),
                selected: Color::rgb(0.9245283, 0.7725114, 0.48406905),
                move_from_highlight: Color::rgb(0.8113208, 0.6759371, 0.41714135),
                move_to_highlight: Color::rgb(0.8679245, 0.813849, 0.48718405),
            },
            dark_squares: SquareColors {
                normal: Color::rgb(0.67058825, 0.47843137, 0.39607844),
                legal: Color::rgb(0.77254903, 0.26666668, 0.30980393),
                selected: Color::rgb(0.7830189, 0.6196811, 0.31394625),
                move_from_highlight: Color::rgb(0.7735849, 0.6194568, 0.36854753),
                move_to_highlight: Color::rgb(0.7735849, 0.6780388, 0.37584552),
            }
        }
    }
}


pub struct PieceSprites {
    pub pawn: Handle<Image>,
    pub rook: Handle<Image>,
    pub knight: Handle<Image>,
    pub bishop: Handle<Image>,
    pub queen: Handle<Image>,
    pub king: Handle<Image>,
}

#[derive(Resource)]
pub struct PieceTheme {
    pub white_pieces: Option<PieceSprites>,
    pub black_pieces: Option<PieceSprites>,
}

impl PieceTheme {
    pub fn get_piece_sprite(&self, piece: Piece) -> Option<Handle<Image>> {
        let white_pieces = self.white_pieces.as_ref().unwrap();
        let black_pieces = self.black_pieces.as_ref().unwrap();
        let piece_sprites = if piece.is_color(Piece::WHITE) { white_pieces } else { black_pieces };

        return match piece.piece_type() {
            Piece::PAWN => Some(piece_sprites.pawn.clone()),
            Piece::ROOK => Some(piece_sprites.rook.clone()),
            Piece::KNIGHT => Some(piece_sprites.knight.clone()),
            Piece::BISHOP => Some(piece_sprites.bishop.clone()),
            Piece::QUEEN => Some(piece_sprites.queen.clone()),
            Piece::KING => Some(piece_sprites.king.clone()),
            _ => None
        };
    }
}

impl Default for PieceTheme {
    fn default() -> Self {
        PieceTheme {
            white_pieces: None,
            black_pieces: None
        }
    }
}

pub fn init_piece_theme(
    mut piece_theme: ResMut<PieceTheme>,
    asset_server: Res<AssetServer>
) {
    piece_theme.white_pieces = Some(PieceSprites {
        pawn: asset_server.load("ui/images/imgs-80px/White_pawn.png"),
        rook: asset_server.load("ui/images/imgs-80px/White_rook.png"),
        knight: asset_server.load("ui/images/imgs-80px/White_knight.png"),
        bishop: asset_server.load("ui/images/imgs-80px/White_bishop.png"),
        queen: asset_server.load("ui/images/imgs-80px/White_queen.png"),
        king: asset_server.load("ui/images/imgs-80px/White_king.png"),
    });
    piece_theme.black_pieces = Some(PieceSprites {
        pawn: asset_server.load("ui/images/imgs-80px/Black_pawn.png"),
        rook: asset_server.load("ui/images/imgs-80px/Black_rook.png"),
        knight: asset_server.load("ui/images/imgs-80px/Black_knight.png"),
        bishop: asset_server.load("ui/images/imgs-80px/Black_bishop.png"),
        queen: asset_server.load("ui/images/imgs-80px/Black_queen.png"),
        king: asset_server.load("ui/images/imgs-80px/Black_king.png"),
    });
}
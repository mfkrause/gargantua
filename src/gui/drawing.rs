use macroquad::texture::{Texture2D, load_texture};

use crate::game_state::{Piece, PieceColor, PieceOnBoard};

pub struct Textures {
    white_pawn: Texture2D,
    white_rook: Texture2D,
    white_knight: Texture2D,
    white_bishop: Texture2D,
    white_queen: Texture2D,
    white_king: Texture2D,
    black_pawn: Texture2D,
    black_rook: Texture2D,
    black_knight: Texture2D,
    black_bishop: Texture2D,
    black_queen: Texture2D,
    black_king: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        Self {
            white_pawn: load_texture("assets/wP.png").await.unwrap(),
            white_rook: load_texture("assets/wR.png").await.unwrap(),
            white_knight: load_texture("assets/wN.png").await.unwrap(),
            white_bishop: load_texture("assets/wB.png").await.unwrap(),
            white_queen: load_texture("assets/wQ.png").await.unwrap(),
            white_king: load_texture("assets/wK.png").await.unwrap(),
            black_pawn: load_texture("assets/bP.png").await.unwrap(),
            black_rook: load_texture("assets/bR.png").await.unwrap(),
            black_knight: load_texture("assets/bN.png").await.unwrap(),
            black_bishop: load_texture("assets/bB.png").await.unwrap(),
            black_queen: load_texture("assets/bQ.png").await.unwrap(),
            black_king: load_texture("assets/bK.png").await.unwrap(),
        }
    }

    pub fn get_texture_for_piece(&self, p: &PieceOnBoard) -> &Texture2D {
        match (p.piece, p.color) {
            (Piece::Pawn, PieceColor::White) => &self.white_pawn,
            (Piece::Rook, PieceColor::White) => &self.white_rook,
            (Piece::Knight, PieceColor::White) => &self.white_knight,
            (Piece::Bishop, PieceColor::White) => &self.white_bishop,
            (Piece::Queen, PieceColor::White) => &self.white_queen,
            (Piece::King, PieceColor::White) => &self.white_king,
            (Piece::Pawn, PieceColor::Black) => &self.black_pawn,
            (Piece::Rook, PieceColor::Black) => &self.black_rook,
            (Piece::Knight, PieceColor::Black) => &self.black_knight,
            (Piece::Bishop, PieceColor::Black) => &self.black_bishop,
            (Piece::Queen, PieceColor::Black) => &self.black_queen,
            (Piece::King, PieceColor::Black) => &self.black_king,
        }
    }
}

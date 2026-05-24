use macroquad::{
    color::{Color, RED, WHITE},
    math::vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};

use crate::game_state::{GameState, Piece, PieceColor, PieceOnBoard, Square};

const LIGHT_SQUARE_COLOR: Color = Color::from_hex(0xE2E2E2);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x6E7E85);

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

pub fn draw_game_state(
    game_state: &GameState,
    textures: &Textures,
    highlighted_square: &Option<Square>,
    window_size: f32,
) {
    let square_size = window_size / 8.0;
    for (y, board_col) in game_state.board.iter().enumerate() {
        for (x, piece) in board_col.iter().enumerate() {
            let square_x = (x as f32) * (window_size / 8.0);
            let square_y = (y as f32) * (window_size / 8.0);

            // Draw square
            draw_rectangle(
                square_x,
                square_y,
                square_size,
                square_size,
                if (x + y) % 2 == 0 {
                    LIGHT_SQUARE_COLOR
                } else {
                    DARK_SQUARE_COLOR
                },
            );

            if let Some(s) = highlighted_square
                && s.column == (y as u8)
                && s.row == (x as u8)
            {
                draw_rectangle_lines(square_x, square_y, square_size, square_size, 4.0, RED);
            }

            // Draw piece
            if let Some(p) = piece {
                let texture = textures.get_texture_for_piece(p);
                draw_texture_ex(
                    texture,
                    square_x,
                    square_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(square_size, square_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

use macroquad::{
    color::{Color, RED, WHITE},
    math::vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};

use crate::{
    game::game_state::{GameState, Piece, PieceColor, PieceKind, Square},
    gui::mouse::{DragState, MouseState},
};

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

    pub fn get_texture_for_piece(&self, p: &Piece) -> &Texture2D {
        match (p.piece, p.color) {
            (PieceKind::Pawn, PieceColor::White) => &self.white_pawn,
            (PieceKind::Rook, PieceColor::White) => &self.white_rook,
            (PieceKind::Knight, PieceColor::White) => &self.white_knight,
            (PieceKind::Bishop, PieceColor::White) => &self.white_bishop,
            (PieceKind::Queen, PieceColor::White) => &self.white_queen,
            (PieceKind::King, PieceColor::White) => &self.white_king,
            (PieceKind::Pawn, PieceColor::Black) => &self.black_pawn,
            (PieceKind::Rook, PieceColor::Black) => &self.black_rook,
            (PieceKind::Knight, PieceColor::Black) => &self.black_knight,
            (PieceKind::Bishop, PieceColor::Black) => &self.black_bishop,
            (PieceKind::Queen, PieceColor::Black) => &self.black_queen,
            (PieceKind::King, PieceColor::Black) => &self.black_king,
        }
    }
}

pub fn draw_game_state(
    game_state: &GameState,
    textures: &Textures,
    mouse_state: &MouseState,
    window_size: f32,
) {
    let square_size = window_size / 8.0;
    for i in 0..64 {
        let current_square = Square::from_bit_index(i);
        let piece = game_state.get_piece_at_square(&current_square);

        let current_square_x = current_square.file as f32 * (window_size / 8.0);
        let current_square_y = (7.0 - current_square.rank as f32) * (window_size / 8.0);

        // Draw square
        draw_rectangle(
            current_square_x,
            current_square_y,
            square_size,
            square_size,
            if (current_square.rank + current_square.file) % 2 == 1 {
                LIGHT_SQUARE_COLOR
            } else {
                DARK_SQUARE_COLOR
            },
        );

        if let Some(s) = mouse_state.active_square
            && matches!(mouse_state.drag_state, DragState::No)
            && s.rank == current_square.rank
            && s.file == current_square.file
        {
            draw_rectangle_lines(
                current_square_x,
                current_square_y,
                square_size,
                square_size,
                4.0,
                RED,
            );
        }

        // Draw piece (if it's not the currently dragged one)
        if let Some(p) = piece
            && (matches!(mouse_state.drag_state, DragState::No)
                || matches!(mouse_state.drag_state, DragState::Pending(..))
                || Some(current_square) != mouse_state.active_square)
        {
            let texture = textures.get_texture_for_piece(&p);
            draw_texture_ex(
                texture,
                current_square_x,
                current_square_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(square_size, square_size)),
                    ..Default::default()
                },
            );
        }
    }

    // Draw piece that's currently being dragged
    if let DragState::Dragging(piece, drag_offset) = mouse_state.drag_state {
        let texture = textures.get_texture_for_piece(&piece);
        draw_texture_ex(
            texture,
            mouse_state.mouse_vec.x - drag_offset.x,
            mouse_state.mouse_vec.y - drag_offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(square_size, square_size)),
                ..Default::default()
            },
        );
    }
}

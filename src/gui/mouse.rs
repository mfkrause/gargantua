use std::time::SystemTime;

use macroquad::{
    input::{
        MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
        mouse_position,
    },
    math::Vec2,
};

use crate::{
    game_state::{GameState, Piece, Square},
    utils::constants::SQUARE_SIZE,
};

#[derive(Debug, Default)]
pub struct MouseState {
    pub time_mouse_down: Option<SystemTime>,
    pub active_square: Option<Square>,
    pub dragging_piece: Option<Piece>,
    pub drag_offset: Vec2,
    pub mouse_vec: Vec2,
}

impl MouseState {
    pub fn handle_events(&mut self, game_state: &mut GameState) {
        let mouse_pos = mouse_position();
        self.mouse_vec = Vec2::from(mouse_pos);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.time_mouse_down = Some(SystemTime::now());
        }

        if let Some(time_down) = self.time_mouse_down
            && is_mouse_button_down(MouseButton::Left)
        {
            let now = SystemTime::now();
            let time_delta = now
                .duration_since(time_down)
                .expect("Time should move forward");

            if time_delta.as_millis() > 200 {
                // The user is holding the mouse
                if self.dragging_piece.is_none() {
                    let square = Square {
                        column: 7 - (self.mouse_vec.y / SQUARE_SIZE).floor() as u8,
                        row: (self.mouse_vec.x / SQUARE_SIZE).floor() as u8,
                    };
                    self.active_square = Some(square);
                    self.dragging_piece = game_state.get_piece_at_square(&square);
                    self.drag_offset = Vec2 {
                        x: self.mouse_vec.x % SQUARE_SIZE,
                        y: self.mouse_vec.y % SQUARE_SIZE,
                    }
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            let target_square = Square {
                column: 7 - (self.mouse_vec.y / SQUARE_SIZE).floor() as u8,
                row: (self.mouse_vec.x / SQUARE_SIZE).floor() as u8,
            };

            if let Some(source_square) = self.active_square {
                // User either dragged or clicked a piece to new location
                game_state.replace_piece(
                    &target_square,
                    game_state.get_piece_at_square(&source_square),
                );
                game_state.replace_piece(&source_square, None);
                self.active_square = None;
            } else {
                // User clicked with no square yet highlighted
                self.active_square = Some(target_square);
            }

            self.time_mouse_down = None;
            self.dragging_piece = None;
        }
    }
}

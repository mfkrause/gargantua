use std::time::SystemTime;

use macroquad::{
    input::{
        MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
        mouse_position,
    },
    math::Vec2,
};

use crate::{
    game::{
        game_state::{GameState, Piece, Square},
        rules::can_touch_piece,
    },
    utils::constants::{SQUARE_SIZE, WINDOW_SIZE},
};

#[derive(Default, Debug, PartialEq)]
pub enum DragState {
    #[default]
    No,
    Pending(Piece, Vec2),
    Dragging(Piece, Vec2),
    Dropped(Piece, Square, Square),
}

#[derive(Debug, Default)]
pub struct MouseState {
    pub time_mouse_down: Option<SystemTime>,
    pub active_square: Option<Square>,
    pub drag_state: DragState,
    pub mouse_vec: Vec2,
}

impl MouseState {
    pub fn handle_events(&mut self, game_state: &GameState) {
        let mouse_pos = mouse_position();
        self.mouse_vec = Vec2::from(mouse_pos);

        if self.mouse_vec.x < 0.0
            || self.mouse_vec.y < 0.0
            || self.mouse_vec.x > WINDOW_SIZE
            || self.mouse_vec.y > WINDOW_SIZE
        {
            return;
        }

        let square_at_mouse_pos = Square {
            column: 7 - (self.mouse_vec.y / SQUARE_SIZE).floor() as u8,
            row: (self.mouse_vec.x / SQUARE_SIZE).floor() as u8,
        };

        if is_mouse_button_pressed(MouseButton::Left)
            && self.active_square.is_none()
            && let Some(piece) = game_state.get_piece_at_square(&square_at_mouse_pos)
            && can_touch_piece(game_state, &square_at_mouse_pos)
        {
            self.time_mouse_down = Some(SystemTime::now());

            self.active_square = Some(square_at_mouse_pos);

            self.drag_state = DragState::Pending(
                piece,
                Vec2 {
                    x: self.mouse_vec.x % SQUARE_SIZE,
                    y: self.mouse_vec.y % SQUARE_SIZE,
                },
            );
        }

        // If we're pending for drag & drop, wait until hold-timeout passed or user started dragging the piece
        if let Some(time_down) = self.time_mouse_down
            && is_mouse_button_down(MouseButton::Left)
            && let DragState::Pending(piece, pos) = self.drag_state
        {
            let now = SystemTime::now();
            let time_delta = now
                .duration_since(time_down)
                .expect("Time should move forward");

            if time_delta.as_millis() > 200 || self.active_square != Some(square_at_mouse_pos) {
                // The user is holding the mouse
                self.drag_state = DragState::Dragging(piece, pos);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            let target_square = Square {
                column: 7 - (self.mouse_vec.y / SQUARE_SIZE).floor() as u8,
                row: (self.mouse_vec.x / SQUARE_SIZE).floor() as u8,
            };

            if !matches!(self.drag_state, DragState::Pending(..))
                && let Some(source_square) = self.active_square
                && let Some(source_piece) = game_state.get_piece_at_square(&source_square)
            {
                // User either dragged or clicked a piece to new location
                self.drag_state = DragState::Dropped(source_piece, source_square, target_square);
                self.active_square = None;
            } else if can_touch_piece(game_state, &square_at_mouse_pos) {
                // User clicked with no square yet highlighted
                self.active_square = Some(target_square);
                self.drag_state = DragState::No;
            }

            self.time_mouse_down = None;
        }
    }

    pub fn reset(&mut self) {
        self.time_mouse_down = None;
        self.active_square = None;
        self.drag_state = DragState::default();
    }
}

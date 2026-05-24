use macroquad::prelude::*;

use crate::game_state::{GameState, Square};
use crate::gui::drawing::{Textures, draw_game_state};

mod game_state;
mod gui;

const WINDOW_SIZE: f32 = 500.0;
const SQUARE_SIZE: f32 = WINDOW_SIZE / 8.0;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Gargantua"),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let textures = Textures::load().await;

    let mut game_state = GameState::initial_position();
    let mut highlighted_field: Option<Square> = None;

    loop {
        draw_game_state(&game_state, &textures, &highlighted_field, WINDOW_SIZE);

        if is_mouse_button_released(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let square = Square {
                column: 7 - (mouse_pos.1 / SQUARE_SIZE).floor() as u8,
                row: (mouse_pos.0 / SQUARE_SIZE).floor() as u8,
            };

            if let Some(source_square) = highlighted_field {
                // Move piece to new location
                game_state.replace_piece(&square, game_state.get_piece_at_square(&source_square));
                game_state.replace_piece(&source_square, None);
                highlighted_field = None;
            } else {
                highlighted_field = Some(square);
            }
        }

        next_frame().await
    }
}

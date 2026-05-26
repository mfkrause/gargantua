use macroquad::prelude::*;

use crate::game::game_state::GameState;
use crate::game::rules::can_move_piece;
use crate::gui::drawing::{Textures, draw_game_state};
use crate::gui::mouse::{DragState, MouseState};
use crate::utils::constants::WINDOW_SIZE;

mod game;
mod gui;
mod utils;

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
    let mut mouse_state = MouseState::default();

    loop {
        mouse_state.handle_events(&game_state);

        if let DragState::Dropped(piece, source_square, target_square) = mouse_state.drag_state {
            if can_move_piece(&game_state, &source_square, &target_square).unwrap_or(false) {
                game_state.replace_piece(&target_square, Some(piece));
                game_state.replace_piece(&source_square, None);
            }
            mouse_state.reset();
        }

        draw_game_state(&game_state, &textures, &mouse_state, WINDOW_SIZE);

        next_frame().await
    }
}

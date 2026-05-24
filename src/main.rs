use macroquad::prelude::*;

use crate::game_state::GameState;
use crate::gui::drawing::{Textures, draw_game_state};
use crate::gui::mouse::MouseState;
use crate::utils::constants::WINDOW_SIZE;

mod game_state;
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
        mouse_state.handle_events(&mut game_state);
        draw_game_state(&game_state, &textures, &mouse_state, WINDOW_SIZE);

        next_frame().await
    }
}

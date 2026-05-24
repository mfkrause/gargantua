use macroquad::prelude::*;

use crate::game_state::GameState;

mod game_state;

const WINDOW_SIZE: i32 = 500;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Gargantua"),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let game_state = GameState::initial_position();

    loop {
        clear_background(BLUE);

        for (x, board_row) in game_state.board.iter().enumerate() {
            for (y, board_col) in board_row.iter().enumerate() {
                draw_rectangle(
                    (x as f32) * ((WINDOW_SIZE as f32) / 8.0),
                    (y as f32) * ((WINDOW_SIZE as f32) / 8.0),
                    (WINDOW_SIZE as f32) / 8.0,
                    (WINDOW_SIZE as f32) / 8.0,
                    if (x + y) % 2 == 0 {
                        Color::from_hex(0xE2E2E2)
                    } else {
                        Color::from_hex(0x6E7E85)
                    },
                );
            }
        }

        next_frame().await
    }
}

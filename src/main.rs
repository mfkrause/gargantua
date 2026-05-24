use macroquad::prelude::*;

use crate::game_state::GameState;

mod game_state;

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
    let game_state = GameState::initial_position();

    loop {
        clear_background(BLUE);

        for (y, board_col) in game_state.board.iter().enumerate() {
            for (x, piece) in board_col.iter().enumerate() {
                let square_x = (x as f32) * (WINDOW_SIZE / 8.0);
                let square_y = (y as f32) * (WINDOW_SIZE / 8.0);

                // Draw square
                draw_rectangle(
                    square_x,
                    square_y,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    if (x + y) % 2 == 0 {
                        Color::from_hex(0xE2E2E2)
                    } else {
                        Color::from_hex(0x6E7E85)
                    },
                );

                // Draw piece
                if let Some(p) = piece {
                    p.draw(square_x, square_y, SQUARE_SIZE, SQUARE_SIZE).await;
                }
            }
        }

        next_frame().await
    }
}

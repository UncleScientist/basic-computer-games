use macroquad::prelude::*;

mod grid;
use grid::Grid;

use checkers::{Checkers, NextMove};

enum CurrentState {
    ComputerMove,
    PlayerFrom,
    PlayerTo,
    PlayerJumpAgain,
    Win,
    Lose,
}

#[macroquad::main("Checkers")]
async fn main() {
    let mut grid = Grid::new(20.0, 8, 8);
    let mut checkers = Checkers::new();
    let mut state = CurrentState::ComputerMove;

    let mut computer_message = "".to_string();
    let mut prompt = "".to_string();
    let mut player_from = (0, 0);

    loop {
        match state {
            CurrentState::ComputerMove => {
                let cmove = checkers.computer_move();
                computer_message = format!("{cmove}");
                prompt = "Click on a checker to move".to_string();
                state = CurrentState::PlayerFrom;
            }

            CurrentState::PlayerFrom => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some((x, y)) = grid.position_to_square(&loc) {
                        let row = y - 1;
                        let col = x - 1;
                        if checkers.is_black(row, col) {
                            player_from = (row, col);
                            state = CurrentState::PlayerTo;
                            prompt = "Click on a square to move to".to_string();
                        }
                    }
                }
            }

            CurrentState::PlayerTo => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some((x, y)) = grid.position_to_square(&loc) {
                        let row = y - 1;
                        let col = x - 1;
                        if checkers.is_empty(row, col) {
                            let player_to = (row, col);
                            state = CurrentState::PlayerTo;
                            match checkers.player_move(
                                7 - player_from.0,
                                player_from.1,
                                7 - player_to.0,
                                player_to.1,
                            ) {
                                Ok(NextMove::JumpAgain) => {
                                    player_from = player_to;
                                    prompt =
                                        "Click on next square to jump to (or outside board to end)"
                                            .to_string();
                                    state = CurrentState::PlayerJumpAgain;
                                }
                                Ok(NextMove::ComputerGoes) => {
                                    prompt = "".to_string();
                                    state = CurrentState::ComputerMove;
                                }
                                Err(e) => println!("{e:?}"),
                            }
                        }
                    }
                }
            }
            CurrentState::PlayerJumpAgain => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some((x, y)) = grid.position_to_square(&loc) {
                        let row = y - 1;
                        let col = x - 1;
                        if row.abs_diff(player_from.0) != 2 || !checkers.is_empty(row, col) {
                            prompt = "".to_string();
                            state = CurrentState::ComputerMove;
                        } else {
                            let player_to = (row, col);
                            match checkers.player_move(
                                7 - player_from.0,
                                player_from.1,
                                7 - player_to.0,
                                player_to.1,
                            ) {
                                Ok(NextMove::JumpAgain) => {
                                    player_from = player_to;
                                }
                                Ok(NextMove::ComputerGoes) => {
                                    prompt = "".to_string();
                                    state = CurrentState::ComputerMove;
                                }
                                Err(e) => println!("{e:?}"),
                            }
                        }
                    }
                }
            }
            CurrentState::Win => todo!(),
            CurrentState::Lose => todo!(),
        }

        clear_background(GRAY);

        draw_text(&computer_message, 20.0, 20.0, 25.0, WHITE);

        let h = screen_height();
        draw_text(&prompt, 20.0, h - 20.0, 25.0, WHITE);

        let grid_version = checkers.get_board();
        let mut vec_version = Vec::new();
        for row in grid_version {
            vec_version.push(row.to_vec());
        }
        grid.update(&vec_version);
        grid.draw();
        next_frame().await
    }
}

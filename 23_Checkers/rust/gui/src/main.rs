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

    let mut computer_message = String::new();
    let mut prompt = String::new();
    let mut player_from = (0, 0);
    let mut frame_count = 0;

    loop {
        let mut hide = None;
        match state {
            CurrentState::ComputerMove => {
                let cmove = checkers.computer_move();
                computer_message = format!("Computer: {cmove}");
                prompt = "Player: Click on a checker to move".to_string();

                state = match checkers.board_state() {
                    checkers::BoardState::RedWins => CurrentState::Lose,
                    checkers::BoardState::BlackWins => CurrentState::Win,
                    checkers::BoardState::GameContinues => CurrentState::PlayerFrom,
                };
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
                            prompt = "Player: Click on a square to move to".to_string();
                        }
                        frame_count = 0;
                    }
                }
            }

            CurrentState::PlayerTo => {
                frame_count += 1;
                let frame_frac = frame_count % 60;

                if (0..30).contains(&frame_frac) {
                    hide = Some(player_from);
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some((x, y)) = grid.position_to_square(&loc) {
                        let row = y - 1;
                        let col = x - 1;
                        if checkers.is_black(row, col) {
                            frame_count = 0;
                            player_from = (row, col);
                        } else if checkers.is_empty(row, col) {
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
                            state = match checkers.board_state() {
                                checkers::BoardState::RedWins => CurrentState::Lose,
                                checkers::BoardState::BlackWins => CurrentState::Win,
                                checkers::BoardState::GameContinues => state,
                            };
                        }
                    }
                }
            }
            CurrentState::PlayerJumpAgain => {
                frame_count += 1;
                let frame_frac = frame_count % 60;

                if (0..30).contains(&frame_frac) {
                    hide = Some(player_from);
                }
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
                            state = match checkers.board_state() {
                                checkers::BoardState::RedWins => CurrentState::Lose,
                                checkers::BoardState::BlackWins => CurrentState::Win,
                                checkers::BoardState::GameContinues => state,
                            };
                        }
                    }
                }
            }
            CurrentState::Win => {
                prompt = "Player: WINS".to_string();
                computer_message = "Computer: YOU WIN!".to_string();
            }
            CurrentState::Lose => {
                prompt = "Player: LOSES".to_string();
                computer_message = "Computer: I WIN!".to_string();
            }
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
        grid.draw(hide);
        next_frame().await
    }
}

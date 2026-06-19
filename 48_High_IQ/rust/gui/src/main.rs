use macroquad::prelude::*;

use high_iq::*;

mod grid;
use grid::Grid;

enum State {
    SelectPeg,
    SelectHole,
    GameOver(usize),
}

#[macroquad::main("H-I-Q")]
async fn main() {
    let mut grid = Grid::new(20.0, 7, 7);
    let mut game = HighIq::new();
    let mut state = State::SelectPeg;
    let mut frame_count = 0;
    let mut from_spot = 0;
    let mut coords = (0usize, 0usize);

    loop {
        let mut hide = None;

        match state {
            State::SelectPeg => {
                draw_text("Select a peg to move", 15.0, 15.0, 20.0, WHITE);
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some(pos) = grid.position_to_square(&loc)
                        && let Some(from) = game.get_board_position(pos.0 - 1, pos.1 - 1)
                        && game.has_peg_at(from)
                    {
                        from_spot = from;
                        coords = (pos.1 - 1, pos.0 - 1);
                        frame_count = 0;
                        state = State::SelectHole;
                    }
                }
            }
            State::SelectHole => {
                frame_count += 1;
                if frame_count < 30 {
                    hide = Some(coords);
                } else if frame_count >= 60 {
                    frame_count = 0;
                    hide = Some(coords);
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some(pos) = grid.position_to_square(&loc)
                        && let Some(to) = game.get_board_position(pos.0 - 1, pos.1 - 1)
                    {
                        if game.has_peg_at(to) {
                            from_spot = to;
                            coords = (pos.1 - 1, pos.0 - 1);
                            frame_count = 0;
                        } else if game.make_move(from_spot, to).is_ok() {
                            match game.check_board() {
                                GameState::MovesRemaining => state = State::SelectPeg,
                                GameState::GameOver(pegs) => state = State::GameOver(pegs),
                            }
                        }
                    }
                }
                draw_text("Where should the peg jump to?", 15.0, 15.0, 20.0, WHITE);
            }

            State::GameOver(pegs) => {
                if pegs > 1 {
                    draw_text(
                        format!("Game over! You have {pegs} pegs remaining."),
                        15.0,
                        15.0,
                        20.0,
                        WHITE,
                    );
                } else {
                    draw_text(
                        "Game over! You cleared all but one peg!",
                        15.0,
                        15.0,
                        20.0,
                        WHITE,
                    );
                    let h = screen_height();
                    draw_text(
                        "Bravo! You made a perfect score! Save a screenshot!",
                        15.0,
                        h - 15.0,
                        20.0,
                        WHITE,
                    );
                }
            }
        }

        grid.update(game.get_board());
        grid.draw(hide);
        next_frame().await;
    }
}

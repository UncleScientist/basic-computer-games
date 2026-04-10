use life_for_two::{Board, Piece};
use macroquad::prelude::*;

const LINE_SPACE: f32 = 18.0;

enum GameState {
    Player1Setup,
    Player2Setup,
    Player1Move,
    FlashPlayer1,
    Player2Move,
    WaitForClick,
    End,
}

#[macroquad::main("Life for Two")]
async fn main() {
    let mut board = Board::new();
    let mut state = GameState::Player1Setup;
    let mut setup_count = 3;
    let mut player1_move = (0, 0);
    let mut flash_state = false;
    let mut flash_frames = 0;
    let mut flash_count = 3;

    loop {
        clear_background(BLACK);

        match state {
            GameState::Player1Setup => {
                draw_board(&board, Piece::Empty);

                bottom_message("Player 1 - Select 3 Squares");

                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some(square) = position_to_square(&loc) {
                        if board.is_empty(square.0 + 1, square.1 + 1) {
                            board.place_piece(Piece::Player1, square.0 + 1, square.1 + 1);
                            setup_count -= 1;
                            if setup_count == 0 {
                                state = GameState::Player2Setup;
                                setup_count = 3;
                            }
                        }
                    }
                }
            }
            GameState::Player2Setup => {
                draw_board(&board, Piece::Player1);

                bottom_message("Player 2 - Select 3 Squares");

                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some(square) = position_to_square(&loc) {
                        if board.is_empty(square.0 + 1, square.1 + 1) {
                            board.place_piece(Piece::Player2, square.0 + 1, square.1 + 1);
                            setup_count -= 1;
                            if setup_count == 0 {
                                state = GameState::Player1Move;
                                setup_count = 3;
                            }
                        }
                    }
                }
            }
            GameState::Player1Move => {
                draw_board(&board, Piece::Empty);
                bottom_message("Player 1 - Select Empty Square");
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some(square) = position_to_square(&loc) {
                        if board.is_empty(square.0 + 1, square.1 + 1) {
                            player1_move = square;
                            state = GameState::FlashPlayer1;
                        }
                    }
                }
            }
            GameState::FlashPlayer1 => {
                if flash_count == 0 {
                    state = GameState::Player2Move;
                } else if flash_state {
                    if flash_frames == 0 {
                        flash_state = false;
                        flash_count -= 1;
                        flash_frames = 40;
                        board.clear_piece(player1_move.0 + 1, player1_move.1 + 1);
                    } else {
                        flash_frames -= 1;
                    }
                } else {
                    if flash_frames == 0 {
                        flash_state = true;
                        flash_frames = 40;
                        board.place_piece(Piece::Player1, player1_move.0 + 1, player1_move.1 + 1);
                    } else {
                        flash_frames -= 1;
                    }
                }
                draw_board(&board, Piece::Empty);
            }
            GameState::Player2Move => {
                draw_board(&board, Piece::Empty);
                bottom_message("Player 2 - Select Empty Square");
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    if let Some(square) = position_to_square(&loc) {
                        if board.is_empty(square.0 + 1, square.1 + 1) {
                            player1_move = square;
                            state = GameState::WaitForClick;
                        }
                    }
                }
            }
            GameState::WaitForClick => {
                draw_board(&board, Piece::Empty);
                bottom_message("Click board to generate next stage");
                if is_mouse_button_pressed(MouseButton::Left) {
                    break;
                }
            }
            GameState::End => todo!(),
        }

        next_frame().await
    }
}

fn draw_board(board: &Board, hide: Piece) {
    let w = screen_width();
    let h = screen_height();

    let grid_width = w - LINE_SPACE * 4.0;
    let grid_height = h - LINE_SPACE * 4.0;

    let mut vertical_pos = LINE_SPACE * 2.0;
    while vertical_pos < w - LINE_SPACE * 2.0 + 1.0 {
        draw_line(
            vertical_pos,
            LINE_SPACE * 2.0,
            vertical_pos,
            grid_height + LINE_SPACE * 2.0,
            2.0,
            GRAY,
        );
        vertical_pos += grid_width / 5.0;
    }

    let mut horizontal_pos = LINE_SPACE * 2.0;
    while horizontal_pos < h - LINE_SPACE * 2.0 + 1.0 {
        draw_line(
            LINE_SPACE * 2.0,
            horizontal_pos,
            grid_width + LINE_SPACE * 2.0,
            horizontal_pos,
            2.0,
            GRAY,
        );
        horizontal_pos += grid_height / 5.0;
    }

    let piece_x = LINE_SPACE * 2.0 + grid_width / 10.0;
    let piece_y = LINE_SPACE * 2.0 + grid_height / 10.0;

    let star = measure_text("*", None, 96, 1.0);
    let hash = measure_text("#", None, 96, 1.0);
    let grid = board.get_grid();
    for (r, row) in grid.iter().enumerate() {
        for (c, piece) in row.iter().enumerate() {
            let xpos = piece_x + c as f32 * grid_width / 5.0;
            let ypos = piece_y + r as f32 * grid_height / 5.0;
            draw_circle(xpos, ypos, 1.0, WHITE);
            match piece {
                Piece::Player1 if hide != Piece::Player1 => {
                    draw_text(
                        "*",
                        xpos - star.width / 2.0,
                        ypos + star.height / 2.0,
                        96.0,
                        WHITE,
                    );
                }
                Piece::Player2 if hide != Piece::Player2 => {
                    draw_text(
                        "#",
                        xpos - hash.width / 2.0,
                        ypos + hash.height / 2.0,
                        96.0,
                        WHITE,
                    );
                }
                _ => {}
            }
        }
    }
}

fn position_to_square(loc: &(f32, f32)) -> Option<(usize, usize)> {
    if loc.0 < LINE_SPACE * 2.0 || loc.1 < LINE_SPACE * 2.0 {
        return None;
    }

    let w = screen_width();
    let h = screen_height();

    let grid_width = w - LINE_SPACE * 4.0;
    let grid_height = h - LINE_SPACE * 4.0;

    if loc.0 > LINE_SPACE * 2.0 + grid_width || loc.1 > LINE_SPACE * 2.0 + grid_height {
        return None;
    }

    let x = ((loc.0 - LINE_SPACE * 2.0) / (grid_width / 5.0)) as usize;
    let y = ((loc.1 - LINE_SPACE * 2.0) / (grid_height / 5.0)) as usize;

    Some((x, y))
}

fn bottom_message<S: AsRef<str>>(msg: S) {
    let h = screen_height();
    let text_size = measure_text(msg.as_ref(), None, 24, 1.0);
    center_text(msg.as_ref(), h - text_size.height, screen_width());
}

fn center_text<S: AsRef<str>>(text: S, ypos: f32, width: f32) {
    let center = get_text_center(text.as_ref(), Option::None, 24, 1., 0.);

    draw_text(text.as_ref(), width / 2. - center[0], ypos, 24., WHITE);
}

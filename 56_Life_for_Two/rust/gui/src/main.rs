use life_for_two::{Board, Piece};
use macroquad::prelude::*;

const LINE_SPACE: f32 = 18.0;

const INTRO_FRAMES: usize = 240;

const INITIAL_COUNT: usize = 3;

enum GameState {
    Intro(usize),
    Player1Setup(usize),
    Player2Setup(usize),
    Player1Move,
    FlashPlayer1((usize, usize), Flash),
    Player2Move((usize, usize)),
    WaitForClick(bool),
    CalculateLife,
    End(usize, usize),
}

enum FlashState {
    Off,    // we just turned off for this frame
    On,     // we just turned on for this frame
    Steady, // we didn't change state this frame
    Done,   // we're done
}

struct Flash {
    state: bool,
    frames: usize,
    count: usize,
}

impl Flash {
    fn new() -> Self {
        Self {
            state: false,
            count: 3,
            frames: 0,
        }
    }

    // returns true when we're done flashing
    fn next_frame(&mut self) -> FlashState {
        if self.count == 0 {
            return FlashState::Done;
        }

        if self.state {
            if self.frames == 0 {
                self.state = false;
                self.count -= 1;
                self.frames = 40;
                FlashState::Off
            } else {
                self.frames -= 1;
                FlashState::Steady
            }
        } else if self.frames == 0 {
            self.state = true;
            self.frames = 40;
            FlashState::On
        } else {
            self.frames -= 1;
            FlashState::Steady
        }
    }
}

#[macroquad::main("Life for Two")]
async fn main() {
    let mut board = Board::new();
    let mut state = GameState::Intro(INTRO_FRAMES);

    loop {
        clear_background(BLACK);

        match &mut state {
            GameState::Intro(frames) => {
                let w = screen_width();
                center_text("Life for Two", LINE_SPACE * 5.0, w);
                center_text(
                    "Creative Computing  Morristown, New Jersey",
                    LINE_SPACE * 7.0,
                    w,
                );
                *frames -= 1;
                if *frames == 0 {
                    state = GameState::Player1Setup(INITIAL_COUNT);
                }
            }
            GameState::Player1Setup(pieces_remaining) => {
                draw_board(&board, Piece::Empty);

                bottom_message(format!("Player 1 - Select {INITIAL_COUNT} Squares"));

                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    let Some(square) = position_to_square(&loc) else {
                        continue;
                    };
                    if board.is_empty(square.0, square.1) {
                        board.place_piece(Piece::Player1, square.0, square.1);
                        *pieces_remaining -= 1;
                        if *pieces_remaining == 0 {
                            state = GameState::Player2Setup(INITIAL_COUNT);
                        }
                    }
                }
            }
            GameState::Player2Setup(pieces_remaining) => {
                draw_board(&board, Piece::Player1);

                bottom_message(format!("Player 2 - Select {INITIAL_COUNT} Squares"));

                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    let Some(square) = position_to_square(&loc) else {
                        continue;
                    };
                    if board.is_empty(square.0, square.1) {
                        board.place_piece(Piece::Player2, square.0, square.1);
                        *pieces_remaining -= 1;
                        if *pieces_remaining == 0 {
                            state = GameState::WaitForClick(false);
                        }
                    }
                }
            }
            GameState::Player1Move => {
                draw_board(&board, Piece::Empty);
                bottom_message("Player 1 - Select Empty Square");
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    let Some(square) = position_to_square(&loc) else {
                        continue;
                    };
                    if board.is_empty(square.0, square.1) {
                        state = GameState::FlashPlayer1(square, Flash::new());
                    }
                }
            }
            GameState::FlashPlayer1(player1_move, flasher) => {
                match flasher.next_frame() {
                    FlashState::Off => {
                        board.clear_piece(player1_move.0, player1_move.1);
                    }
                    FlashState::On => {
                        board.place_piece(Piece::Player1, player1_move.0, player1_move.1);
                    }
                    FlashState::Steady => {}
                    FlashState::Done => {
                        state = GameState::Player2Move(*player1_move);
                    }
                };
                draw_board(&board, Piece::Empty);
            }
            GameState::Player2Move(player1_move) => {
                draw_board(&board, Piece::Empty);
                bottom_message("Player 2 - Select Empty Square");
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    let Some(square) = position_to_square(&loc) else {
                        continue;
                    };
                    if board.is_empty(square.0, square.1) {
                        let player2_move = square;

                        if *player1_move != player2_move {
                            board.place_piece(Piece::Player1, player1_move.0, player1_move.1);
                            board.place_piece(Piece::Player2, player2_move.0, player2_move.1);
                        }
                        state = GameState::WaitForClick(*player1_move == player2_move);
                    }
                }
            }
            GameState::WaitForClick(same_spot) => {
                draw_board(&board, Piece::Empty);
                if *same_spot {
                    top_message("Players chose the same location - leaving it empty!");
                }
                bottom_message("Click board to generate next stage");
                if is_mouse_button_pressed(MouseButton::Left) {
                    state = GameState::CalculateLife;
                }
            }
            GameState::CalculateLife => {
                let (p1count, p2count) = board.step();
                if p1count == 0 || p2count == 0 {
                    state = GameState::End(p1count, p2count);
                } else {
                    state = GameState::Player1Move;
                }
            }
            GameState::End(p1count, p2count) => {
                draw_board(&board, Piece::Empty);
                match (p1count, p2count) {
                    (0, 0) => top_message("Game ends in a draw!"),
                    (0, _) => top_message("Player 2 is the winner!"),
                    (_, 0) => top_message("Player 1 is the winner!"),
                    _ => unreachable!(),
                };
                bottom_message("Click to exit");
                if is_mouse_button_pressed(MouseButton::Left) {
                    break;
                }
            }
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

    let font_size = 1.5 * grid_width.min(grid_height) / 5.0;
    let star = measure_text("*", None, font_size as u16, 1.0);
    let hash = measure_text("#", None, font_size as u16, 1.0);
    let grid = board.get_grid();
    for (r, row) in grid.iter().enumerate() {
        for (c, piece) in row.iter().enumerate() {
            let xpos = piece_x + c as f32 * grid_width / 5.0;
            let ypos = piece_y + r as f32 * grid_height / 5.0;
            // draw_circle(xpos, ypos, 1.0, WHITE);
            match piece {
                Piece::Player1 if hide != Piece::Player1 => {
                    draw_text(
                        "*",
                        xpos - star.width / 2.0,
                        ypos + star.height / 2.0,
                        font_size,
                        WHITE,
                    );
                }
                Piece::Player2 if hide != Piece::Player2 => {
                    draw_text(
                        "#",
                        xpos - hash.width / 2.0,
                        ypos + hash.height / 2.0,
                        font_size,
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

    Some((x + 1, y + 1))
}

fn top_message<S: AsRef<str>>(msg: S) {
    let text_size = measure_text(msg.as_ref(), None, 24, 1.0);
    center_text(msg.as_ref(), text_size.height, screen_width());
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

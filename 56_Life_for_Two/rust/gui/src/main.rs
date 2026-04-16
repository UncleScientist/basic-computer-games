use life_for_two::{Board, Piece};
use macroquad::prelude::*;

const LINE_SPACE: f32 = 18.0;

const INTRO_FRAMES: usize = 240;

const INITIAL_COUNT: usize = 3;

enum GameResult {
    Player1Wins,
    Player2Wins,
    Draw,
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
    let mut same_spot = false;

    intro(INTRO_FRAMES).await;

    board.player_setup(INITIAL_COUNT, 1).await;
    board.player_setup(INITIAL_COUNT, 2).await;

    let winner = loop {
        board.wait_for_click(same_spot).await;
        match board.step() {
            (0, 0) => break GameResult::Draw,
            (_, 0) => break GameResult::Player1Wins,
            (0, _) => break GameResult::Player2Wins,
            _ => {}
        };

        let p1_square = board.player_move(1).await;
        board.flash_position(p1_square).await;
        let p2_square = board.player_move(2).await;
        same_spot = p1_square == p2_square;

        if !same_spot {
            board.place_piece(Piece::Player1, p1_square.0, p1_square.1);
            board.place_piece(Piece::Player2, p2_square.0, p2_square.1);
        }
    };

    board.end_game(winner).await;
}

trait GuiGame {
    async fn player_setup(&mut self, count: usize, player: usize);
    async fn wait_for_click(&self, same_spot: bool);
    async fn player_move(&self, player: usize) -> (usize, usize);
    async fn flash_position(&mut self, loc: (usize, usize));
    async fn end_game(&self, winner: GameResult);

    fn draw(&self, hide: Piece);
}

impl GuiGame for Board {
    async fn player_setup(&mut self, mut pieces_remaining: usize, player: usize) {
        loop {
            clear_frame().await;
            self.draw(if player == 1 {
                Piece::Empty
            } else {
                Piece::Player1
            });

            bottom_message(format!("Player {player} - Select {INITIAL_COUNT} Squares"));

            if is_mouse_button_pressed(MouseButton::Left) {
                let loc = mouse_position();
                let Some(square) = position_to_square(&loc) else {
                    continue;
                };
                if self.is_empty(square.0, square.1) {
                    self.place_piece(
                        if player == 1 {
                            Piece::Player1
                        } else {
                            Piece::Player2
                        },
                        square.0,
                        square.1,
                    );
                    pieces_remaining -= 1;
                    if pieces_remaining == 0 {
                        return;
                    }
                }
            }
        }
    }

    async fn wait_for_click(&self, same_spot: bool) {
        loop {
            clear_frame().await;
            self.draw(Piece::Empty);
            if same_spot {
                top_message("Players chose the same location - leaving it empty!");
            }
            bottom_message("Click board to generate next stage");
            if is_mouse_button_pressed(MouseButton::Left) {
                return;
            }
        }
    }

    async fn player_move(&self, player: usize) -> (usize, usize) {
        loop {
            clear_frame().await;
            self.draw(Piece::Empty);
            bottom_message(format!("Player {player} - Select Empty Square"));
            if is_mouse_button_pressed(MouseButton::Left) {
                let loc = mouse_position();
                let Some(square) = position_to_square(&loc) else {
                    continue;
                };
                if self.is_empty(square.0, square.1) {
                    return square;
                }
            }
        }
    }

    async fn flash_position(&mut self, pos: (usize, usize)) {
        let mut flasher = Flash::new();
        loop {
            clear_frame().await;
            match flasher.next_frame() {
                FlashState::Off => {
                    self.clear_piece(pos.0, pos.1);
                }
                FlashState::On => {
                    self.place_piece(Piece::Player1, pos.0, pos.1);
                }
                FlashState::Steady => {}
                FlashState::Done => {
                    return;
                }
            };
            self.draw(Piece::Empty);
        }
    }

    async fn end_game(&self, winner: GameResult) {
        loop {
            clear_frame().await;
            self.draw(Piece::Empty);
            match winner {
                GameResult::Draw => top_message("Game ends in a draw!"),
                GameResult::Player1Wins => top_message("Player 1 is the winner!"),
                GameResult::Player2Wins => top_message("Player 2 is the winner!"),
            };
            bottom_message("Click to exit");
            if is_mouse_button_pressed(MouseButton::Left) {
                return;
            }
        }
    }

    fn draw(&self, hide: Piece) {
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
        let grid = self.get_grid();
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

async fn clear_frame() {
    next_frame().await;
    clear_background(BLACK);
}

async fn intro(mut frames: usize) {
    loop {
        clear_frame().await;
        let w = screen_width();
        center_text("Life for Two", LINE_SPACE * 5.0, w);
        center_text(
            "Creative Computing  Morristown, New Jersey",
            LINE_SPACE * 7.0,
            w,
        );
        frames -= 1;
        if frames == 0 {
            return;
        }
    }
}

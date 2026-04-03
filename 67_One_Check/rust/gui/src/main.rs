use macroquad::prelude::*;
use one_check::{Board, Piece};

const LINE_HEIGHT: f32 = 18.0;

enum DisplayState {
    Instructions,
    GetFromSquare,
    GetToSquare,
    GameOver,
}

#[macroquad::main("One Check")]
async fn main() {
    let mut board = Board::new();
    let mut display_state = DisplayState::Instructions;
    let mut highlight = 0;

    loop {
        next_frame().await;
        clear_background(BLACK);
        match display_state {
            DisplayState::Instructions => {
                if draw_instructions() {
                    display_state = DisplayState::GetFromSquare;
                }
            }
            DisplayState::GetFromSquare => {
                show_board(&board);
                draw_text(
                    "Click on a checker to select it",
                    LINE_HEIGHT,
                    LINE_HEIGHT,
                    24.,
                    WHITE,
                );
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    let Some(board_square) = position_to_square(&loc) else {
                        if do_scoring(&loc) {
                            display_state = DisplayState::GameOver;
                        }
                        continue;
                    };
                    if board.is_occupied(board_square + 1) {
                        highlight = board_square;
                        display_state = DisplayState::GetToSquare;
                    }
                }
            }
            DisplayState::GetToSquare => {
                show_board(&board);
                draw_text(
                    "Click an empty square to jump to",
                    LINE_HEIGHT,
                    LINE_HEIGHT,
                    24.,
                    WHITE,
                );
                highlight_square(highlight);
                if is_mouse_button_pressed(MouseButton::Left) {
                    let loc = mouse_position();
                    let Some(board_square) = position_to_square(&loc) else {
                        if do_scoring(&loc) {
                            display_state = DisplayState::GameOver;
                        }
                        continue;
                    };
                    if board.is_occupied(board_square + 1) {
                        highlight = board_square;
                        display_state = DisplayState::GetToSquare;
                    }
                    if board.legal_move(highlight + 1, board_square + 1) {
                        board.make_move(highlight + 1, board_square + 1);
                        display_state = DisplayState::GetFromSquare;
                    }
                }
            }
            DisplayState::GameOver => {
                if let Some(answer) = show_final_score(&board) {
                    match answer {
                        true => {
                            board = Board::new();
                            display_state = DisplayState::GetFromSquare;
                        }
                        false => {
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("O.K.   Hope you had fun!!");
}

fn show_final_score(board: &Board) -> Option<bool> {
    let w = screen_width();

    let piece_count = board
        .get_board()
        .iter()
        .filter(|p| **p == Piece::Occupied)
        .count();
    let moves = board.get_moves();
    let result = format!("You made {moves} jumps and had {piece_count} pieces");
    center_text(result, LINE_HEIGHT * 10.0, w);
    center_text("remaining on the board.", LINE_HEIGHT * 11.0, w);

    center_text("Try again?", LINE_HEIGHT * 15.0, w);

    let yes_box = text_with_border("YES", w / 2. - LINE_HEIGHT * 5.0, LINE_HEIGHT * 20.0);
    let no_box = text_with_border("NO", w / 2. + LINE_HEIGHT * 10.0, LINE_HEIGHT * 20.0);

    if is_mouse_button_pressed(MouseButton::Left) {
        let loc = mouse_position();
        if loc.0 >= yes_box.0
            && loc.0 <= yes_box.0 + yes_box.2
            && loc.1 >= yes_box.1
            && loc.1 <= yes_box.1 + yes_box.3
        {
            return Some(true);
        }
        if loc.0 >= no_box.0
            && loc.0 <= no_box.0 + no_box.2
            && loc.1 >= no_box.1
            && loc.1 <= no_box.1 + no_box.3
        {
            return Some(false);
        }
    }

    None
}

fn do_scoring(loc: &(f32, f32)) -> bool {
    if loc.0 < LINE_HEIGHT * 2.0 || loc.1 < LINE_HEIGHT * 2.0 {
        return false;
    }

    let w = screen_width();
    let h = screen_height();

    let text_size = measure_text("Finish & Score!", None, 24, 1.0);
    let (x, y, width, height) = (
        w - LINE_HEIGHT * 2.0 - text_size.width - LINE_HEIGHT,
        h - text_size.height - LINE_HEIGHT,
        text_size.width + LINE_HEIGHT * 2.0,
        text_size.height + text_size.offset_y,
    );

    loc.0 >= x && loc.0 <= x + width && loc.1 >= y && loc.1 <= y + height
}

fn highlight_square(highlight: usize) {
    let w = screen_width();
    let h = screen_height();

    let grid_width = w - LINE_HEIGHT * 4.0;
    let grid_height = h - LINE_HEIGHT * 4.0;

    let checker_x = LINE_HEIGHT * 2.0 + grid_width / 16.0;
    let checker_y = LINE_HEIGHT * 2.0 + grid_height / 16.0;
    let size = 0.9 * grid_height.min(grid_width) / 16.0;

    let xpos = (highlight % 8) as f32;
    let ypos = (highlight / 8) as f32;
    draw_rectangle_lines(
        checker_x + xpos * grid_width / 8.0 - size,
        checker_y + ypos * grid_height / 8.0 - size,
        size * 2.0,
        size * 2.0,
        2.,
        WHITE,
    );
}

fn position_to_square(loc: &(f32, f32)) -> Option<usize> {
    if loc.0 < LINE_HEIGHT * 2.0 || loc.1 < LINE_HEIGHT * 2.0 {
        return None;
    }

    let w = screen_width();
    let h = screen_height();

    let grid_width = w - LINE_HEIGHT * 4.0;
    let grid_height = h - LINE_HEIGHT * 4.0;

    if loc.0 > LINE_HEIGHT * 2.0 + grid_width || loc.1 > LINE_HEIGHT * 2.0 + grid_height {
        return None;
    }

    let x = ((loc.0 - LINE_HEIGHT * 2.0) / (grid_width / 8.0)) as usize;
    let y = ((loc.1 - LINE_HEIGHT * 2.0) / (grid_height / 8.0)) as usize;

    Some((x + y * 8) as usize)
}

fn draw_instructions() -> bool {
    let w = screen_width();

    let mut ypos = 20.0;
    center_text("One Check", ypos, w);
    ypos += LINE_HEIGHT;

    center_text("Creative Computing  Morristown, New Jersey", ypos, w);
    ypos += LINE_HEIGHT * 2.0;

    center_text("Solitare Checker Puzzle by David Ahl", ypos, w);
    ypos += LINE_HEIGHT * 3.0;

    let instruction_text = [
        "48 checkers are placed on the 2 outside spaces of a",
        "standard 64-square checkerboard. The object is to",
        "remove as many checkers as possible by diagonal jumps",
        "(as in standard checkers). Use the numbered board to",
        "indicate the square you wish to jump from and to. On",
        "the board printed out on each turn '1' indicates a",
        "checker and '0' and empty square. When you have no",
        "possible jumps remaining, input a '0' in response to",
        "question 'jump from?'",
    ];

    for line in instruction_text {
        center_text(line, ypos, w);
        ypos += LINE_HEIGHT;
    }

    ypos += LINE_HEIGHT * 2.0;

    let (x, y, width, height) = (w / 2.0 - 100.0, ypos, 200.0, LINE_HEIGHT * 3.);
    draw_rectangle_lines(x, y, width, height, 2., BLUE);
    let center = get_text_center("PLAY!", Option::None, 48, 1., 0.);
    draw_text(
        "PLAY!",
        w / 2. - center[0],
        ypos + LINE_HEIGHT * 2.0,
        48.,
        WHITE,
    );

    if is_mouse_button_pressed(MouseButton::Left) {
        let loc = mouse_position();
        loc.0 >= x && loc.0 <= (x + width) && loc.1 >= y && loc.1 <= (y + height)
    } else {
        false
    }
}

fn center_text<S: AsRef<str>>(text: S, ypos: f32, width: f32) {
    let center = get_text_center(text.as_ref(), Option::None, 24, 1., 0.);

    draw_text(text.as_ref(), width / 2. - center[0], ypos, 24., WHITE);
}

fn show_board(board: &Board) {
    let w = screen_width();
    let h = screen_height();

    let grid_width = w - LINE_HEIGHT * 4.0;
    let grid_height = h - LINE_HEIGHT * 4.0;

    let mut vertical_pos = LINE_HEIGHT * 2.0;
    while vertical_pos < w - LINE_HEIGHT * 2.0 + 1.0 {
        draw_line(
            vertical_pos,
            LINE_HEIGHT * 2.0,
            vertical_pos,
            grid_height + LINE_HEIGHT * 2.0,
            2.0,
            GRAY,
        );
        vertical_pos += grid_width / 8.0;
    }

    let mut horizontal_pos = LINE_HEIGHT * 2.0;
    while horizontal_pos < h - LINE_HEIGHT * 2.0 + 1.0 {
        draw_line(
            LINE_HEIGHT * 2.0,
            horizontal_pos,
            grid_width + LINE_HEIGHT * 2.0,
            horizontal_pos,
            2.0,
            GRAY,
        );
        horizontal_pos += grid_height / 8.0;
    }

    let checker_x = LINE_HEIGHT * 2.0 + grid_width / 16.0;
    let checker_y = LINE_HEIGHT * 2.0 + grid_height / 16.0;
    let radius = 0.8 * grid_height.min(grid_width) / 16.0;

    let grid = board.get_board();
    for (pos, g) in grid.iter().enumerate() {
        let xpos = (pos % 8) as f32;
        let ypos = (pos / 8) as f32;
        if *g == Piece::Occupied {
            draw_circle(
                checker_x + xpos * grid_width / 8.0,
                checker_y + ypos * grid_height / 8.0,
                radius,
                RED,
            );
            draw_circle_lines(
                checker_x + xpos * grid_width / 8.0,
                checker_y + ypos * grid_height / 8.0,
                radius * 0.6,
                2.0,
                MAROON,
            );
            draw_circle_lines(
                checker_x + xpos * grid_width / 8.0,
                checker_y + ypos * grid_height / 8.0,
                radius * 0.95,
                2.0,
                MAROON,
            );
        }
    }

    text_with_border("Finish & Score!", w - LINE_HEIGHT * 2.0, h);
}

fn text_with_border<S: AsRef<str>>(text: S, xpos: f32, ypos: f32) -> (f32, f32, f32, f32) {
    let text_size = measure_text(text.as_ref(), None, 24, 1.0);
    draw_text(
        text.as_ref(),
        xpos - LINE_HEIGHT * 2.0 - text_size.width,
        ypos - text_size.height,
        24.0,
        WHITE,
    );

    let (x, y, w, h) = (
        xpos - LINE_HEIGHT * 2.0 - text_size.width - LINE_HEIGHT,
        ypos - text_size.height - LINE_HEIGHT,
        text_size.width + LINE_HEIGHT * 2.0,
        text_size.height + text_size.offset_y,
    );
    draw_rectangle_lines(x, y, w, h, 1.5, BLUE);
    (x, y, w, h)
}

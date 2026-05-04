use macroquad::prelude::*;

use chomp::{Chomp, ChompResult};

#[macroquad::main("Chomp")]
async fn main() {
    show_instructions().await;

    let player_count = loop {
        let player_count = get_number("HOW MANY PLAYERS").await;
        if (1..=9).contains(&player_count) {
            break player_count;
        }
    };

    let rows = loop {
        let rows = get_number("HOW MANY ROWS").await;
        if (1..=9).contains(&rows) {
            break rows;
        }
    };

    let cols = loop {
        let cols = get_number("HOW MANY COLUMNS").await;
        if (1..=9).contains(&cols) {
            break rows;
        }
    };

    let mut game = Chomp::new(rows, cols);

    let loser = play_game(&mut game, player_count).await;
    println!("Player {loser} lost!");
}

async fn play_game(game: &mut Chomp, player_count: usize) -> usize {
    let mut cur_player = 0;
    loop {
        clear_background(BLACK);

        let (row, col) = get_coords(
            game,
            &format!(
                "PLAYER {} COORDINATES OF CHOMP (ROW,COLUMN)",
                cur_player + 1
            ),
        )
        .await;
        match game.chomp(row - 1, col - 1) {
            ChompResult::Invalid => {}
            ChompResult::Safe => {
                cur_player = (cur_player + 1) % player_count;
            }
            ChompResult::Dead => {
                return cur_player + 1;
            }
        }

        next_frame().await;
    }
}

const LINE_SPACE: f32 = 20.0;
const GRID_SIZE: f32 = 9.0;

fn draw_board(board: &Chomp) {
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
        vertical_pos += grid_width / GRID_SIZE;
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
        horizontal_pos += grid_height / GRID_SIZE;
    }

    let piece_x = LINE_SPACE * 2.0 + grid_width / (GRID_SIZE * 2.0);
    let piece_y = LINE_SPACE * 2.0 + grid_height / (GRID_SIZE * 2.0);

    let font_size = 1.5 * grid_width.min(grid_height) / GRID_SIZE;
    let star = measure_text("*", None, font_size as u16, 1.0);
    let grid = board.game_grid();
    for (r, cols) in grid.iter().enumerate() {
        for c in 0..*cols {
            let xpos = piece_x + c as f32 * grid_width / GRID_SIZE;
            let ypos = piece_y + r as f32 * grid_height / GRID_SIZE;
            draw_text(
                "*",
                xpos - star.width / 2.0,
                ypos + star.height / 2.0,
                font_size,
                WHITE,
            );
        }
    }
}

async fn show_instructions() {
    loop {
        const INSTRUCTIONS: [&str; 15] = [
            "CHOMP IS FOR 1 OR MORE PLAYERS (HUMANS ONLY).",
            "",
            "HERE'S HOW A BOARD LOOKS (THIS ONE IS 5 BY 7):",
            "",
            "THE BOARD IS A BIG COOKIE - R ROWS HIGH AND C COLUMNS",
            "WIDE. YOU INPUT R AND C AT THE START. IN THE UPPER LEFT",
            "CORNER OF THE COOKIE IS A POISON SQUARE (P). THE ONE WHO",
            "CHOMPS THE POISON SQUARE LOSES. TO TAKE A CHOMP, TYPE THE",
            "ROW AND COLUMN OF ONE OF THE SQUARES ON THE COOKIE.",
            "ALL OF THE SQUARES BELOW AND TO THE RIGHT OF THAT SQUARE",
            "(INCLUDING THAT SQUARE, TOO) DISAPPEAR -- CHOMP!!",
            "NO FAIR CHOMPING SQUARES THAT HAVE ALREADY BEEN CHOMPED,",
            "OR THAT ARE OUTSIDE THE ORIGINAL DIMENSIONS OF THE COOKIE.",
            "",
            "HERE WE GO...",
        ];

        let mut ypos = 20.;
        for line in &INSTRUCTIONS {
            draw_text(line, 10., ypos, 30., WHITE);
            ypos += 20.;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        next_frame().await;
    }
}

async fn get_number<S: AsRef<str>>(prompt: S) -> usize {
    loop {
        let num = prompt_for_input(prompt.as_ref()).await;
        if let Ok(num) = num.parse::<usize>() {
            return num;
        }
    }
}

async fn get_coords<S: AsRef<str>>(game: &Chomp, prompt: S) -> (usize, usize) {
    loop {
        clear_background(BLACK);
        draw_board(game);
        draw_text(prompt.as_ref(), 10., 20., 30., WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let loc = mouse_position();
            if let Some(square) = position_to_square(&loc) {
                return (square.1, square.0);
            }
        }

        next_frame().await;
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

    let x = ((loc.0 - LINE_SPACE * 2.0) / (grid_width / GRID_SIZE)) as usize;
    let y = ((loc.1 - LINE_SPACE * 2.0) / (grid_height / GRID_SIZE)) as usize;

    Some((x + 1, y + 1))
}

async fn prompt_for_input<S: AsRef<str>>(prompt: S) -> String {
    let text = prompt.as_ref();
    let mut input_buffer = String::new();
    let mut frame_count = 0;
    loop {
        next_frame().await;
        frame_count += 1;
        let prompt = if (frame_count % 60) < 30 {
            format!("{text}? {input_buffer}_")
        } else {
            format!("{text}? {input_buffer}")
        };

        draw_text(&prompt, 10., 20., 30., WHITE);

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Backspace => {
                    input_buffer.pop();
                }
                KeyCode::Enter => {
                    return input_buffer;
                }
                KeyCode::Key0
                | KeyCode::Key1
                | KeyCode::Key2
                | KeyCode::Key3
                | KeyCode::Key4
                | KeyCode::Key5
                | KeyCode::Key6
                | KeyCode::Key7
                | KeyCode::Key8
                | KeyCode::Key9
                | KeyCode::Comma => {
                    input_buffer.push(key as u8 as char);
                }
                _ => {}
            }
        }
    }
}

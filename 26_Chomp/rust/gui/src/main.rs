use macroquad::prelude::*;

use chomp::{Chomp, ChompResult};

mod grid;
use grid::Grid;

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
            break cols;
        }
    };

    let mut game = Chomp::new(rows, cols);

    let loser = play_game(&mut game, player_count).await;
    println!("Player {loser} lost!");
}

async fn play_game(game: &mut Chomp, player_count: usize) -> usize {
    let mut grid = Grid::new(18.0, 9, 9);

    update_board(game, &mut grid);

    let mut cur_player = 0;
    loop {
        clear_background(BLACK);

        let (row, col) = get_coords(
            &format!(
                "PLAYER {} COORDINATES OF CHOMP (ROW,COLUMN)",
                cur_player + 1
            ),
            &grid,
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

        update_board(game, &mut grid);

        next_frame().await;
    }
}

fn update_board(board: &Chomp, grid: &mut Grid) {
    let board = board.game_grid();

    let mut display = Vec::new();
    for (rowindex, r) in board.iter().enumerate() {
        let mut row = Vec::new();
        for c in 0..*r {
            if rowindex == 0 && c == 0 {
                row.push('P');
            } else {
                row.push('*');
            }
        }
        display.push(row);
    }
    grid.update(&display);
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

async fn get_coords<S: AsRef<str>>(prompt: S, board: &Grid) -> (usize, usize) {
    loop {
        clear_background(BLACK);
        board.draw();
        draw_text(prompt.as_ref(), 10., 20., 30., WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let loc = mouse_position();
            if let Some(square) = board.position_to_square(&loc) {
                return (square.1, square.0);
            }
        }

        next_frame().await;
    }
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

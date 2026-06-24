use hangman::{Guess, GuessResult, Hangman};
use macroquad::prelude::*;

#[macroquad::main("Hangman")]
async fn main() {
    let mut hangman = Hangman::new();
    while hangman.words_left() {
        println!("{:?}", hangman.current_state());
        guess_a_word(&mut hangman).await;
    }
}

async fn guess_a_word(hangman: &mut Hangman) {
    let game_won = loop {
        clear_background(BLACK);
        let w = screen_width();
        let m = measure_text("#", None, TEXT_SIZE as u16, 1.0);
        {
            let state = hangman.current_state();
            draw_letter_choices(w, &m, state);
            draw_hangman(w, state);
            draw_word_so_far(w, state);
            draw_text(
                format!(
                    "Guesses: {}/{}",
                    state.total_guesses - state.wrong_guesses,
                    state.total_guesses
                ),
                30.0,
                30.0,
                30.0,
                WHITE,
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let loc = mouse_position();
            if let Some(letter) = mouse_loc(w, &m, loc) {
                match hangman.guess_letter(letter) {
                    GuessResult::AlreadyGuessed => {}
                    GuessResult::FoundLetter(_) => {}
                    GuessResult::NotPresent => {
                        if hangman.current_state().wrong_guesses >= 10 {
                            break false;
                        }
                    }
                    GuessResult::FoundWord => {
                        break true;
                    }
                }
            }
        }

        next_frame().await;
    };

    next_frame().await;

    loop {
        clear_background(BLACK);
        let w = screen_width();
        let m = measure_text("#", None, TEXT_SIZE as u16, 1.0);
        let state = hangman.current_state();
        draw_letter_choices(w, &m, state);
        draw_hangman(w, state);
        draw_word_so_far(w, state);
        if game_won {
            draw_text("Congrats, you guessed it!", 30.0, 30.0, 30.0, WHITE);
        } else {
            draw_text(
                format!(
                    "You lost, the word was {}",
                    state.solution_word.iter().collect::<String>()
                ),
                30.0,
                30.0,
                30.0,
                WHITE,
            );
        }

        let h = screen_height();
        draw_text("Click to play again", 30.0, h - 10.0, 30.0, WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            break;
        }
        next_frame().await;
    }

    next_frame().await;
}

fn draw_letter_choices(w: f32, m: &TextDimensions, state: &Guess) {
    for row in 0..5 {
        for col in 0..5 {
            let letter = (b'A' + row as u8 * 5 + col as u8) as char;
            if !state.letters_guessed.contains(&letter) {
                draw_char_button(w, m, letter, row as f32, col as f32);
            }
        }
    }
    if !state.letters_guessed.contains(&'Z') {
        draw_char_button(w, m, 'Z', 5.0, 2.0);
    }
}

const BUTTON_SIZE: f32 = 70.0;
const TEXT_SIZE: f32 = 50.0;

fn char_coords(width: f32, row: f32, col: f32) -> (f32, f32) {
    (
        width / 2.0 + col * BUTTON_SIZE,
        BUTTON_SIZE + row * BUTTON_SIZE,
    )
}

fn mouse_loc(width: f32, m: &TextDimensions, click: (f32, f32)) -> Option<char> {
    let x = ((click.0 - width / 2.0 - m.width / 2.0) / BUTTON_SIZE).round() as isize;
    let y = ((click.1 - BUTTON_SIZE + m.height / 2.0) / BUTTON_SIZE).round() as isize;
    if x < 0 || y < 0 || x >= 5 {
        return None;
    }

    let index = y * 5 + x;
    match index {
        0..=25 => Some((b'A' + index as u8) as char),
        27 => Some('Z'),
        _ => None,
    }
}

fn draw_char_button(width: f32, m: &TextDimensions, letter: char, row: f32, col: f32) {
    let coords = char_coords(width, row, col);
    draw_text(format!("{letter}"), coords.0, coords.1, TEXT_SIZE, WHITE);

    let center_x = width / 2.0 + col * BUTTON_SIZE + m.width / 2.0;
    let center_y = BUTTON_SIZE + row * BUTTON_SIZE - m.height / 2.0;

    draw_circle_lines(center_x, center_y, BUTTON_SIZE / 3.0, 2.0, WHITE);
}

fn draw_hangman(_w: f32, state: &Guess) {
    let top = 50.0;

    // Scaffolding
    draw_line(50.0, top, 50.0, top + 400.0, 20.0, BROWN);
    draw_line(30.0, top, 200.0, top, 20.0, BROWN);
    draw_line(170.0, top, 170.0, top + 30.0, 20.0, BROWN);
    draw_line(170.0, top + 30.0, 170.0, top + 80.0, 3.0, BROWN);

    if state.wrong_guesses > 0 {
        // Head
        draw_circle(170.0, top + 100.0, 30.0, YELLOW);
        draw_circle(160.0, top + 90.0, 5.0, BLACK);
        draw_circle(180.0, top + 90.0, 5.0, BLACK);
        draw_line(160.0, top + 110.0, 180.0, top + 110.0, 3.0, BLACK);
    }

    if state.wrong_guesses > 1 {
        // Body
        draw_line(170.0, top + 130.0, 170.0, top + 230.0, 10.0, YELLOW);
    }

    if state.wrong_guesses > 2 {
        // Arm
        draw_line(170.0, top + 170.0, 140.0, top + 140.0, 7.0, YELLOW);
    }

    if state.wrong_guesses > 3 {
        // Other arm
        draw_line(170.0, top + 170.0, 200.0, top + 140.0, 7.0, YELLOW);
    }

    if state.wrong_guesses > 4 {
        // Leg
        draw_line(170.0, top + 220.0, 140.0, top + 310.0, 7.0, YELLOW);
    }

    if state.wrong_guesses > 5 {
        // Other leg
        draw_line(170.0, top + 220.0, 200.0, top + 310.0, 7.0, YELLOW);
    }

    if state.wrong_guesses > 6 {
        // Hand
        draw_circle(140.0, top + 140.0, 7.0, YELLOW);
    }

    if state.wrong_guesses > 7 {
        // Other hand
        draw_circle(200.0, top + 140.0, 7.0, YELLOW);
    }

    if state.wrong_guesses > 8 {
        // Foot
        draw_ellipse(130.0, top + 310.0, 20.0, 5.0, 0.0, YELLOW);
    }

    if state.wrong_guesses > 9 {
        // Other foot
        draw_ellipse(210.0, top + 310.0, 20.0, 5.0, 0.0, YELLOW);
    }
}

fn draw_word_so_far(_w: f32, state: &Guess) {
    for (col, letter) in state.word_so_far.iter().enumerate() {
        draw_text(
            format!("{letter}"),
            70.0 + col as f32 * TEXT_SIZE * 3.0 / 4.0,
            500.0,
            TEXT_SIZE * 3.0 / 4.0,
            WHITE,
        );
    }
}

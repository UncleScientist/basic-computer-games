use std::num::TryFromIntError;

use banner::Banner;
use macroquad::prelude::*;

#[macroquad::main("Banner")]
async fn main() {
    clear_background(BLACK);

    let horizontal = loop {
        let horiz = get_text("Enter horizontal size").await;
        if let Ok(horiz) = horiz.parse::<usize>() {
            break horiz;
        }
    };

    let vertical = loop {
        let vert = get_text("Enter vertical size").await;
        if let Ok(vert) = vert.parse::<usize>() {
            break vert;
        }
    };

    let ch = get_text("Enter character (or 'ALL')").await;
    let statement = format!("{} ... ", get_text("Statement").await);

    let banner = Banner::new();
    let character = if ch == "ALL" { None } else { Some(ch) };

    let iter = banner.banner(statement, character, horizontal, vertical);
    let mut screen = Screen::new();
    for column in iter {
        screen.add_column(column);
    }

    let mut frame_count = 0;
    let mut offset = 0;
    loop {
        frame_count += 1;
        if frame_count % 2 == 0 {
            offset += 1;
        }
        screen.draw(offset);
        next_frame().await;
    }
}

async fn get_text<S: AsRef<str>>(prompt: S) -> String {
    let mut cursor_count = 0u64;
    let mut input_buffer = String::new();

    loop {
        next_frame().await;
        cursor_count += 1;

        let text = if cursor_count % 60 < 30 {
            format!("{}? {input_buffer}_", prompt.as_ref())
        } else {
            format!("{}? {input_buffer}", prompt.as_ref())
        };
        draw_text(&text, 10., 20., 30., WHITE);

        if let Some(key) = get_last_key_pressed() {
            let repr_u8: Result<u8, TryFromIntError> = (key as u16).try_into();
            let ascii_key = if let Ok(ru8) = repr_u8 {
                Some(ru8 as char)
            } else {
                None
            };

            match key {
                KeyCode::Backspace => {
                    input_buffer.pop();
                }
                KeyCode::Enter => {
                    return input_buffer;
                }
                _ => {
                    if let Some(ascii_key) = ascii_key {
                        match ascii_key {
                            '0'..='9' | 'A'..='Z' | ' ' | '?' | '*' | '=' | '!' | '.' => {
                                input_buffer.push(ascii_key);
                            }
                            _ => {}
                        }
                    }
                }
            }
            cursor_count = 0;
        }
    }
}

struct Screen {
    display: Vec<String>,
    char_width: f32,
}

impl Screen {
    fn new() -> Self {
        let TextDimensions { width, .. } = measure_text("#", None, 30, 1.0);
        Self {
            display: Vec::new(),
            char_width: width,
        }
    }

    fn add_column<S: AsRef<str>>(&mut self, text: S) {
        let text = text.as_ref().chars().collect::<Vec<_>>();
        if text.is_empty() {
            // just adding a blank column
            for text in self.display.iter_mut() {
                text.push(' ');
            }
            return;
        }
        if text.len() >= self.display.len() {
            self.display
                .extend(vec![String::new(); text.len() - self.display.len() + 1]);
        }
        let mut index = self.display.len();
        let mut text_index = 0;
        while index > 0 && text_index < text.len() {
            index -= 1;
            self.display[index].push(text[text_index]);
            text_index += 1;
        }
        while index > 0 {
            index -= 1;
            self.display[index].push(' ');
        }
    }

    fn draw(&self, mut offset: usize) {
        let w = (screen_width() / self.char_width) as usize;
        let sheight = screen_height();
        offset %= self.display[0].len();
        for (row, text) in self.display.iter().enumerate() {
            let ypos = 20. + 20. * row as f32;

            if ypos > sheight {
                break;
            }

            let s = format!("{}{}", &text[offset..], &text[..offset]);
            draw_text(&s[..w], 10., 20. + 20. * row as f32, 30., WHITE);
        }
    }
}

use std::io::{BufRead, Write};

use banner::Banner;

fn main() {
    // Line 10
    let horiz = loop {
        let ret = prompt_for_input("HORIZONTAL");
        if let Ok(val) = ret.parse::<usize>() {
            break val;
        }
    };

    // Line 20
    let vert = loop {
        let ret = prompt_for_input("VERTICAL");
        if let Ok(val) = ret.parse::<usize>() {
            break val;
        }
    };

    // Line 22
    let centered = prompt_for_input("CENTERED").as_str() > "P";

    // Line 23
    let (elem, elem_len) = {
        let text = prompt_for_input("CHARACTER (TYPE 'ALL' IF YOU WANT CHARACTER BEING PRINTED)");
        if text == "ALL" {
            (None, 1)
        } else {
            let len = text.len();
            (Some(text), len)
        }
    };

    // Lines 29-30
    let message = prompt_for_input("STATEMENT");

    // Note: Ignoring line 35 as it has no effect

    let centering = if centered {
        (63.0 - 4.5 * vert as f32) / (elem_len as f32 + 1.0)
    } else {
        0.0
    } as usize;

    let engine = Banner::new();
    let iter = engine.banner(message, elem, horiz, vert);
    for line in iter {
        println!("{:<centering$}{line}", "");
    }
}

fn prompt_for_input<S: AsRef<str>>(prompt: S) -> String {
    print!("{}? ", prompt.as_ref());
    let mut lock = std::io::stdout().lock();
    let _ = lock.flush();

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut buffer);
    buffer.trim().to_string()
}

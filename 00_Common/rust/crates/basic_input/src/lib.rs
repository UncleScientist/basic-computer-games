pub mod input;
mod parse;

pub use input::*;

/*
 * TODO:

use std::io::{BufRead, Write};

pub fn input<F: FromStr, P: AsRef<str>>(prompt: P) -> F {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut stdout = std::io::stdout().lock();
        let _ = stdout.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);

        if let Ok(result) = buffer.trim().to_string().parse::<F>() {
            return result;
        }
        println!("?Re-enter");
    }
}
*/

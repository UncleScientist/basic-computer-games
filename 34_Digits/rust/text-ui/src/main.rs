use std::{
    io::{BufRead, Write},
    num::ParseFloatError,
};

use digits::Guesser;

fn main() {
    println!("{:<33}DIGITS", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n", "");
    println!("THIS IS A GAME OF GUESSING.");
    print!("FOR INSTRUCTIONS, TYPE '1', ELSE TYPE '0'?");
    loop {
        let response = prompt_for_input("");
        if let Ok(val) = response.parse::<usize>() {
            if val != 0 {
                println!("PLEASE TAKE A PIECE OF PAPER AND WRITE DOWN");
                println!("THE DIGITS '0', '1', OR '2' THIRTY TIMES AT RANDOM.");
                println!("ARRANGE THEM IN THREE LINES OF TEN DIGITS EACH.");
                println!("I WILL ASK FOR THEM TEN AT A TIME.");
                println!("I WILL ALWAYS GUESS THEM FIRST AND THEN LOOK AT YOUR");
                println!("NEXT NUMBER TO SEE IF I WAS RIGHT. BY PURE LUCK,");
                println!("I OUGHT TO BE RIGHT TEN TIMES. BUT I HOPE TO DO BETTER");
                println!("THAN THAT *****\n\n");
            }
            break;
        } else {
            println!("?REENTER");
        }
    }

    let mut guesser = Guesser::new();

    for _round in 1..=3 {
        loop {
            let values = loop {
                let entry = prompt_for_input("TEN NUMBERS, PLEASE");
                let Ok(values) = split_into_numbers(entry) else {
                    continue;
                };

                if values.len() == 10 {
                    break values;
                }
            };

            println!(
                "\n{:<10} {:<10} {:<10} NO. RIGHT\n",
                "MY GUESS", "YOUR NO.", "RESULT"
            );
            let mut slice: [f32; 10] = [0.0; 10];
            slice.clone_from_slice(&values[0..10]);
            if let Ok(result) = guesser.guess_sequence(slice) {
                for r in result {
                    println!(
                        "{:<10} {:<10} {:<10} {}",
                        r.game_guess,
                        r.user_num,
                        if r.game_guess == r.user_num {
                            "RIGHT"
                        } else {
                            "WRONG"
                        },
                        r.right_so_far
                    );
                }
                break;
            } else {
                println!("ONLY USE THE DIGITS '0', '1', OR '2'");
                println!("LET'S TRY AGAIN");
            }
        }
    }

    match guesser.game_result() {
        digits::GameResult::ComputerWins => {
            println!("I GUESSED MORE THAN 1/3 OF YOUR NUMBERS.");
            println!("I WIN.");
        }
        digits::GameResult::PlayerWins => {
            println!("I GUESSED LESS THAN 1/3 OF YOUR NUMBERS.");
            println!("YOU BEAT ME.  CONGRATULATIONS *****\n");
        }
        digits::GameResult::Tie => {
            println!("I GUESSED EXACTLY 1/3 OF YOUR NUMBERS.");
            println!("IT'S A TIE GAME.");
        }
    }

    println!("\nTHANKS FOR THE GAME.");
}

fn split_into_numbers<S: AsRef<str>>(text: S) -> Result<Vec<f32>, ParseFloatError> {
    let mut result = Vec::new();
    for item in text.as_ref().split(',') {
        let num = item.parse::<f32>()?;
        result.push(num);
    }
    Ok(result)
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

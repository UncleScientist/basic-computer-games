use std::io::{BufRead, Write};

use hangman::*;

fn main() {
    println!("{:<32}HANGMAN", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("\n\n");

    let mut game = Hangman::new();
    while game.words_left() {
        guess_a_word(&mut game);
        let again = prompt_for_string("Want another word");
        if again != "YES" {
            break;
        }
    }

    println!("\nIt's been fun! Bye for now.");
}

fn guess_a_word(hangman: &mut Hangman) {
    let mut image = [[' '; 12]; 12];

    for row in 0..12 {
        image[row][0] = 'X';
    }
    for col in 0..7 {
        image[0][col] = 'X';
    }
    image[1][6] = 'X';

    loop {
        let state = hangman.current_state();
        let guessed = state
            .letters_guessed
            .iter()
            .map(|ch| String::from(*ch))
            .collect::<Vec<_>>();
        println!("Here are the letters you used: {}", guessed.join(","));

        let word = state.word_so_far.iter().collect::<String>();
        println!("\n\n{word}\n");
        let guess = loop {
            let Some(guess) = prompt_for_string("What is your guess").chars().next() else {
                continue;
            };
            break guess;
        };

        let guess_result = hangman.guess_letter(guess);
        let state = hangman.current_state();

        match guess_result {
            GuessResult::AlreadyGuessed => {
                println!("You guessed that letter before!");
                continue;
            }
            GuessResult::FoundLetter(guess_count) => {
                let word_guess = prompt_for_string("What is your guess for the word");
                if hangman.guess_word(&word_guess) {
                    println!("Right!! It took you {guess_count} guesses");
                    break;
                } else {
                    println!("Wrong. Try another letter");
                }
                continue;
            }
            GuessResult::NotPresent => {
                println!("Sorry, that letter isn't in the word.");
                match state.wrong_guesses {
                    1 => {
                        println!("First, we draw a head");
                        image[2][5] = '-';
                        image[2][6] = '-';
                        image[2][7] = '-';
                        image[3][4] = '(';
                        image[3][5] = '.';
                        image[3][7] = '.';
                        image[3][8] = ')';
                        image[4][5] = '-';
                        image[4][6] = '-';
                        image[4][7] = '-';
                    }
                    2 => {
                        println!("Now we draw a body.");
                        for row in 5..9 {
                            image[row][6] = 'X';
                        }
                    }
                    3 => {
                        println!("Next we draw an arm.");
                        for idx in 3..7 {
                            image[idx][idx - 1] = '\\';
                        }
                    }
                    4 => {
                        println!("This time it's the other arm.");
                        image[3][10] = '/';
                        image[4][9] = '/';
                        image[5][8] = '/';
                        image[6][7] = '/';
                    }
                    5 => {
                        println!("Now, let's draw the right leg.");
                        image[9][5] = '/';
                        image[10][4] = '/';
                    }
                    6 => {
                        println!("This time we draw the left leg.");
                        image[9][7] = '\\';
                        image[10][8] = '\\';
                    }

                    7 => {
                        println!("Now we put up a hand.");
                        image[2][10] = '\\';
                    }
                    8 => {
                        println!("Next the other hand");
                        image[2][2] = '/';
                    }
                    9 => {
                        println!("Now we draw one foot");
                        image[11][9] = '\\';
                        image[11][10] = '-';
                    }
                    10 => {
                        println!("Here's the other foot -- you're hung!!");
                        image[11][2] = '-';
                        image[11][3] = '/';
                    }
                    _ => {
                        panic!("Shouldn't get here");
                    }
                }

                for row in 0..12 {
                    for col in 0..12 {
                        print!("{}", image[row][col]);
                    }
                    println!();
                }
                println!();

                if state.wrong_guesses >= 10 {
                    let word = state.solution_word.iter().collect::<String>();
                    hangman.move_along();
                    println!("Sorry, you lose. The word was: {word}",);
                    println!("You missed that one.");
                    return;
                }
            }

            GuessResult::FoundWord => {
                println!("You found the word!");
                break;
            }
        }
    }
}

fn prompt_for_string<S: AsRef<str>>(prompt: S) -> String {
    print!("{}? ", prompt.as_ref());
    let mut lock = std::io::stdout().lock();
    let _ = lock.flush();

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut buffer);
    buffer.trim().to_string()
}

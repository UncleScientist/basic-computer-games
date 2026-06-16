use std::io::{BufRead, Write};

use checkers::{BoardState, Checkers, NextMove};

fn main() {
    println!("{:<32}CHECKERS", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("This is the game of Checkers. The computer is X,");
    println!("and you are O. The computer will move first.");
    println!("Squares are referred to by a coordinate system.");
    println!("(0,0) is the lower left corner");
    println!("(0,7) is the upper left corner");
    println!("(7,0) is the lower right corner");
    println!("(7,7) is the upper right corner");
    println!("The computer will type '+TO' when you have another");
    println!("jump. Type two negative numbers if you cannot jump.\n\n\n");

    let mut checkers = Checkers::new();
    let state = loop {
        let computer_move = checkers.computer_move();
        println!("{computer_move}");
        print_board(&checkers);
        let state = checkers.board_state();
        if state != BoardState::GameContinues {
            break state;
        }
        'reenter: loop {
            let from = loop {
                let from = prompt_for_string("From");
                let Some((from_col, from_row)) = from.split_once(",") else {
                    continue;
                };
                let Ok(from_col) = from_col.trim().parse::<usize>() else {
                    continue;
                };
                let Ok(from_row) = from_row.trim().parse::<usize>() else {
                    continue;
                };
                if from_col > 7 || from_row > 7 {
                    continue;
                }
                break (from_col, from_row);
            };

            let mut plus = "";

            'next_jump: loop {
                let to = loop {
                    let to = prompt_for_string(format!("{}To", plus));
                    if !plus.is_empty() && to.starts_with("-") {
                        break 'reenter;
                    }

                    let Some((to_col, to_row)) = to.split_once(",") else {
                        continue;
                    };
                    let Ok(to_col) = to_col.trim().parse::<usize>() else {
                        continue;
                    };
                    let Ok(to_row) = to_row.trim().parse::<usize>() else {
                        continue;
                    };
                    if to_col > 7 || to_row > 7 {
                        continue;
                    }
                    break (to_col, to_row);
                };
                plus = "+";

                match checkers.player_move(from.1, from.0, to.1, to.0) {
                    Ok(NextMove::ComputerGoes) => break 'reenter,
                    Ok(NextMove::JumpAgain) => continue 'next_jump,
                    Err(e) => {
                        println!("{e:?}");
                        continue 'reenter;
                    }
                }
            }
        }
        let state = checkers.board_state();
        if state != BoardState::GameContinues {
            break state;
        }
    };

    println!(
        "{}",
        match state {
            BoardState::RedWins => "I WIN",
            BoardState::BlackWins => "YOU WIN",
            BoardState::GameContinues => unreachable!(),
        }
    );
}

fn print_board(checkers: &Checkers) {
    let board = checkers.get_board();

    #[allow(clippy::needless_range_loop)]
    for row in 0..8 {
        for col in 0..8 {
            let b = format!("{}", board[row][col]);
            print!("{b:<5}");
        }
        println!("\n");
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

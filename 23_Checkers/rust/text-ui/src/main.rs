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
                let Some((from_x, from_y)) = from.split_once(",") else {
                    continue;
                };
                let Ok(from_x) = from_x.trim().parse::<usize>() else {
                    continue;
                };
                let Ok(from_y) = from_y.trim().parse::<usize>() else {
                    continue;
                };
                if from_x > 7 || from_y > 7 {
                    continue;
                }
                break (from_x, from_y);
            };

            let mut plus = "";

            'next_jump: loop {
                let to = loop {
                    let to = prompt_for_string(format!("{}To", plus));
                    let Some((to_x, to_y)) = to.split_once(",") else {
                        continue;
                    };
                    let Ok(to_x) = to_x.trim().parse::<usize>() else {
                        continue;
                    };
                    let Ok(to_y) = to_y.trim().parse::<usize>() else {
                        continue;
                    };
                    if to_x > 7 || to_y > 7 {
                        continue;
                    }
                    break (to_x, to_y);
                };
                plus = "+";

                match checkers.player_move(from.0, from.1, to.0, to.1) {
                    Ok(NextMove::ComputerGoes) => break 'reenter,
                    Ok(NextMove::JumpAgain) => continue 'next_jump,
                    Err(_) => continue 'reenter,
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

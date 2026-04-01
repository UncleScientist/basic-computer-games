use std::io::{BufRead, Write};

use one_check::Board;

fn main() {
    println!("{:>30}One Check", "");
    println!("{:>15}Creative Computing  Morristown, New Jersey\n\n\n", "");

    println!("Solitare Checker Puzzle by David Ahl\n");

    println!("48 checkers are placed on the 2 outside spaces of a");
    println!("standard 64-square checkerboard. The object is to");
    println!("remove as many checkers as possible by diagonal jumps");
    println!("(as in standard checkers). Use the numbered board to");
    println!("indicate the square you wish to jump from and to. On");
    println!("the board printed out on each turn '1' indicates a");
    println!("checker and '0' and empty square. When you have no");
    println!("possible jumps remaining, input a '0' in response to");
    println!("question 'jump from?'\n");

    println!("Here is the numerical board\n");

    for j in (1..=57).step_by(8) {
        println!(
            " {:3} {:3} {:3} {:3} {:3} {:3} {:3} {:3}",
            j,
            j + 1,
            j + 2,
            j + 3,
            j + 4,
            j + 5,
            j + 6,
            j + 7
        );
    }

    println!("\nAnd here is the opening position of the checkers.\n");

    let mut board = Board::new();

    loop {
        println!("{board}");

        let (from, to) = loop {
            // Lines 100-112
            let from = get_board_position("Jump from");
            if from == 0 {
                break (0, 0);
            }

            let to = get_board_position("To");
            if board.legal_move(from, to) {
                break (from, to);
            }

            // Line 230-240
            println!("Illegal move. Try again...");
        };

        println!();

        // Line 105
        if from == 0 {
            break;
        }

        // Lines 250-290
        board.make_move(from, to);
    }
}

fn get_board_position<S: AsRef<str>>(prompt: S) -> usize {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(result) = buffer.trim().parse::<usize>() {
            return result;
        } else {
            println!("?REENTER");
        }
    }
}

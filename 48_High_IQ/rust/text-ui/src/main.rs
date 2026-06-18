use std::io::{BufRead, Write};

use high_iq::*;

fn main() {
    println!("{:<33}H-I-Q", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("\n\n\nHERE IS THE BOARD:\n");
    println!("          !    !    !");
    println!("         13   14   15\n");
    println!("          !    !    !");
    println!("         22   23   24\n");
    println!("!    !    !    !    !    !    !");
    println!("29   30   31   32   33   34   35\n");
    println!("!    !    !    !    !    !    !");
    println!("38   39   40   41   42   43   44\n");
    println!("!    !    !    !    !    !    !");
    println!("47   48   49   50   51   52   53\n");
    println!("          !    !    !");
    println!("         58   59   60\n");
    println!("          !    !    !");
    println!("         67   68   69\n");

    println!("To save typing time, a compressed version of the game board");
    println!("will be used during play. Refer to the above one for peg");
    println!("numbers. OK, let's begin.");

    loop {
        play_game();
        let play_again = prompt_for_string("Play again (yes or no)");
        if play_again == "NO" {
            break;
        }
    }

    println!("\nSo long for now.\n");
}

fn play_game() {
    let mut game = HighIq::new();

    loop {
        println!("{game}");

        let from = loop {
            let from = prompt_for_input("Move which piece");
            if game.has_peg_at(from) {
                break from;
            }
            println!("Illegal move, try again...");
        };
        let to = prompt_for_input("To where");

        match game.make_move(from, to) {
            Ok(_) => {}
            Err(_) => println!("Illegal move, try again..."),
        }

        match game.check_board() {
            GameState::MovesRemaining => continue,
            GameState::GameOver(pegs) => {
                println!("{game}");
                println!("The game is over.");
                println!("You had {pegs} pieces remaining.");
                if pegs == 1 {
                    println!("Bravo! You made a perfect score!");
                    println!("Save this paper as a record of your accomplishment");
                }
                return;
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

fn prompt_for_input<S: AsRef<str>>(prompt: S) -> usize {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(num) = buffer.trim().to_string().parse::<usize>() {
            return num;
        }
        println!("?REENTER");
    }
}

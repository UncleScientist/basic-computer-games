use std::io::{BufRead, Write};

use bowling::*;

fn main() {
    println!("{:<34}BOWL", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n", "");
    println!("WELCOME TO THE ALLEY");
    println!("BRING YOUR FRIENDS");
    println!("OKAY LET'S FIRST GET ACQUAINTED\n");

    println!("THE INSTRUCTIONS (Y/N)");
    let response = prompt_for_input("");
    if response != "N" {
        println!("THE GAME OF BOWLING TAKES MIND AND SKILL.DURING THE GAME");
        println!("THE COMPUTER WILL KEEP SCORE.YOU MAY COMPETE WITH");
        println!("OTHER PLAYERS[UP TO FOUR].YOU WILL BE PLAYING TEN FRAMES");
        println!("ON THE PIN DIAGRAM 'O' MEANS THE PIN IS DOWN...'+' MEANS THE");
        println!("PIN IS STANDING.AFTER THE GAME THE COMPUTER WILL SHOW YOUR");
        println!("SCORES .");
    }

    let response = prompt_for_input("FIRST OF ALL...HOW MANY ARE PLAYING");
    let player_count = loop {
        let Ok(player_count) = response.parse::<usize>() else {
            println!("??REENTER");
            continue;
        };
        break player_count;
    };
    println!("VERY GOOD...");

    let mut game = Bowling::new(player_count);
    loop {
        println!("TYPE ROLL TO GET THE BALL GOING.");
        prompt_for_input("");

        let (player, frame, ball) = game.current_turn_info();
        println!("PLAYER: {player} FRAME: {frame} BALL: {ball}");
        let response = game.roll();
        if response.contains(&Response::GameOver) {
            break;
        }

        for r in response
            .iter()
            .filter(|r| **r != Response::NeedsAnotherBall)
        {
            println!("{r}");
        }

        let pins = game.get_pins();
        let mut k = 0;
        for i in 0..=3 {
            print!("\n{:<i$}", "");
            for _ in 1..=(4 - i) {
                k += 1;
                if pins[k] == 1 {
                    print!("O ");
                } else {
                    print!("+ ");
                }
            }
        }
        println!();
    }

    let result = game.get_scores();
    println!("FRAMES");
    for i in 1..=10 {
        print!(" {i}");
    }
    println!();
    for player in 1..=2 {
        #[allow(clippy::needless_range_loop)]
        for i in 1..=3 {
            for j in 1..=10 {
                print!(" {}", result[j * player][i]);
            }
            println!();
        }
        println!();
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

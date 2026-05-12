use std::{
    cmp::Ordering,
    io::{BufRead, Write},
};

use slots::{PullResult, Slots, WinState};

fn main() {
    println!("{:<30}SLOTS", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("\n\n\nPRODUCED BY FRED MIRABELLE AND BOB HARPER ON JAN 29, 1973");
    println!("IT SIMULATES THE SLOT MACHINE.\n");
    println!("You are in the H&M Casino, in front of one of our");
    println!("one-arm bandits. Bet from $1 to $100.");
    println!("To pull the arm, punch the return key after making your bet.");

    let mut game = Slots::new();

    loop {
        let bet = prompt_for_bet("\nYour bet");
        if bet < 1.0 {
            println!("Minimum bet is $1");
            continue;
        }
        if bet > 100.0 {
            println!("House limits are $100");
            continue;
        }
        let PullResult {
            dial1,
            dial2,
            dial3,
            result,
        } = game.pull_arm(bet);
        println!("\n{dial1} {dial2} {dial3}");

        match result {
            WinState::None => println!("\nYOU LOST."),
            WinState::Double => println!("\nDOUBLE!!"),
            WinState::DoubleBar => println!("\n*DOUBLE BAR*"),
            WinState::TopDollar => println!("\n**TOP DOLLAR**"),
            WinState::Jackpot => println!("\n***JACKPOT***"),
        }

        if result != WinState::None {
            println!("YOU WON!");
        }

        println!("YOUR STANDINGS ARE ${}", game.get_payout());

        let again = prompt_for_text("Again");
        if again != "Y" && again != "y" {
            break;
        }
    }

    match game
        .get_payout()
        .partial_cmp(&0.0)
        .unwrap_or(Ordering::Equal)
    {
        Ordering::Less => println!("PAY UP!  PLEASE LEAVE YOUR MONEY ON THE TERMIANL."),
        Ordering::Equal => println!("HEY, YOU BROKE EVEN."),
        Ordering::Greater => println!("COLLECT YOUR WINNINGS FROM THE H&M CASHIER."),
    }
}

fn prompt_for_text<S: AsRef<str>>(prompt: S) -> String {
    print!("{}? ", prompt.as_ref());
    let mut lock = std::io::stdout().lock();
    let _ = lock.flush();

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut buffer);
    buffer.trim().to_string()
}

fn prompt_for_bet<S: AsRef<str>>(prompt: S) -> f32 {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(num) = buffer.trim().to_string().parse::<f32>() {
            return num;
        }
        println!("?REENTER");
    }
}

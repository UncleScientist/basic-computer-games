use std::io::{BufRead, Write};

use boxing::{Boxing, MatchResult, Outcome, Punch};

fn main() {
    println!("{:<33}BOXING", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n", "");
    let opp_name = prompt_for_string("What is your opponent's name");
    let plr_name = prompt_for_string("Input your man's name");

    println!("Different punches are: (1) Full Swing; (2) Hook; (3) Uppercut; (4) Jab.");
    let best = prompt_for_punch("What is your mans best");
    let vuln = prompt_for_punch("What is his vulerability");

    let mut boxing = Boxing::new(plr_name, opp_name, best, vuln);
    println!(
        "{}'s advantage is {} and vulerability is secret.",
        boxing.opponent_name(),
        boxing.opponent_advantage()
    );
    'boxing_match: for round in 0..3 {
        println!("Round {} begins", round + 1);
        boxing.start_round();
        for _ in 0..7 {
            let turn = boxing.who_swings();
            match turn {
                boxing::Turn::Player => {
                    print!("{}'s punch", boxing.player_name());
                    let punch = prompt_for_punch("");
                    match punch {
                        Punch::FullSwing => println!("{} swings and", boxing.player_name()),
                        Punch::Hook => println!("{} gives the hook...", boxing.player_name()),
                        Punch::Uppercut => println!("{} tries an uppercut", boxing.player_name()),
                        Punch::Jab => println!(
                            "{} jabs at {}'s head",
                            boxing.player_name(),
                            boxing.opponent_name()
                        ),
                    }
                    match boxing.player_swings(punch) {
                        Outcome::Blocked => println!("it's blocked."),
                        Outcome::Connects => println!("he connects!"),
                        Outcome::Misses => println!("he misses"),
                        Outcome::Knockout => {
                            println!(
                                "{} is knocked cold and {} is the winner and champ!",
                                boxing.opponent_name(),
                                boxing.player_name()
                            );
                            break 'boxing_match;
                        }
                        _ => unreachable!(),
                    }
                }
                boxing::Turn::Opponent => {
                    let (punch, outcome) = boxing.opponent_swings();
                    match punch {
                        Punch::FullSwing => {
                            println!("{} takes a full swing and", boxing.opponent_name())
                        }
                        Punch::Hook => println!(
                            "{} gets {} in the jaw (ouch!)",
                            boxing.opponent_name(),
                            boxing.player_name()
                        ),
                        Punch::Uppercut => println!(
                            "{} is attacked by an uppercut (oh, oh)...",
                            boxing.player_name()
                        ),
                        Punch::Jab => println!("{} jabs and", boxing.opponent_name()),
                    }
                    match outcome {
                        Outcome::Blocked => println!("it's blocked!"),
                        Outcome::BloodSpills => println!("blood spills !!!"),
                        Outcome::InTheFace => println!("Pow!!! He hits him right in the face!"),
                        Outcome::AndAgain => println!("...and again!"),
                        Outcome::Connects => println!("Connects..."),
                        Outcome::BlocksAndHooks => {
                            println!("blocks and hits {} with a hook.", boxing.opponent_name())
                        }
                        Outcome::Knockout => {
                            println!(
                                "{} is knocked cold and {} is the winner and champ!",
                                boxing.player_name(),
                                boxing.opponent_name()
                            );
                            break 'boxing_match;
                        }
                        Outcome::Misses => todo!(),
                    }
                }
            }
        }
        match boxing.end_round() {
            boxing::RoundWinner::Player => {
                println!("\n{} wins round {}", boxing.player_name(), round + 1)
            }
            boxing::RoundWinner::Opponent => {
                println!("\n{} wins round {}", boxing.opponent_name(), round + 1)
            }
        }
    }

    match boxing.match_result() {
        MatchResult::Knockout => {}
        MatchResult::Player => println!("{} amazingly wins!!", boxing.player_name()),
        MatchResult::Opponent => println!(
            "{} wins (nice going, {}).",
            boxing.opponent_name(),
            boxing.opponent_name()
        ),
    }

    println!("and now goodbye from the Olympic arena.");
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

fn prompt_for_punch<S: AsRef<str>>(prompt: S) -> Punch {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(punch) = buffer.trim().to_string().parse::<Punch>() {
            return punch;
        }
        println!("?REENTER");
    }
}

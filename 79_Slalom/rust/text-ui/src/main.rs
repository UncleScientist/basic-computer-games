use std::io::{BufRead, Write};

use slalom::{CommandOutcome, ErrorOutcome, Failure, SkillLevel, Slalom, Success};

fn main() {
    println!("{:<33}SLALOM", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("\n\n\n");
    let gate_count = loop {
        let count = prompt_for_number("How many gates does this course have (1 to 25)");
        if count < 1.0 {
            println!("Try again,");
        } else if count > 25.0 {
            println!("25 is the limit");
        } else {
            break count as usize;
        }
    };

    println!("\nType \"INS\" for instructions");
    println!("Type \"MAX\" for approximate maxiumum speeds");
    println!("Type \"RUN\" for the beginning of the race");

    loop {
        let input = prompt_for_string("COMMAND--");

        match input.as_str() {
            "INS" => {
                println!("\n*** SLALOM: This is the 1976 Winter Olympic Giant Slalom. You are");
                println!("    the American team's only hope of a gold medal.");
                println!();
                println!("     0 -- Type this if you want to see how long you've taken.");
                println!("     1 -- Type this if you want to speed up a lot.");
                println!("     2 -- Type this if you want to speed up a little.");
                println!("     3 -- Type this if you want to speed up a teensy.");
                println!("     4 -- Type this if you want to keep going the same speed.");
                println!("     5 -- Type this if you want to check a teensy.");
                println!("     6 -- Type this if you want to check a little.");
                println!("     7 -- Type this if you want to check a lot.");
                println!("     8 -- Type this if you want to cheat and try to skip a gate.");
                println!("\n The place to use these options is when the computer asks:");
                println!("\nOPTION?");
                println!("\n                Good luck!");
            }
            "MAX" => {
                println!("Gate Max");
                println!(" #  M.P.H.");
                for (gate, speed) in Slalom::gate_speeds(gate_count).iter().enumerate() {
                    println!("{:2}  {speed}", gate + 1);
                }
            }
            "RUN" => {
                break;
            }
            _ => println!("\"{input}\" is an illegal command--retry"),
        }
    }

    let skill_level: SkillLevel = loop {
        let level = prompt_for_number("Rate yourself as a skiier, (1=worst, 3=best)");
        if !(1.0..=3.0).contains(&level) {
            println!("The bounds are 1-3");
        } else if let Ok(level) = (level as usize).try_into() {
            break level;
        } else {
            println!("The bounds are 1-3");
        }
    };

    let mut slalom = Slalom::new(gate_count, skill_level);

    'game_over: loop {
        println!("The starter counts down...5...4...3...2...1...GO!");
        println!("\nYou're off!");

        loop {
            println!("\nHere comes gate #{}:", slalom.gate());
            println!("{} M.P.H.", slalom.speed());
            let option = loop {
                let option = prompt_for_number("Option") as isize;
                if option == 0 {
                    println!("You've taken {} seconds.", slalom.time());
                    continue;
                }
                if !(1..=8).contains(&option) {
                    println!("What?");
                    continue;
                }
                break option;
            };

            if let Ok(adj) = (option as usize).try_into() {
                match slalom.adjust_speed(&adj) {
                    Ok(success) => {
                        println!("{} M.P.H.", slalom.speed());
                        match success {
                            CommandOutcome::Success(success) => match success {
                                Success::NothingSpecial => {}
                                Success::CloseOne => {
                                    println!("Close one!");
                                }
                                Success::YouMadeIt => {
                                    println!("***CHEAT\nYou made it!");
                                }
                                Success::MadeItOverMax => {
                                    println!("You went over the maximum speed and made it!");
                                }
                            },
                            CommandOutcome::Failure(failure) => {
                                match failure {
                                    Failure::OfficialCaughtYou => {
                                        println!("***CHEAT");
                                        println!("An official caught you!");
                                    }
                                    Failure::WipedOut => {
                                        println!("You went over the maximum speed and wiped out!");
                                    }
                                    Failure::SnaggedFlag => {
                                        println!(
                                            "You went over the maximum speed and snagged a flag!"
                                        );
                                    }
                                }
                                println!("You took {} seconds", slalom.time());
                            }
                        }
                    }
                    Err(failure) => match failure {
                        ErrorOutcome::RaceIsOver => {
                            panic!("should not have continued race in this state {slalom:?}");
                        }
                        ErrorOutcome::TooSlow => {
                            println!("Let's be realistic, OK?  Let's go back and try again...");
                        }
                    },
                }
            } else {
                println!("What?");
            }

            if slalom.race_over() {
                break;
            }
        }

        println!("\nYou took {} seconds.", slalom.time());
        match slalom.get_result() {
            slalom::Medal::None => {}
            slalom::Medal::Bronze => {
                println!("You won a bronze medal");
            }
            slalom::Medal::Silver => {
                println!("You won a silver medal");
            }
            slalom::Medal::Gold => {
                println!("You won a gold medal");
            }
        }

        loop {
            let again = prompt_for_string("\nDo you want to race again");
            match again.as_str() {
                "YES" => {
                    slalom.start_new_race();
                    break;
                }
                "NO" => {
                    println!("Thanks for the race");
                    let (g, s, b) = slalom.medal_counts();
                    if g > 0 {
                        println!("Gold medals: {g}");
                    }
                    if s > 0 {
                        println!("Silver medals: {s}");
                    }
                    if b > 0 {
                        println!("Bronze medals: {b}");
                    }
                    break 'game_over;
                }
                _ => println!("Please type 'YES' or 'NO'"),
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

fn prompt_for_number<S: AsRef<str>>(prompt: S) -> f64 {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(num) = buffer.trim().to_string().parse::<f64>() {
            return num;
        }
        println!("?REENTER");
    }
}

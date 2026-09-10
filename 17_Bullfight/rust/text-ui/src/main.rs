use std::io::{BufRead, Write};

use bullfight::{
    Award, Bullfight, CapeMove, Crowd, KillMove, KillResult, Outcome, PrepResult, RunOrRemain,
};

fn main() {
    println!("{:<34}BULL", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("\n\n");
    let inst = prompt_for_string("Do you want instructions");
    if !inst.eq_ignore_ascii_case("no") {
        println!("HELLO, ALL YOU BLOODLOVERS AND AFICIONADOS.");
        println!("HERE IS YOUR BIG CHANCE TO KILL A BULL.");
        println!();
        println!("ON EACH PASS OF THE BULL, YOU MAY TRY");
        println!("0 - VERONICA (DANGEROUS INSIDE MOVE OF THE CAPE)");
        println!("1 - LESS DANGEROUS OUTSIDE MOVE OF THE CAPE");
        println!("2 - ORDINARY SWIRL OF THE CAPE.");
        println!();
        println!("INSTEAD OF THE ABOVE, YOU MAY TRY TO KILL THE BULL");
        println!("ON ANY TURN: 4 (OVER THE HORNS), 5 (IN THE CHEST).");
        println!("BUT IF I WERE YOU,");
        println!("I WOULDN'T TRY IT BEFORE THE SEVENTH PASS.");
        println!();
        println!("THE CROWD WILL DETERMINE WHAT AWARD YOU DESERVE");
        println!("(POSTHUMOUSLY IF NECESSARY).");
        println!("THE BRAVER YOU ARE, THE BETTER THE AWARD YOU RECEIVE.");
        println!();
        println!("THE BETTER THE JOB THE PICADORES AND TOREADORES DO,");
        println!("THE BETTER YOUR CHANCES ARE.");
    }

    println!("\n");

    let mut bullfight = Bullfight::new();
    let ability = bullfight.bull_ability();
    println!("You have drawn a {ability} bull.");
    match ability {
        bullfight::Ability::Superb => println!("Good luck. You'll need it."),
        bullfight::Ability::Good | bullfight::Ability::Fair | bullfight::Ability::Poor => {}
        bullfight::Ability::Awful => println!("You're lucky."),
    }

    print_prep_result("Picadores", &bullfight.picadores);
    print_prep_result("Toreadores", &bullfight.toreadores);

    let mut player_dead = false;
    loop {
        let pass = bullfight.next_pass();
        println!("\n\nPass number {pass}");

        let kill_desire = if pass < 3 {
            println!("The bull is charging at you! You are the matador--");
            prompt_for_yes_no("Do you want to kill the bull")
        } else {
            prompt_for_yes_no("Here comes the bull. Try for a kill")
        };

        match kill_desire {
            YesOrNo::Yes => {
                let kill_prompt = prompt_for_kill_method();
                match kill_prompt {
                    KillPrompt::KillMove(kill_move) => match bullfight.try_to_kill(kill_move) {
                        KillResult::PlayerDead => {
                            println!("The bull has gored you!");
                            player_dead = true;
                            break;
                        }
                        KillResult::BullDead => break,
                        KillResult::ContinueGame => {
                            println!("The bull has gored you!");
                            continue;
                        }
                    },
                    KillPrompt::Panic => {
                        println!("You panicked. The bull gored you.");
                        match player_panic(&mut bullfight) {
                            PanicResult::PlayerDeath => {
                                player_dead = true;
                                break;
                            }
                            PanicResult::StandsAndFights => continue,
                            PanicResult::RunsAway => {
                                println!(
                                    "THE CROWD BOOS FOR TEN MINUTES.  IF YOU EVER DARE TO SHOW"
                                );
                                println!(
                                    "YOUR FACE IN A RING AGAIN, THEY SWEAR THEY WILL KILL YOU--"
                                );
                                println!("UNLESS THE BULL DOES FIRST.");
                                break;
                            }
                            PanicResult::GameOver => break,
                        }
                    }
                }
            }
            YesOrNo::No => {
                let cape_move = prompt_for_cape_twirl(if pass < 3 {
                    "What move do you make with the cape"
                } else {
                    "Cape move"
                });
                let outcome = bullfight.cape_move(cape_move);
                match outcome {
                    Outcome::Continue => continue,
                    Outcome::PlayerDead => {
                        println!("The bull has gored you!");
                        player_dead = true;
                        break;
                    }
                    Outcome::StillAlive => {
                        println!("The bull has gored you!");
                        continue;
                    }
                    Outcome::Done | Outcome::BullDead => unreachable!(),
                }
            }
        }
    }

    if player_dead {
        println!("You are dead.");
    }

    let (crowd, award) = bullfight.final_result();
    match crowd {
        Crowd::CheerWildly => println!("The crowd cheers wildly!"),
        Crowd::Cheer => println!("The crowd cheers!\n"),
        Crowd::RemainSilent => {}
    }

    println!("The crowd awards you");
    match award {
        Award::MuyHombre => println!("Ole! You are 'muy hombre'!! Ole! Ole!"),
        Award::BothEars => println!("Both ears of the bull!\nOle!"),
        Award::SingleEar => println!("One ear of the bull."),
        Award::NothingAtAll => println!("Nothing at all."),
    }
    println!("\nAdios\n\n");
}

fn player_panic(bullfight: &mut Bullfight) -> PanicResult {
    match bullfight.check_for_death() {
        Outcome::PlayerDead => PanicResult::PlayerDeath,
        Outcome::StillAlive => {
            println!("You are still alive.\n");
            let run = prompt_for_yes_no("Do you run from the ring");
            let decision = match run {
                YesOrNo::No => {
                    println!("You are brave. Stupid, but brave.");
                    RunOrRemain::Remain
                }
                YesOrNo::Yes => {
                    println!("Coward");
                    RunOrRemain::Run
                }
            };
            let outcome = bullfight.after_bull_charge(decision);
            match outcome {
                Outcome::Continue => PanicResult::StandsAndFights,
                Outcome::PlayerDead => PanicResult::PlayerDeath,
                Outcome::StillAlive => PanicResult::StandsAndFights,
                Outcome::Done => PanicResult::RunsAway,
                Outcome::BullDead => PanicResult::GameOver,
            }
        }
        _ => unreachable!(),
    }
}

enum PanicResult {
    PlayerDeath,
    RunsAway,
    StandsAndFights,
    GameOver,
}

fn print_prep_result(who: &str, prep: &PrepResult) {
    println!("\nThe {who} did a {} job.", prep.ability);
    if prep.horses_killed > 0 {
        println!(" {} of the horses of the {who} killed.", prep.horses_killed);
    }
    println!(" {} of the {who} killed.", prep.people_killed);
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

fn prompt_for_yes_no<S: AsRef<str>>(prompt: S) -> YesOrNo {
    loop {
        let answer = prompt_for_string(&prompt);
        if answer.eq_ignore_ascii_case("yes") {
            return YesOrNo::Yes;
        }
        if answer.eq_ignore_ascii_case("no") {
            return YesOrNo::No;
        }
        println!("Incorrect answer - - please type 'yes' or 'no'.");
    }
}

fn prompt_for_cape_twirl(prompt: &str) -> CapeMove {
    loop {
        let cape = prompt_for_string(prompt);
        match cape.as_str() {
            "0" => break CapeMove::Veronica,
            "1" => break CapeMove::Outside,
            "2" => break CapeMove::Swirl,
            _ => println!("Don't panic, you idiot! Put down a correct number"),
        }
    }
}

fn prompt_for_kill_method() -> KillPrompt {
    println!("\nIt is the moment of truth\n");
    let method = prompt_for_string("How do you try to kill the bull");
    if method == "4" {
        KillPrompt::KillMove(KillMove::OverTheHorns)
    } else if method == "5" {
        KillPrompt::KillMove(KillMove::InTheChest)
    } else {
        KillPrompt::Panic
    }
}

enum KillPrompt {
    KillMove(KillMove),
    Panic,
}

enum YesOrNo {
    Yes,
    No,
}

use std::io::{BufRead, Write};

use combat::{Combat, Unit};

fn main() {
    println!("{:<33}COMBAT", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    println!("\n\n\nI am at war with you");
    println!("We have 72000 soldiers apiece.");
    println!("\nDistribute your forces.");
    println!("{:<8}{:<8}{:<8}", "", "ME", "YOU");
    let mut combat = loop {
        let army = prompt_for_input(format!("{:<8}{:<8}", "ARMY", "30000"));
        let navy = prompt_for_input(format!("{:<8}{:<8}", "NAVY", "30000"));
        let air_force = prompt_for_input(format!("{:<8}{:<8}", "A. F.", "30000"));
        match Combat::new(army, navy, air_force) {
            Ok(c) => break c,
            Err(_) => continue,
        }
    };
    println!("You attack first. Type (1) for Army; (2) for Navy;");
    println!("and (3) for Air Force.");
    let choice = prompt_for_input("");
    loop {
        println!("How many men");
        let amount = prompt_for_input("");
        let attacking_unit = match choice {
            2 => Unit::Navy(amount),
            3 => Unit::AirForce(amount),
            _ => Unit::Army(amount),
        };
        match combat.first_battle(attacking_unit) {
            combat::FirstBattleOutcome::TooManyUnits => continue,
            combat::FirstBattleOutcome::PlayerLose { units } => {
                println!("You lost {} men from your army.", units.units());
                break;
            }
            combat::FirstBattleOutcome::BothLose { player, computer } => {
                println!(
                    "You lost {} men, but I lost {}",
                    player.units(),
                    computer.units()
                );
                break;
            }
            combat::FirstBattleOutcome::ComputerStoppedAttack => {
                println!("Your attack was stopped!");
                break;
            }
            combat::FirstBattleOutcome::ComputerLose { units } => {
                println!("You destroyed {} of my army.", units.units());
                break;
            }
            combat::FirstBattleOutcome::ComputerLosePatrolBoat => {
                println!("You sunk one of my patrol boats, but I wiped out two");
                println!("of your Air Force bases and 3 Army bases.");
                break;
            }
            combat::FirstBattleOutcome::AttackWipedOut => {
                println!("Your attack was wiped out.");
                break;
            }
            combat::FirstBattleOutcome::Dogfight => {
                println!("We had a dogfight. You won - and finished your mission.");
                break;
            }
            combat::FirstBattleOutcome::ComputerLoseArmyPatrol => {
                println!("You wiped out one of my Army patrols, but I destroyed");
                println!("two Navy bases and bombed three Army bases");
                break;
            }
        }
    }

    println!("{:<8}{:<8}{:<8}", "", "YOU", "ME");
    println!(
        "{:<8}{:<8}{:<8}",
        "ARMY",
        combat.player.army_units(),
        combat.computer.army_units()
    );
    println!(
        "{:<8}{:<8}{:<8}",
        "NAVY",
        combat.player.navy_units(),
        combat.computer.navy_units()
    );
    println!(
        "{:<8}{:<8}{:<8}",
        "A. F.",
        combat.player.air_force_units(),
        combat.computer.air_force_units()
    );
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

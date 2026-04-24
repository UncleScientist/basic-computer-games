use std::io::{BufRead, Write};
use target::Target;

fn main() {
    println!("{:<33}TARGET", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n", "");

    println!("YOU ARE THE WEAPONS OFFICER ON THE STARSHIP ENTERPRISE");
    println!("AND THIS IS A TEST TO SEE HOW ACCURATE A SHOT YOU");
    println!("ARE IN A THREE-DIMENSIONAL RANGE.  YOU WILL BE TOLD");
    println!("THE RADIAN OFFSET FOR THE X AND Z AXES, THE LOCATION");
    println!("OF THE TARGET IN THREE DIMENSIONAL RECTANGULAR COORDINATES,");
    println!("THE APPROXIMATE NUMBER OF DEGREES FROM THE X AND Z");
    println!("AXES, AND THE APPROXIMATE DISTANCE TO THE TARGET.");
    println!("YOU WILL THEN PROCEEED TO SHOOT AT THE TARGET UNTIL IT IS");
    println!("DESTROYED!\n\n");
    println!("GOOD LUCK!!\n\n");

    loop {
        let mut target = Target::new();
        let target_info = target.approx_location();
        println!(
            "RADIANS FROM X AXIS = {}   FROM Z AXIS = {}",
            target_info.radians_x, target_info.radians_z
        );
        println!(
            "TARGET SIGHTED: APPROXIMATE COORDINATES:  X= {},  Y= {},  Z= {}",
            target_info.loc.0, target_info.loc.1, target_info.loc.2
        );

        loop {
            let dist_est = target.estimated_distance();
            println!("     ESTIMATED DISTANCE: {dist_est}\n");
            println!("INPUT ANGLE DEVIATION FROM X, DEVIATION FROM Z, DISTANCE");
            let (degrees_x, degrees_z, distance) = loop {
                let input = prompt_for_input("");
                let split = input.split(',').collect::<Vec<_>>();
                if split.len() != 3 {
                    println!("?REENTER");
                    continue;
                }
                let Ok(degrees_x) = split[0].parse::<f32>() else {
                    println!("?REENTER");
                    continue;
                };
                let Ok(degrees_z) = split[1].parse::<f32>() else {
                    println!("?REENTER");
                    continue;
                };
                let Ok(distance) = split[2].parse::<f32>() else {
                    println!("?REENTER");
                    continue;
                };
                break (degrees_x, degrees_z, distance);
            };

            match target.fire(degrees_x, degrees_z, distance) {
                target::FireResult::SelfDestructed => {
                    println!("YOU BLEW YOURSELF UP!!");
                    break;
                }
                target::FireResult::Miss(explosion) => {
                    println!(
                        "RADIANS FROM X AXIS = {}  FROM Z AXIS = {}",
                        explosion.radians_x, explosion.radians_z
                    );
                    if explosion.delta.0 < 0.0 {
                        println!("SHOT BEHIND TARGET {} KILOMETERS", -explosion.delta.0);
                    } else {
                        println!("SHOT IN FRONT OF TARGET {} KILOMETERS", explosion.delta.0);
                    }

                    if explosion.delta.1 < 0.0 {
                        println!("SHOT TO RIGHT OF TARGET {} KILOMETERS", -explosion.delta.1);
                    } else {
                        println!("SHOT TO LEFT OF TARGET {} KILOMETERS", explosion.delta.1);
                    }
                    if explosion.delta.2 < 0.0 {
                        println!("SHOT BELOW TARGET {} KILOMETERS", -explosion.delta.2);
                    } else {
                        println!("SHOT ABOVE TARGET {} KILOMETERS", explosion.delta.2);
                    }

                    println!(
                        "APPROX POSITION OF EXPLOSION:  X= {}   Y= {}   Z= {}",
                        explosion.coords.0, explosion.coords.1, explosion.coords.2
                    );
                    println!(
                        "     DISTANCE FROM TARGET = {}",
                        explosion.distance_from_target
                    );
                }
                target::FireResult::Hit(shots_fired, distance_from_target) => {
                    println!("\n * * * HIT * * *   TARGET IS NON-FUNCTIONAL\n");
                    println!(
                        "DISTANCE OF EXPLOSION FROM TARGET WAS {distance_from_target} KILOMETERS."
                    );
                    println!("\nMISSION ACCOMPLISHED IN {shots_fired} SHOTS.");
                    break;
                }
            }
        }
        println!("\n\n\n\n\nNEXT TARGET...\n");
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

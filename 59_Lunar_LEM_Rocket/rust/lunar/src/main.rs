use std::{
    io::{BufRead, Write},
    str::FromStr,
};

fn main() {
    println!("{:>33}LUNAR", "");
    println!("{:>15}CREATIVE COMPUTING MORRISTOWN, NEW JERSEY\n\n\n", "");

    println!("This is a computer simulation of an apollo lunar");
    println!("landing capsule.\n\n");
    println!("The on-board computer has failed (it was made by");
    println!("Xerox) so you have to land the capsule manually.\n");
    println!("Set burn rate of retro rockets to any value between");
    println!("0 (free fall) and 200 (maximum burn) pounds per second.");
    println!("Set new burn rate every 10 seconds.\n");
    println!("Capsule weight 32,500 LBS; Fuel weight 16,000 LBS.");
    println!("\n\n\nGood luck");

    let mut lunar = LunarGame::new();

    lunar.play();
}

struct LunarGame {
    seconds: f32,  // "L" variable in basic
    distance: f32, // "A" variable, distance in miles
    velocity: f32, // "V" variable, miles per hour
    mass: f32,     // "M" variable - mass of capsule + mass of fuel
    time: f32,     // "T" variable - time remaining in current loop
}

impl LunarGame {
    const G_VAR: f32 = 0.001;
    const N_VAR: f32 = 16_500.0;
    const Z_VAR: f32 = 1.8;

    fn new() -> Self {
        Self {
            seconds: 0.0,
            distance: 120.0,
            velocity: 1.0,
            mass: 33_000.0,
            time: 0.0,
        }
    }

    fn play(&mut self) {
        let mut svar = 0.0;
        println!(
            "{:<9} {:<8} {:<10} {:<8} BURN RATE",
            "SEC", "MI + FT", "MPH", "LB_FUEL"
        );

        'moon_landing: loop {
            // the "K" variable in basic is the burn rate
            let burn_rate: f32 = get_input(self.current_status());
            self.time = 10.0; // "T" variable

            'time_loop: while self.time >= 0.001 {
                // Line 160: out-of-fuel check
                if self.mass - Self::N_VAR < 0.001 {
                    break;
                }

                svar = self.time;

                if self.mass < Self::N_VAR + svar * burn_rate {
                    svar = (self.mass - Self::N_VAR) / burn_rate;
                }

                // Line 200: get new dist + velocity, and check if we hit the moon
                let (new_distance, new_velocity) = self.calc_burn(svar, burn_rate);
                if new_distance <= 0.0 {
                    self.reached_moon(svar, burn_rate);
                    break 'moon_landing;
                }

                // Line 210: is our current velocity negative?
                if self.velocity <= 0.0 {
                    self.update(svar, burn_rate, new_distance, new_velocity);
                    continue; // stay in time loop
                }

                // Line 220: is our new_velocity negative?
                let mut check_velocity = new_velocity;
                while check_velocity < 0.0 {
                    // Line 370: update velocity & time
                    let vnext = (1.0 - self.mass * Self::G_VAR / (Self::Z_VAR * burn_rate)) / 2.0;
                    svar = self.mass * self.velocity
                        / (Self::Z_VAR
                            * burn_rate
                            * (vnext + (vnext * vnext + self.velocity / Self::Z_VAR).sqrt()))
                        + 0.05;
                    let (new_velocity, new_distance) = self.calc_burn(svar, burn_rate);

                    // Line 380: did we get to the moon?
                    if new_distance <= 0.0 {
                        self.reached_moon(svar, burn_rate);
                        break 'moon_landing;
                    }

                    // Line 390: see if our velocity wet positive
                    self.update(svar, burn_rate, new_distance, new_velocity);
                    if new_velocity > 0.0 {
                        continue 'time_loop;
                    }

                    if self.velocity <= 0.0 {
                        continue 'time_loop;
                    }
                    check_velocity = new_velocity;
                }

                // Line 230: update current state and continue
                self.update(svar, burn_rate, new_distance, new_velocity);
            }

            if self.mass - Self::N_VAR < 0.001 {
                println!("Fuel out at {} seconds.", self.seconds);
                self.velocity += Self::G_VAR * svar;
                self.seconds += svar;
                break;
            }
        }

        let landing_velocity = 3600.0 * self.velocity;
        println!(
            "On moon at {} seconds - Impact velocity {} mph",
            self.seconds, landing_velocity
        );
        if landing_velocity <= 1.2 {
            println!("Perfect landing!");
        } else if landing_velocity <= 10.0 {
            println!("Good landing (could be better)");
        } else if landing_velocity > 60.0 {
            println!("Sorry there were no survivors. You blew it!");
            println!(
                "In fact, you blasted a new lunar crater {} feet deep!",
                landing_velocity * 0.227
            );
        } else {
            println!("Craft damage... you're stranded here until a rescue");
            println!("party arrives. Hope you have enough oxygen!");
        }
    }

    // Subroutine at line 420: calculate "I" and "J"
    fn calc_burn(&self, svar: f32, burn_rate: f32) -> (f32, f32) {
        let q = svar * burn_rate / self.mass;

        let j = self.velocity
            + Self::G_VAR * svar
            + Self::Z_VAR
                * (-q - q * q - q.powf(3.0) / 3.0 - q.powf(4.0) / 4.0 - q.powf(5.0) / 5.0);
        let i = self.distance - Self::G_VAR * svar * svar / 2.0 - self.velocity * svar
            + Self::Z_VAR
                * svar
                * (q / 2.0
                    + q.powf(2.0) / 6.0
                    + q.powf(3.0) / 12.0
                    + q.powf(4.0) / 20.0
                    + q.powf(5.0) / 30.0);
        (i, j)
    }

    // Subroutine at line 330: updating current state
    fn update(&mut self, svar: f32, burn_rate: f32, new_distance: f32, new_velocity: f32) {
        self.seconds += svar;
        self.time -= svar;
        self.mass -= svar * burn_rate;
        self.distance = new_distance;
        self.velocity = new_velocity;
    }

    // Loop that starts at line 340
    fn reached_moon(&mut self, mut svar: f32, burn_rate: f32) {
        while svar > 0.005 {
            let delta = self.velocity
                * (self.velocity * self.velocity
                    + 2.0 * self.distance * (Self::G_VAR - Self::Z_VAR * burn_rate / self.mass))
                    .sqrt();
            svar = 2.0 * self.distance / delta;
            let (new_distance, new_velocity) = self.calc_burn(svar, burn_rate);
            self.update(svar, burn_rate, new_distance, new_velocity);
        }
    }

    fn current_status(&self) -> String {
        // "SEC", "MI + FT", "MPH", "LB FUEL", "BURN RATE"
        format!(
            "{:<9.2} {:<3} {:<4} {:<10.2} {:<8.2}  ",
            self.seconds,
            self.distance.floor(),
            (5280.0 * (self.distance - self.distance.floor())).floor(),
            3600.0 * self.velocity,
            self.mass - Self::N_VAR
        )
    }
}

fn get_input<R: FromStr, S: AsRef<str>>(prompt: S) -> R {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut stdout = std::io::stdout().lock();
        let _ = stdout.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);

        if let Ok(result) = buffer.trim().to_string().parse::<R>() {
            return result;
        }
        println!("?Re-enter");
    }
}

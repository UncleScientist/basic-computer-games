use rand::prelude::*;

#[derive(Debug)]
pub struct Slalom {
    gates: usize,
    level: SkillLevel,
    current_gate: usize,
    speed: i32,
    skiing: bool,
    rng: ThreadRng,
    time: f64,
    gold_count: usize,
    silver_count: usize,
    bronze_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Debug, Copy, Clone)]
pub enum Success {
    NothingSpecial,
    CloseOne,
    YouMadeIt,
    MadeItOverMax,
    FinishedRace,
}

#[derive(Debug, Copy, Clone)]
pub enum Failure {
    OfficialCaughtYou,
    WipedOut,
    SnaggedFlag,
}

#[derive(Debug, Copy, Clone)]
pub enum CommandOutcome {
    Success(Success),
    Failure(Failure),
}

#[derive(Debug, Copy, Clone)]
pub enum ErrorOutcome {
    RaceIsOver,
    TooSlow,
}

#[derive(Debug, Copy, Clone)]
pub enum SpeedAdjustment {
    IncreaseMax,
    IncreaseSome,
    IncreaseTeensy,
    NoChange,
    DecreaseTeensy,
    DecreaseSome,
    DecreaseMax,
}

#[derive(Debug, Copy, Clone)]
pub enum Medal {
    None,
    Bronze,
    Silver,
    Gold,
}

impl Slalom {
    const GATESPEED: [i32; 25] = [
        14, 18, 26, 29, 18, 25, 28, 32, 29, 20, 29, 29, 25, 21, 26, 29, 20, 21, 20, 18, 26, 25, 33,
        31, 22,
    ];

    pub fn new(gates: usize, level: SkillLevel) -> Self {
        let mut rng = rand::rng();
        let speed = rng.random_range(0..9) + 9; // 510 LET S=INT(RND(1)*(18-9)+9)

        Self {
            // Invariants
            gates,
            level,
            rng,

            // Medals
            gold_count: 0,
            silver_count: 0,
            bronze_count: 0,

            // Resettables
            current_gate: 0,
            skiing: true,
            speed,
            time: 0.0,
        }
    }

    pub fn start_new_race(&mut self) {
        self.current_gate = 0;
        self.skiing = true;
        self.speed = self.rng.random_range(0..9) + 9;
        self.time = 0.0;
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn speed(&self) -> i32 {
        self.speed
    }

    pub fn gate_speeds(&self, count: usize) -> &[i32] {
        &Self::GATESPEED[0..count]
    }

    pub fn adjust_speed(
        &mut self,
        adjustment: &SpeedAdjustment,
    ) -> Result<CommandOutcome, ErrorOutcome> {
        if !self.skiing || self.current_gate > self.gates {
            return Err(ErrorOutcome::RaceIsOver);
        }

        // 990 ON O1 GOTO 1130,1010,1170,1080,1190,1100,1150,1210
        // [except 1210 is handled by try_cheating()]
        let adj = match adjustment {
            SpeedAdjustment::IncreaseMax => self.rng.random_range(0..5) + 5,
            SpeedAdjustment::IncreaseSome => self.rng.random_range(0..2) + 3,
            SpeedAdjustment::IncreaseTeensy => self.rng.random_range(0..3) + 1,
            SpeedAdjustment::NoChange => 0,
            SpeedAdjustment::DecreaseTeensy => -self.rng.random_range(0..3) + 1,
            SpeedAdjustment::DecreaseSome => -self.rng.random_range(0..2) + 3,
            SpeedAdjustment::DecreaseMax => -self.rng.random_range(0..5) + 5,
        };

        if self.speed + adj < 7 {
            return Err(ErrorOutcome::TooSlow);
        }

        self.time += (Self::GATESPEED[self.current_gate] - self.speed + 1) as f64;

        self.speed += adj;
        self.current_gate += 1;

        if self.speed > Self::GATESPEED[self.current_gate] {
            // 1290 IF RND(1)<((S-Q)*.1)+.2 THEN 1320
            if self.rng.random::<f64>()
                < ((self.speed - Self::GATESPEED[self.current_gate]) as f64 * 0.1) + 0.2
            {
                self.skiing = false;
                if self.rng.random::<bool>() {
                    return Ok(CommandOutcome::Failure(Failure::SnaggedFlag));
                } else {
                    return Ok(CommandOutcome::Failure(Failure::WipedOut));
                }
            } else {
                return Ok(CommandOutcome::Success(Success::YouMadeIt));
            }
        } else if self.speed > Self::GATESPEED[self.current_gate] - 1 {
            return Ok(CommandOutcome::Success(Success::CloseOne));
        }

        Ok(CommandOutcome::Success(Success::NothingSpecial))
    }

    pub fn try_cheating(&mut self) -> Result<CommandOutcome, ErrorOutcome> {
        if !self.skiing || self.current_gate > self.gates {
            return Err(ErrorOutcome::RaceIsOver);
        }

        if self.rng.random::<f64>() < 0.7 {
            return Ok(CommandOutcome::Failure(Failure::OfficialCaughtYou));
        }

        self.time += 1.5;

        Ok(CommandOutcome::Success(Success::YouMadeIt))
    }

    pub fn get_result(&mut self) -> Medal {
        let medal = self.time / (self.speed as f64);
        let level: f64 = self.level.into();

        if medal < 1.5f64 - (level * 0.1) {
            self.gold_count += 1;
            Medal::Gold
        } else if medal < 2.9 - (level * 0.1) {
            self.silver_count += 1;
            Medal::Silver
        } else if medal < 4.4 - (level * 0.01) {
            self.bronze_count += 1;
            Medal::Bronze
        } else {
            Medal::None
        }
    }
}

impl From<SkillLevel> for f64 {
    fn from(value: SkillLevel) -> Self {
        match value {
            SkillLevel::Beginner => 1.0,
            SkillLevel::Intermediate => 2.0,
            SkillLevel::Expert => 3.0,
        }
    }
}

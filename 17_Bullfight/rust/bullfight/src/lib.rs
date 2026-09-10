use std::fmt::Display;

use rand::{distr::StandardUniform, prelude::*};

#[derive(Debug)]
pub struct Bullfight {
    bull_ability: Ability,
    d: [f32; 3],
    rng: ThreadRng,
    pub toreadores: PrepResult,
    pub picadores: PrepResult,
    technique: f32,    // Note: This is the "L" variable in the basic code
    bull_killed: bool, // Note: this is D(5) in basic
    bravery: Bravery,  // Note: this is D(4) in basic
}

impl Default for Bullfight {
    fn default() -> Self {
        Self::new()
    }
}

impl Bullfight {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let bull_ability = rng.random::<Ability>();

        let picador_data = Self::bull_prep(&mut rng, bull_ability, Human::Picador);
        let d_1 = picador_data.prep_level; // Line 610
        let toreador_data = Self::bull_prep(&mut rng, bull_ability, Human::Toreador);
        let d_2 = toreador_data.prep_level; // Line 650

        Self {
            bull_ability,
            d: [d_1, d_2, 0.0],
            rng,
            picadores: picador_data,
            toreadores: toreador_data,
            technique: 1.0, // Line 202
            bull_killed: false,
            bravery: Bravery::Normal,
        }
    }

    pub fn bull_ability(&self) -> Ability {
        self.bull_ability
    }

    // (0..3), (0..1.5), (0..1), (0..0.75), (0..0.6)
    fn bull_prep(rng: &mut ThreadRng, bull_ability: Ability, human: Human) -> PrepResult {
        let bull_ability: f32 = bull_ability.into();
        let range: f32 = rng.sample(StandardUniform);

        // Line 1610
        let b = (3.0f32 / bull_ability) * range;

        // Lines 1620-1750
        let prep_level = if b < 0.37 {
            0.5
        } else if b < 0.5 {
            0.4
        } else if b < 0.63 {
            0.3
        } else if b < 0.87 {
            0.2
        } else {
            0.1
        };
        let ability: Ability = ((10.0 * prep_level + 0.2) as usize).into();

        // Lines 1770-1910
        match ability {
            Ability::Superb | Ability::Good | Ability::Fair => PrepResult {
                ability,
                prep_level,
                horses_killed: 0,
                people_killed: 0,
            },
            Ability::Poor => PrepResult {
                ability,
                prep_level,
                horses_killed: 0,
                people_killed: rng.random_range(0..=1),
            },
            Ability::Awful => {
                let (horses_killed, people_killed) = match human {
                    Human::Picador => (rng.random_range(1..=2), rng.random_range(1..=2)),
                    Human::Toreador => (0, rng.random_range(1..=2)),
                };
                PrepResult {
                    ability,
                    prep_level,
                    horses_killed,
                    people_killed,
                }
            }
        }
    }

    pub fn next_pass(&mut self) -> usize {
        self.d[2] += 1.0; // Line 690
        self.d[2] as usize
    }

    pub fn cape_move(&mut self, action: CapeMove) -> Outcome {
        let m: f32 = match action {
            CapeMove::Veronica => 3.0,
            CapeMove::Outside => 2.0,
            CapeMove::Swirl => 0.5,
        };

        self.technique += m; // Line 930

        let bull_ability: f32 = self.bull_ability.into();
        let f = (6.0 - bull_ability + m / 10.0) * self.basic_rnd()
            / ((self.d[0] + self.d[1] + self.d[2] / 10.0) * 5.0);

        if f < 0.51 {
            return Outcome::Continue;
        }

        self.check_for_death()
    }

    pub fn check_for_death(&mut self) -> Outcome {
        match self.flip_coin() {
            true => {
                self.bravery = Bravery::Heightened; // Line 990
                Outcome::PlayerDead
            }
            false => Outcome::StillAlive,
        }
    }

    pub fn after_bull_charge(&mut self, action: RunOrRemain) -> Outcome {
        match action {
            RunOrRemain::Run => {
                self.bravery = Bravery::Coward; // Line 1050
                Outcome::Done
            }
            RunOrRemain::Remain => match self.flip_coin() {
                true => {
                    self.bravery = Bravery::Fearless; // Line 1090
                    Outcome::Continue
                }
                false => self.check_for_death(),
            },
        }
    }

    pub fn kill_move(&mut self, action: KillMove) -> Outcome {
        let bull_ability: f32 = self.bull_ability.into();
        let k = (6.0 - bull_ability) * 10.0 * self.basic_rnd()
            / ((self.d[0] + self.d[1]) * 5.0 * self.d[2]);

        match action {
            KillMove::OverTheHorns => {
                if k > 0.8 {
                    return self.check_for_death();
                }
            }
            KillMove::InTheChest => {
                if k > 0.2 {
                    return self.check_for_death();
                }
            }
        }

        self.bull_killed = true; // Line 1270
        Outcome::BullDead
    }

    pub fn try_to_kill(&mut self, kill_move: KillMove) -> KillResult {
        let outcome = self.kill_move(kill_move);
        match outcome {
            Outcome::PlayerDead => KillResult::PlayerDead,
            Outcome::StillAlive => KillResult::ContinueGame,
            Outcome::BullDead => KillResult::BullDead,
            Outcome::Done | Outcome::Continue => unreachable!(),
        }
    }

    pub fn final_result(&mut self) -> (Crowd, Award) {
        let crowd = if self.bravery == Bravery::Fearless {
            Crowd::CheerWildly
        } else if self.bull_killed {
            Crowd::Cheer
        } else {
            Crowd::RemainSilent
        };

        let award = if self.award_chance() < 2.4 {
            Award::NothingAtAll
        } else if self.award_chance() < 4.9 {
            Award::SingleEar
        } else if self.award_chance() < 7.4 {
            Award::BothEars
        } else {
            Award::MuyHombre
        };

        (crowd, award)
    }

    fn flip_coin(&mut self) -> bool {
        self.rng.random::<bool>()
    }

    fn basic_rnd(&mut self) -> f32 {
        let num = (self.rng.random::<u32>()) as f32;
        num / (u32::MAX as f32)
    }

    fn award_chance(&mut self) -> f32 {
        let bull_ability: f32 = self.bull_ability.into();
        4.5 + self.technique / 6.0 - (self.d[0] + self.d[1]) * 2.5
            + 4.0 * self.bravery.val()
            + if self.bull_killed { 4.0 } else { 2.0 }
            - (self.d[2] * self.d[2]) / 120.0
            - bull_ability
    }

    pub fn bull_killed(&self) -> bool {
        self.bull_killed
    }
}

#[derive(Copy, Clone)]
pub enum Crowd {
    CheerWildly,
    Cheer,
    RemainSilent,
}

#[derive(Copy, Clone)]
pub enum Award {
    MuyHombre,
    BothEars,
    SingleEar,
    NothingAtAll,
}

#[derive(Debug, Copy, Clone)]
pub enum KillResult {
    PlayerDead,
    BullDead,
    ContinueGame,
}

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Continue,
    PlayerDead,
    StillAlive,
    Done,
    BullDead,
}

#[derive(Copy, Clone)]
pub enum CapeMove {
    Veronica,
    Outside,
    Swirl,
}

#[derive(Copy, Clone)]
pub enum KillMove {
    OverTheHorns,
    InTheChest,
}

#[derive(Copy, Clone)]
pub enum RunOrRemain {
    Run,
    Remain,
}

#[derive(Debug)]
pub struct PrepResult {
    pub ability: Ability,
    prep_level: f32,
    pub horses_killed: usize,
    pub people_killed: usize,
}

enum Human {
    Picador,
    Toreador,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Bravery {
    Normal,     // 1
    Heightened, // 1.5
    Coward,     // 0
    Fearless,   // 2
}

impl Bravery {
    fn val(&self) -> f32 {
        match self {
            Bravery::Normal => 1.0,
            Bravery::Heightened => 1.5,
            Bravery::Coward => 0.0,
            Bravery::Fearless => 2.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Ability {
    Superb,
    Good,
    Fair,
    Poor,
    Awful,
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ability::Superb => "Superb",
                Ability::Good => "Good",
                Ability::Fair => "Fair",
                Ability::Poor => "Poor",
                Ability::Awful => "Awful",
            }
        )
    }
}

impl From<Ability> for f32 {
    fn from(value: Ability) -> Self {
        match value {
            Ability::Superb => 1.0,
            Ability::Good => 2.0,
            Ability::Fair => 3.0,
            Ability::Poor => 4.0,
            Ability::Awful => 5.0,
        }
    }
}

impl From<usize> for Ability {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::Superb,
            2 => Self::Good,
            3 => Self::Fair,
            4 => Self::Poor,
            5 => Self::Awful,
            _ => panic!("invalid value {value} for Ability"),
        }
    }
}

impl Distribution<Ability> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ability {
        let index: u8 = rng.random_range(0..=4);
        match index {
            0 => Ability::Superb,
            1 => Ability::Good,
            2 => Ability::Fair,
            3 => Ability::Poor,
            4 => Ability::Awful,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_bullfight() {
        let bullfight = Bullfight::new();
        assert!(bullfight.d[0] <= 0.5);
        assert!(bullfight.d[1] <= 0.5);
    }

    #[test]
    fn test_rnd() {
        let mut bullfight = Bullfight::new();
        for _ in 0..100 {
            let num = bullfight.basic_rnd();
            assert!(0.0 <= num && num < 1.0);
        }
    }
}

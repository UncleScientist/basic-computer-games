use rand::{distr::StandardUniform, prelude::*};

#[derive(Debug)]
pub struct Bullfight {
    bull_ability: Ability,
    d: [f32; 5],
    rng: ThreadRng,
    toreadores: Ability,
    picadores: Ability,
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
            d: [d_1, d_2, 0.0, 1.0, 1.0],
            rng,
            picadores: picador_data.ability,
            toreadores: toreador_data.ability,
        }
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
            Ability::Superb | Ability::Good | Ability::Fair => {
                return PrepResult {
                    ability,
                    prep_level,
                    horses_killed: 0,
                    people_killed: 0,
                };
            }
            Ability::Poor => {
                return PrepResult {
                    ability,
                    prep_level,
                    horses_killed: 0,
                    people_killed: rng.random_range(0..=1),
                };
            }
            Ability::Awful => {
                let (horses_killed, people_killed) = match human {
                    Human::Picador => (rng.random_range(1..=2), rng.random_range(1..=2)),
                    Human::Toreador => (0, rng.random_range(1..=2)),
                };
                return PrepResult {
                    ability,
                    prep_level,
                    horses_killed,
                    people_killed,
                };
            }
        }
    }

    pub fn cape_move(&mut self, action: CapeMove) {}
    pub fn kill_move(&mut self, action: KillMove) {}
}

#[derive(Copy, Clone)]
pub enum CapeMove {
    Veronica,
    OutsideCape,
    CapeSwirl,
}

#[derive(Copy, Clone)]
pub enum KillMove {
    OverTheHorns,
    InTheChest,
}

struct PrepResult {
    ability: Ability,
    prep_level: f32,
    horses_killed: usize,
    people_killed: usize,
}

enum Human {
    Picador,
    Toreador,
}

#[derive(Copy, Clone, Debug)]
pub enum Ability {
    Superb,
    Good,
    Fair,
    Poor,
    Awful,
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
}

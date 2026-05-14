const COMPUTER_ARMY: Unit = Unit::Army(30000);
const COMPUTER_NAVY: Unit = Unit::Navy(20000);
const COMPUTER_AIR_FORCE: Unit = Unit::AirForce(22000);

const TOTAL_UNITS: usize = 72000;

#[derive(Debug)]
pub struct Combat {
    player: Units,
    computer: Units,
}

#[derive(Debug, PartialEq)]
pub struct Units {
    army: Unit,
    navy: Unit,
    air_force: Unit,
}

#[cfg(test)]
impl Units {
    const fn build(army: usize, navy: usize, air_force: usize) -> Self {
        Self {
            army: Unit::Army(army),
            navy: Unit::Navy(navy),
            air_force: Unit::AirForce(air_force),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Army(usize),
    Navy(usize),
    AirForce(usize),
}

impl Unit {
    fn units(&self) -> usize {
        match self {
            Unit::Army(a) => *a,
            Unit::Navy(n) => *n,
            Unit::AirForce(af) => *af,
        }
    }

    fn lose(&mut self, amount: usize) {
        match self {
            Unit::Army(a) => *self = Unit::Army(*a - amount),
            Unit::Navy(n) => *self = Unit::Navy(*n - amount),
            Unit::AirForce(af) => *self = Unit::AirForce(*af - amount),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FirstBattleResult {
    TooManyUnits,                                                  // Line 100
    PlayerLose { units: Unit },                                    // Line 120
    BothLose { player: Unit, computer: Unit },                     // Line 150
    ComputerStoppedAttack,                                         // Line 230
    ComputerLose { units: Unit },                                  // Line 250
    ComputerLosePatrolBoat,                                        // Line 270-275
    AttackWipedOut,                                                // Line 350
    Dogfight,                                                      // Line 370
    ComputerLoseArmyPatrol { p1: Unit, p2: Unit, computer: Unit }, // Line 380-381
}

pub enum SecondBattleResult {
    PlayerDestroyedComputer,    // Line 1615
    ComputerWipedOutAttack,     // Line 1630
    ComputerSankTwoBattleships, // Line 1750
    PlayerShotDownPlanes,       // Line 1770
    PlayerInShambles,           // Line 1830
    PlayerCrashedIntoHouse,     // Line 1850
}

#[derive(Debug, PartialEq)]
pub enum BuildError {
    TooManyUnits,
}

impl Combat {
    pub fn new(army: usize, navy: usize, air_force: usize) -> Result<Self, BuildError> {
        if army + navy + air_force > TOTAL_UNITS {
            return Err(BuildError::TooManyUnits);
        }

        Ok(Self {
            player: Units {
                army: Unit::Army(army),
                navy: Unit::Navy(navy),
                air_force: Unit::AirForce(air_force),
            },
            computer: Units {
                army: COMPUTER_ARMY,
                navy: COMPUTER_NAVY,
                air_force: COMPUTER_AIR_FORCE,
            },
        })
    }

    pub fn first_battle(&mut self, forces: Unit) -> FirstBattleResult {
        match forces {
            Unit::Army(army) => {
                let units = self.player.army.units();
                if army > units {
                    return FirstBattleResult::TooManyUnits;
                }
                if army < units / 3 {
                    self.player.army = Unit::Army(units - army);
                    return FirstBattleResult::PlayerLose {
                        units: Unit::Army(army),
                    };
                }
                if army < 2 * units / 3 {
                    let player_lost = army / 3;
                    let computer_lost = 2 * self.computer.army.units() / 3;
                    self.player.army.lose(player_lost);
                    self.computer.army.lose(computer_lost);
                    return FirstBattleResult::BothLose {
                        player: Unit::Army(player_lost),
                        computer: Unit::Army(computer_lost),
                    };
                }
                self.player.army = Unit::Army(self.player.army.units() / 3);
                self.player.air_force = Unit::AirForce(self.player.air_force.units() / 3);
                self.computer.navy = Unit::Navy(2 * self.computer.navy.units() / 3);
                FirstBattleResult::ComputerLosePatrolBoat
            }
            Unit::Navy(navy) => {
                if navy > self.player.navy.units() {
                    return FirstBattleResult::TooManyUnits;
                }
                if navy < self.player.navy.units() / 3 {
                    self.player.navy.lose(navy);
                    return FirstBattleResult::ComputerStoppedAttack;
                }
                if navy < 2 * self.player.navy.units() / 3 {
                    let loss = 2 * self.computer.navy.units() / 3;
                    self.computer.navy = Unit::Navy(self.computer.navy.units() / 3);
                    return FirstBattleResult::ComputerLose {
                        units: Unit::Navy(loss),
                    };
                }
                // TODO: Fix code duplication (see army above)
                self.player.army = Unit::Army(self.player.army.units() / 3);
                self.player.air_force = Unit::AirForce(self.player.air_force.units() / 3);
                self.computer.navy = Unit::Navy(2 * self.computer.navy.units() / 3);
                FirstBattleResult::ComputerLosePatrolBoat
            }
            Unit::AirForce(air_force) => {
                if air_force > self.player.air_force.units() {
                    return FirstBattleResult::TooManyUnits;
                }
                if air_force < self.player.air_force.units() / 3 {
                    self.player.air_force =
                        Unit::AirForce(self.player.air_force.units() - air_force);
                    return FirstBattleResult::AttackWipedOut;
                }
                if air_force < 2 * self.player.air_force.units() / 3 {
                    self.computer = Units {
                        army: Unit::Army(2 * self.computer.army.units() / 3),
                        navy: Unit::Navy(self.computer.navy.units() / 3),
                        air_force: Unit::AirForce(self.computer.air_force.units() / 3),
                    };
                    return FirstBattleResult::Dogfight;
                }
                self.player.army = Unit::Army(self.player.army.units() / 4);
                self.player.navy = Unit::Navy(self.player.navy.units() / 3);
                self.computer.army = Unit::Army(2 * self.computer.army.units() / 3);
                FirstBattleResult::AttackWipedOut
            }
        }
    }

    pub fn get_current_results(&self) -> (&Units, &Units) {
        (&self.player, &self.computer)
    }

    pub fn second_battle(&mut self, forces: Unit) -> SecondBattleResult {
        match forces {
            Unit::Army(_) => todo!(),
            Unit::Navy(_) => todo!(),
            Unit::AirForce(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_invalid_game() {
        let combat = Combat::new(60000, 70000, 80000);
        assert_eq!(BuildError::TooManyUnits, combat.expect_err("error"));
    }

    #[test]
    fn test_create_valid_game() {
        let combat = Combat::new(25000, 27000, 20000).expect("Valid Game");
        assert_eq!(Unit::Army(25000), combat.player.army);
        assert_eq!(Unit::Army(30000), combat.computer.army);
    }

    const FIRST_BATTLES: [(Unit, FirstBattleResult, Units, Units); 11] = [
        (
            Unit::Army(30000),
            FirstBattleResult::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Army(500),
            FirstBattleResult::PlayerLose {
                units: Unit::Army(500),
            },
            Units::build(24500, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Army(15000),
            FirstBattleResult::BothLose {
                player: Unit::Army(5000),
                computer: Unit::Army(20000),
            },
            Units::build(20000, 27000, 20000),
            Units::build(10000, 20000, 22000),
        ),
        (
            Unit::Navy(80000),
            FirstBattleResult::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Navy(8000),
            FirstBattleResult::ComputerStoppedAttack,
            Units::build(25000, 19000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Navy(16000),
            FirstBattleResult::ComputerLose {
                units: Unit::Navy(13333),
            },
            Units::build(25000, 27000, 20000),
            Units::build(30000, 6666, 22000),
        ),
        (
            Unit::Navy(18000),
            FirstBattleResult::ComputerLosePatrolBoat,
            Units::build(25000 / 3, 27000, 20000 / 3),
            Units::build(30000, 2 * 20000 / 3, 22000),
        ),
        (
            Unit::AirForce(80000),
            FirstBattleResult::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::AirForce(6000),
            FirstBattleResult::AttackWipedOut,
            Units::build(25000, 27000, 14000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::AirForce(12000),
            FirstBattleResult::Dogfight,
            Units::build(25000, 27000, 20000),
            Units::build(2 * 30000 / 3, 20000 / 3, 22000 / 3),
        ),
        (
            Unit::AirForce(15000),
            FirstBattleResult::AttackWipedOut,
            Units::build(25000 / 4, 27000 / 3, 20000),
            Units::build(2 * 30000 / 3, 20000, 22000),
        ),
    ];

    #[test]
    fn test_first_battles() {
        for battle in FIRST_BATTLES {
            println!("{:?}", battle.0);
            let mut combat = Combat::new(25000, 27000, 20000).expect("Valid Game");
            let result = combat.first_battle(battle.0);
            assert_eq!(battle.1, result);
            assert_eq!(combat.player, battle.2);
            assert_eq!(combat.computer, battle.3);
        }
    }
}

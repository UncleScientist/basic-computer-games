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

impl Units {
    #[cfg(test)]
    const fn build(army: usize, navy: usize, air_force: usize) -> Self {
        Self {
            army: Unit::Army(army),
            navy: Unit::Navy(navy),
            air_force: Unit::AirForce(air_force),
        }
    }

    fn score(&self) -> usize {
        self.army.units() + self.navy.units() + self.air_force.units()
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
pub enum FirstBattleOutcome {
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

#[derive(Debug, PartialEq)]
pub enum SecondBattleOutcome {
    TooManyUnits,               // Line 80
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

#[derive(Debug, PartialEq)]
pub enum CombatOutcome {
    PlayerWins,
    ComputerWins,
    TreatyOfParis,
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

    pub fn first_battle(&mut self, forces: Unit) -> FirstBattleOutcome {
        match forces {
            Unit::Army(army) => {
                let units = self.player.army.units();
                if army > units {
                    return FirstBattleOutcome::TooManyUnits;
                }
                if army < units / 3 {
                    self.player.army = Unit::Army(units - army);
                    return FirstBattleOutcome::PlayerLose {
                        units: Unit::Army(army),
                    };
                }
                if army < 2 * units / 3 {
                    let player_lost = army / 3;
                    let computer_lost = 2 * self.computer.army.units() / 3;
                    self.player.army.lose(player_lost);
                    self.computer.army.lose(computer_lost);
                    return FirstBattleOutcome::BothLose {
                        player: Unit::Army(player_lost),
                        computer: Unit::Army(computer_lost),
                    };
                }
                self.player.army = Unit::Army(self.player.army.units() / 3);
                self.player.air_force = Unit::AirForce(self.player.air_force.units() / 3);
                self.computer.navy = Unit::Navy(2 * self.computer.navy.units() / 3);
                FirstBattleOutcome::ComputerLosePatrolBoat
            }
            Unit::Navy(navy) => {
                if navy > self.player.navy.units() {
                    return FirstBattleOutcome::TooManyUnits;
                }
                if navy < self.player.navy.units() / 3 {
                    self.player.navy.lose(navy);
                    return FirstBattleOutcome::ComputerStoppedAttack;
                }
                if navy < 2 * self.player.navy.units() / 3 {
                    let loss = 2 * self.computer.navy.units() / 3;
                    self.computer.navy = Unit::Navy(self.computer.navy.units() / 3);
                    return FirstBattleOutcome::ComputerLose {
                        units: Unit::Navy(loss),
                    };
                }
                // TODO: Fix code duplication (see army above)
                self.player.army = Unit::Army(self.player.army.units() / 3);
                self.player.air_force = Unit::AirForce(self.player.air_force.units() / 3);
                self.computer.navy = Unit::Navy(2 * self.computer.navy.units() / 3);
                FirstBattleOutcome::ComputerLosePatrolBoat
            }
            Unit::AirForce(air_force) => {
                if air_force > self.player.air_force.units() {
                    return FirstBattleOutcome::TooManyUnits;
                }
                if air_force < self.player.air_force.units() / 3 {
                    self.player.air_force =
                        Unit::AirForce(self.player.air_force.units() - air_force);
                    return FirstBattleOutcome::AttackWipedOut;
                }
                if air_force < 2 * self.player.air_force.units() / 3 {
                    self.computer = Units {
                        army: Unit::Army(2 * self.computer.army.units() / 3),
                        navy: Unit::Navy(self.computer.navy.units() / 3),
                        air_force: Unit::AirForce(self.computer.air_force.units() / 3),
                    };
                    return FirstBattleOutcome::Dogfight;
                }
                self.player.army = Unit::Army(self.player.army.units() / 4);
                self.player.navy = Unit::Navy(self.player.navy.units() / 3);
                self.computer.army = Unit::Army(2 * self.computer.army.units() / 3);
                FirstBattleOutcome::AttackWipedOut
            }
        }
    }

    pub fn get_current_results(&self) -> (&Units, &Units) {
        (&self.player, &self.computer)
    }

    pub fn second_battle(&mut self, forces: Unit) -> SecondBattleOutcome {
        match forces {
            Unit::Army(army) => {
                let units = self.player.army.units();
                if army > units {
                    return SecondBattleOutcome::TooManyUnits;
                }
                if army < self.computer.army.units() / 2 {
                    self.player.army.lose(army);
                    return SecondBattleOutcome::ComputerWipedOutAttack;
                }
                self.computer.army = Unit::Army(0);
                SecondBattleOutcome::PlayerDestroyedComputer
            }
            Unit::Navy(navy) => {
                let units = self.player.navy.units();
                if navy > units {
                    return SecondBattleOutcome::TooManyUnits;
                }
                if navy < self.computer.navy.units() / 2 {
                    self.player.army = Unit::Army(self.player.army.units() / 4);
                    self.player.navy = Unit::Navy(self.player.navy.units() / 2);
                    return SecondBattleOutcome::ComputerSankTwoBattleships;
                }
                self.computer.navy = Unit::Navy(self.computer.navy.units() / 2);
                self.computer.air_force = Unit::AirForce(self.computer.air_force.units() * 2 / 3);
                SecondBattleOutcome::PlayerShotDownPlanes
            }
            Unit::AirForce(air_force) => {
                let units = self.player.air_force.units();
                if air_force > units {
                    return SecondBattleOutcome::TooManyUnits;
                }
                if air_force > self.computer.air_force.units() / 2 {
                    self.player.army = Unit::Army(self.player.army.units() / 3);
                    self.player.navy = Unit::Navy(self.player.navy.units() / 3);
                    self.player.air_force = Unit::AirForce(self.player.air_force.units() / 3);
                    return SecondBattleOutcome::PlayerInShambles;
                }
                SecondBattleOutcome::PlayerCrashedIntoHouse
            }
        }
    }

    pub fn combat_outcome(&self) -> CombatOutcome {
        let player_score = self.player.score();
        let computer_score = self.computer.score();
        println!("p={player_score}, c={computer_score}");

        if player_score > 3 * computer_score / 2 {
            CombatOutcome::PlayerWins
        } else if player_score < 2 * computer_score / 3 {
            CombatOutcome::ComputerWins
        } else {
            CombatOutcome::TreatyOfParis
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

    const FIRST_BATTLES: [(Unit, FirstBattleOutcome, Units, Units); 11] = [
        (
            Unit::Army(30000),
            FirstBattleOutcome::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Army(500),
            FirstBattleOutcome::PlayerLose {
                units: Unit::Army(500),
            },
            Units::build(24500, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Army(15000),
            FirstBattleOutcome::BothLose {
                player: Unit::Army(5000),
                computer: Unit::Army(20000),
            },
            Units::build(20000, 27000, 20000),
            Units::build(10000, 20000, 22000),
        ),
        (
            Unit::Navy(80000),
            FirstBattleOutcome::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Navy(8000),
            FirstBattleOutcome::ComputerStoppedAttack,
            Units::build(25000, 19000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Navy(16000),
            FirstBattleOutcome::ComputerLose {
                units: Unit::Navy(13333),
            },
            Units::build(25000, 27000, 20000),
            Units::build(30000, 6666, 22000),
        ),
        (
            Unit::Navy(18000),
            FirstBattleOutcome::ComputerLosePatrolBoat,
            Units::build(25000 / 3, 27000, 20000 / 3),
            Units::build(30000, 2 * 20000 / 3, 22000),
        ),
        (
            Unit::AirForce(80000),
            FirstBattleOutcome::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::AirForce(6000),
            FirstBattleOutcome::AttackWipedOut,
            Units::build(25000, 27000, 14000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::AirForce(12000),
            FirstBattleOutcome::Dogfight,
            Units::build(25000, 27000, 20000),
            Units::build(2 * 30000 / 3, 20000 / 3, 22000 / 3),
        ),
        (
            Unit::AirForce(15000),
            FirstBattleOutcome::AttackWipedOut,
            Units::build(25000 / 4, 27000 / 3, 20000),
            Units::build(2 * 30000 / 3, 20000, 22000),
        ),
    ];

    const SECOND_BATTLES: [(Unit, SecondBattleOutcome, Units, Units); 9] = [
        (
            Unit::Army(30000),
            SecondBattleOutcome::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Army(30000 / 2 - 1),
            SecondBattleOutcome::ComputerWipedOutAttack,
            Units::build(25000 - (30000 / 2 - 1), 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Army(30000 / 2 + 1),
            SecondBattleOutcome::PlayerDestroyedComputer,
            Units::build(25000, 27000, 20000),
            Units::build(0, 20000, 22000),
        ),
        (
            Unit::Navy(30000),
            SecondBattleOutcome::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Navy(20000 / 2 - 1),
            SecondBattleOutcome::ComputerSankTwoBattleships,
            Units::build(25000 / 4, 27000 / 2, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::Navy(20000 / 2 + 1),
            SecondBattleOutcome::PlayerShotDownPlanes,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000 / 2, 22000 * 2 / 3),
        ),
        (
            Unit::AirForce(30000),
            SecondBattleOutcome::TooManyUnits,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::AirForce(20000),
            SecondBattleOutcome::PlayerInShambles,
            Units::build(25000 / 3, 27000 / 3, 20000 / 3),
            Units::build(30000, 20000, 22000),
        ),
        (
            Unit::AirForce(1),
            SecondBattleOutcome::PlayerCrashedIntoHouse,
            Units::build(25000, 27000, 20000),
            Units::build(30000, 20000, 22000),
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

    #[test]
    fn test_second_battles() {
        for battle in SECOND_BATTLES {
            println!("{:?}", battle.0);
            let mut combat = Combat::new(25000, 27000, 20000).expect("Valid Game");
            let result = combat.second_battle(battle.0);
            assert_eq!(battle.1, result);
            assert_eq!(combat.player, battle.2);
            assert_eq!(combat.computer, battle.3);
        }
    }

    #[test]
    fn test_computer_win() {
        let combat = Combat::new(0, 0, 0).expect("Valid Game");
        assert_eq!(CombatOutcome::ComputerWins, combat.combat_outcome());
    }

    #[test]
    fn test_player_win() {
        let mut combat = Combat::new(25000, 27000, 20000).expect("Valid Game");
        combat.computer.army = Unit::Army(0);
        assert_eq!(CombatOutcome::PlayerWins, combat.combat_outcome());
    }

    #[test]
    fn test_tie() {
        let combat = Combat::new(25000, 27000, 20000).expect("Valid Game");
        assert_eq!(CombatOutcome::TreatyOfParis, combat.combat_outcome());
    }
}

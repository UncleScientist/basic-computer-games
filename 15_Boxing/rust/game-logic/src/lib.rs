use rand::{
    distr::{Distribution, StandardUniform},
    rngs::ThreadRng,
    *,
};

#[derive(Debug)]
pub struct Boxing {
    player: Boxer,
    opponent: Boxer,
    current: Option<Turn>,
    player_rounds_won: usize,
    opponent_rounds_won: usize,
    rng: ThreadRng,
}

#[derive(Debug)]
pub struct Boxer {
    name: String,
    best: Punch,
    vulerability: Punch,
    points_against: usize,
}

#[derive(Debug, PartialEq)]
pub enum Punch {
    FullSwing,
    Hook,
    Uppercut,
    Jab,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoundWinner {
    Player,
    Opponent,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    Player,
    Opponent,
}

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    Blocked,
    BloodSpills,
    InTheFace,
    AndAgain,
    Connects,
    BlocksAndHooks,
    Knockout,
    Misses,
}

impl Boxing {
    pub fn new(
        player_name: String,
        opponent_name: String,
        player_best: Punch,
        player_vulnerability: Punch,
    ) -> Self {
        let mut rng = rand::rng();
        let player = Boxer {
            name: player_name,
            best: player_best,
            vulerability: player_vulnerability,
            points_against: 0,
        };
        let best = rng.random::<Punch>();
        let vulerability = loop {
            let vul = rng.random::<Punch>();
            if vul != best {
                break vul;
            }
        };
        let opponent = Boxer {
            name: opponent_name,
            best,
            vulerability,
            points_against: 0,
        };

        Boxing {
            player,
            opponent,
            current: None,
            player_rounds_won: 0,
            opponent_rounds_won: 0,
            rng,
        }
    }

    pub fn player_name(&self) -> &str {
        self.player.name.as_str()
    }

    pub fn opponent_name(&self) -> &str {
        self.opponent.name.as_str()
    }

    pub fn start_round(&mut self) {
        self.player.points_against = 0;
        self.opponent.points_against = 0;
    }

    pub fn end_round(&mut self) -> RoundWinner {
        if self.player.points_against > self.opponent.points_against {
            self.opponent_rounds_won += 1;
            RoundWinner::Opponent
        } else {
            self.player_rounds_won += 1;
            RoundWinner::Player
        }
    }

    pub fn who_swings(&mut self) -> Turn {
        let turn = self.rng.random::<Turn>();
        self.current = Some(turn);
        turn
    }

    pub fn player_swings(&mut self, punch: Punch) -> Outcome {
        if self.current != Some(Turn::Player) {
            panic!("current boxer is not the player");
        }

        if punch == self.player.best {
            self.opponent.points_against += 2;
        }

        match punch {
            Punch::FullSwing => {
                // Lines 340-440
                if self.opponent.vulerability == punch || self.rng.random_range(0..30) < 9 {
                    if self.opponent.points_against > 35 {
                        return Outcome::Knockout;
                    }
                    self.opponent.points_against += 15;
                    Outcome::Connects
                } else {
                    Outcome::Misses
                }
            }
            Punch::Hook => {
                // Lines 450-510
                if self.opponent.vulerability == punch || self.rng.random::<bool>() {
                    self.opponent.points_against += 7;
                    Outcome::Connects
                } else {
                    Outcome::Blocked
                }
            }
            Punch::Uppercut => {
                // Lines 520-590
                if self.opponent.vulerability == punch || self.rng.random::<bool>() {
                    self.opponent.points_against += 4;
                    Outcome::Connects
                } else {
                    Outcome::Blocked
                }
            }
            Punch::Jab => {
                // Lines 270-330
                if self.opponent.vulerability == punch || self.rng.random::<bool>() {
                    self.opponent.points_against += 3;
                    Outcome::Connects
                } else {
                    Outcome::Blocked
                }
            }
        }
    }

    pub fn opponent_swings(&mut self) -> (Punch, Outcome) {
        if self.current != Some(Turn::Opponent) {
            panic!("current boxer is not the opponent");
        }

        let punch = self.rng.random::<Punch>();
        if punch == self.opponent.best {
            self.player.points_against += 2;
        }

        let outcome = match punch {
            Punch::FullSwing => {
                // Lines 720-800
                if self.player.vulerability == punch || self.rng.random::<bool>() {
                    if self.player.points_against > 35 {
                        return (punch, Outcome::Knockout);
                    }
                    self.player.points_against += 15;
                    Outcome::InTheFace
                } else {
                    Outcome::Blocked
                }
            }

            Punch::Hook => {
                // Lines 810-850 (changing line 850 to "GOTO 300")
                self.player.points_against += 7 + 5;
                if self.player.points_against > 35 {
                    return (punch, Outcome::Knockout);
                }
                Outcome::AndAgain
            }
            Punch::Uppercut => {
                // Lines 860-940
                if self.player.vulerability == punch || self.rng.random_range(0..200) < 74 {
                    self.player.points_against += 8;
                    Outcome::Connects
                } else {
                    self.opponent.points_against += 5;
                    Outcome::BlocksAndHooks
                }
            }
            Punch::Jab => {
                // Lines 640-710
                if self.player.vulerability == punch || self.rng.random_range(0..7) > 3 {
                    self.player.points_against += 5;
                    Outcome::BloodSpills
                } else {
                    Outcome::Blocked
                }
            }
        };

        (punch, outcome)
    }
}

impl Distribution<Punch> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Punch {
        match rng.random_range(0..=3) {
            0 => Punch::FullSwing,
            1 => Punch::Hook,
            2 => Punch::Uppercut,
            3 => Punch::Jab,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Turn> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turn {
        match rng.random_range(0..=1) {
            0 => Turn::Player,
            1 => Turn::Opponent,
            _ => unreachable!(),
        }
    }
}

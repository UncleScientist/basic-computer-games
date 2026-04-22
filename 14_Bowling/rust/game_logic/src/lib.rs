use std::fmt::Display;

use rand::{RngExt, rngs::ThreadRng};

#[derive(Debug)]
pub struct Bowling {
    players: usize, // player count
    turn: usize,    // which player is playing
    frame: usize,   // what frame they are on
    ball: usize,    // which ball they are throwing
    scores: [[usize; 7]; 101],
    rng: ThreadRng,
    current_frame: [usize; 16],
}

impl Bowling {
    pub fn new(players: usize) -> Self {
        Self {
            players,
            turn: 0,
            frame: 1,
            ball: 1,
            scores: [[0; 7]; 101],
            current_frame: [0; 16],
            rng: rand::rng(),
        }
    }

    pub fn current_turn_info(&self) -> (usize, usize, usize) {
        (self.turn + 1, self.frame, self.ball)
    }

    pub fn roll(&mut self) -> Vec<Response> {
        if self.ball == 1 {
            self.current_frame = [0; 16];
        }

        if self.frame == 11 {
            return vec![Response::GameOver];
        }

        let preroll_count = (1..=10).map(|idx| self.current_frame[idx]).sum::<usize>();

        // Lines 2880-3420: determine which pins get knocked down
        for _ in 1..=20 {
            let x = self.rng.random_range(0..100);
            for j in 1..=10 {
                if x < 15 * j {
                    self.current_frame[15 * j - x] = 1;
                    break;
                }
            }
        }

        // Determine outcome

        let mut result = Vec::new();
        let mut q = 0;

        let count = (1..=10).map(|idx| self.current_frame[idx]).sum::<usize>();
        if count == preroll_count {
            result.push(Response::Gutter);
        }
        if self.ball == 1 && count == 10 {
            result.push(Response::Strike);
            q = 3;
        }
        if self.ball == 2 && count == 10 {
            result.push(Response::Spare);
            q = 2;
        }
        if self.ball == 2 && count != 10 {
            result.push(Response::Error);
            q = 1;
        }

        if self.ball == 1 && count != 10 {
            result.push(Response::NeedsAnotherBall);
        }
        let index = self.frame * (self.turn + 1);
        if self.ball == 1 {
            self.scores[index][self.ball] = count;
            self.ball = 2;
            if q == 3 {
                self.scores[index][self.ball] = count;
                self.next_player_frame(q);
            }
        } else {
            assert_eq!(self.ball, 2);
            self.scores[index][self.ball] = count;
            self.next_player_frame(q);
        }

        if self.frame == 11 {
            result.push(Response::GameOver);
        }

        result
    }

    fn next_player_frame(&mut self, q: usize) {
        self.scores[self.frame * (self.turn + 1)][3] = q;
        self.ball = 1;

        self.turn = (self.turn + 1) % self.players;
        if self.turn == 0 {
            self.frame += 1;
        }
    }

    pub fn get_pins(&self) -> &[usize; 16] {
        &self.current_frame
    }

    pub fn get_scores(&self) -> &[[usize; 7]; 101] {
        &self.scores
    }
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Strike,
    Spare,
    Gutter,
    Error,
    NeedsAnotherBall,
    GameOver,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Response::Strike => "STRIKE!!!!!",
                Response::Spare => "SPARE!!!!",
                Response::Gutter => "GUTTER!!",
                Response::Error => "ERROR!!!",
                Response::NeedsAnotherBall => "",
                Response::GameOver => unreachable!(),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_bowling() {
        let b = Bowling::new(3);
        assert_eq!(3, b.players);
    }

    #[test]
    fn test_roll() {
        let mut b = Bowling::new(1);
        let result = b.roll();
        assert_eq!(1, result.len());
        assert!(matches!(
            result[0],
            Response::Strike | Response::NeedsAnotherBall
        ));
    }

    #[test]
    fn test_strike_possible() {
        loop {
            let mut b = Bowling::new(1);
            let result = b.roll();
            if result[0] == Response::Strike {
                break;
            }
        }
    }

    #[test]
    fn test_two_rolls() {
        loop {
            let mut b = Bowling::new(1);
            let result = b.roll();
            if result[0] == Response::Strike {
                continue;
            }
            let result = b.roll();
            if matches!(
                result[0],
                Response::Error | Response::Spare | Response::Gutter
            ) {
                break;
            }
        }
    }

    #[test]
    fn test_gutterball() {
        loop {
            let mut b = Bowling::new(1);
            let result = b.roll();
            if result[0] == Response::NeedsAnotherBall {
                let result = b.roll();
                if result[0] == Response::Gutter {
                    break;
                }
            }
        }
    }

    #[test]
    fn test_full_play() {
        let mut b = Bowling::new(2);
        loop {
            let result = b.roll();
            if result.contains(&Response::GameOver) {
                break;
            }
            assert!(b.frame < 11);
            if result[0] != Response::Strike {
                b.roll();
            }
        }
        assert_eq!(b.frame, 11);
    }
}

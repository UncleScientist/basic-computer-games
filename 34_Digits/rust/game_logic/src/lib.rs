use rand::{RngExt, rngs::ThreadRng};

pub enum GameResult {
    ComputerWins,
    PlayerWins,
    Tie,
}

#[derive(Default)]
pub struct Guesser {
    a: f32,
    b: f32,
    c: f32,
    m: [[f32; 3]; 27],
    k: [[f32; 3]; 3],
    l: [[f32; 3]; 9],
    z: usize,
    z1: usize,
    z2: usize,
    rng: ThreadRng,
    total_correct_guesses: usize,
}

impl Guesser {
    pub fn new() -> Self {
        let mut this = Self {
            a: 0.0,
            b: 1.0,
            c: 3.0,
            m: [[1.0; 3]; 27],
            k: [[9.0; 3]; 3],
            l: [[3.0; 3]; 9],
            z: 26,
            z1: 8,
            z2: 2,
            rng: rand::rng(),
            total_correct_guesses: 0,
        };
        this.l[0][0] = 2.0;
        this.l[4][1] = 2.0;
        this.l[8][2] = 2.0;
        this
    }

    pub fn guess_sequence(&mut self, numbers: [f32; 10]) -> Result<Vec<GuessResult>, GameError> {
        let mut retval = Vec::new();
        for num in numbers {
            if num != 0.0 && num != 1.0 && num != 2.0 {
                return Err(GameError::BadDigit);
            }
            let mut guess = 0.0;
            let mut s = 0.0;
            for j in 0..=2 {
                let s1 = self.a * self.k[self.z2][j]
                    + self.b * self.l[self.z1][j]
                    + self.c * self.m[self.z][j];
                if s > s1 {
                    continue;
                }
                if s < s1 || self.rng.random_bool(0.5) {
                    s = s1;
                    guess = j as f32;
                }
            }

            self.total_correct_guesses += (guess == num) as usize;
            retval.push(GuessResult {
                game_guess: guess,
                user_num: num,
                right_so_far: self.total_correct_guesses,
            });

            if guess == num {
                self.m[self.z][num as usize] += 1.0;
                self.l[self.z1][num as usize] += 1.0;
                self.k[self.z2][num as usize] += 1.0;
                self.z -= (self.z / 9) * 9;
                self.z = 3 * self.z + num as usize;
            }

            self.z1 = self.z - (self.z / 9) * 9;
            self.z2 = num as usize;
        }

        Ok(retval)
    }

    pub fn game_result(&self) -> GameResult {
        if self.total_correct_guesses > 10 {
            GameResult::ComputerWins
        } else if self.total_correct_guesses < 10 {
            GameResult::PlayerWins
        } else {
            GameResult::Tie
        }
    }
}

pub struct GuessResult {
    pub game_guess: f32,
    pub user_num: f32,
    pub right_so_far: usize,
}

pub enum GameError {
    BadDigit,
}

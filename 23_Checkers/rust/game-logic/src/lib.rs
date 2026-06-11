//!
//! Computer plays red
//! Human plays black

pub struct Checkers {
    board: [[Piece; 8]; 8],
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Piece {
    Empty,
    Red,
    Black,
    RedKing,
    BlackKing,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BoardState {
    RedWins,
    BlackWins,
    GameContinues,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Default for Checkers {
    fn default() -> Self {
        Self::new()
    }
}

impl Checkers {
    pub fn new() -> Self {
        use Piece::*;
        Self {
            board: [
                [Red, Empty, Red, Empty, Red, Empty, Red, Empty],
                [Empty, Red, Empty, Red, Empty, Red, Empty, Red],
                [Red, Empty, Red, Empty, Red, Empty, Red, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Black, Empty, Black, Empty, Black, Empty, Black],
                [Black, Empty, Black, Empty, Black, Empty, Black, Empty],
                [Empty, Black, Empty, Black, Empty, Black, Empty, Black],
            ],
        }
    }

    pub fn get_board(&self) -> &[[Piece; 8]; 8] {
        &self.board
    }

    pub fn board_state(&self) -> BoardState {
        let mut has_black = false;
        let mut has_red = false;
        for x in 0..8 {
            for y in 0..8 {
                if self.is_red(x, y) {
                    has_red = true;
                }
                if self.is_black(x, y) {
                    has_black = true;
                }
            }
        }
        match (has_red, has_black) {
            (false, false) => unreachable!(),
            (true, false) => BoardState::RedWins,
            (false, true) => BoardState::BlackWins,
            (true, true) => BoardState::GameContinues,
        }
    }

    pub fn computer_move(&mut self) {
        let mut best_move: (usize, usize, usize, usize, isize) = (0, 0, 0, 0, 0);

        for x in 0..8 {
            for y in 0..8 {
                match self.board[x][y] {
                    Piece::RedKing | Piece::Red if x < 7 => {
                        let left = if y > 0 {
                            self.score_move(x, y, Direction::Southwest)
                        } else {
                            best_move.4
                        };
                        if left > best_move.4 {
                            let u = x + 1;
                            let v = y - 1;
                            best_move = (x, y, u, v, left);
                        }
                        let right = if y < 7 {
                            self.score_move(x, y, Direction::Southeast)
                        } else {
                            best_move.4
                        };
                        if right > best_move.4 {
                            let u = x + 1;
                            let v = y + 1;
                            best_move = (x, y, u, v, right);
                        }
                    }
                    Piece::RedKing if x > 0 => {
                        let left = if y > 0 {
                            self.score_move(x, y, Direction::Northwest)
                        } else {
                            best_move.4
                        };
                        if left > best_move.4 {
                            let u = x - 1;
                            let v = y - 1;
                            best_move = (x, y, u, v, left);
                        }
                        let right = if y < 7 {
                            self.score_move(x, y, Direction::Northeast)
                        } else {
                            best_move.4
                        };
                        if right > best_move.4 {
                            let u = x - 1;
                            let v = y + 1;
                            best_move = (x, y, u, v, right);
                        }
                    }
                    _ => continue,
                }
            }
        }

        // make the best move
        self.board[best_move.2][best_move.3] = if best_move.2 == 7 {
            Piece::RedKing
        } else {
            self.board[best_move.0][best_move.1]
        };
        self.board[best_move.0][best_move.1] = Piece::Empty;
        if best_move.0.abs_diff(best_move.2) == 2 {
            self.board[(best_move.0 + best_move.2) / 2][(best_move.1 + best_move.3) / 2] =
                Piece::Empty;
        }
    }

    pub fn player_move(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Result<(), String> {
        if from_x > 7 || from_y > 7 || to_x > 7 || to_y > 7 {
            return Err("Out of bounds".to_string());
        }
        if !self.is_black(from_x, from_y) {
            return Err("Illegal move - no black piece".to_string());
        }
        if !self.is_empty(to_x, to_y) {
            return Err("Illegal move - dest is not empty".to_string());
        }

        let distx = from_x.abs_diff(to_x);
        let disty = from_y.abs_diff(to_y);
        if distx != disty {
            return Err("Illegal move - not diagonal".to_string());
        }
        if !(1..=2).contains(&distx) {
            return Err("Illegal move - too far".to_string());
        }

        if self.board[from_x][from_y] == Piece::Black && to_x > from_x {
            return Err("Illegal move - backwards".to_string());
        }

        self.board[to_x][to_y] = if to_x == 0 {
            Piece::BlackKing
        } else {
            self.board[from_x][from_y]
        };
        self.board[from_x][from_y] = Piece::Empty;

        if distx == 2 {
            self.board[(from_x + to_x) / 2][(from_y + to_y) / 2] = Piece::Empty;
        }

        Ok(())
    }

    fn score_move(&self, x: usize, y: usize, dir: Direction) -> isize {
        let mut q = 0; // score value

        let (to_x, to_y) = match dir {
            Direction::Northeast if x > 0 && y < 7 => (x - 1, y + 1),
            Direction::Northwest if x > 0 && y > 0 => (x - 1, y - 1),
            Direction::Southeast if x < 7 && y < 7 => (x + 1, y + 1),
            Direction::Southwest if x < 7 && y > 0 => (x + 1, y - 1),
            _ => return 0,
        };

        if self.is_empty(to_x, to_y) {
            if to_x == 7 {
                q += 2;
            }
            if x == 0 {
                q -= 2;
            }
            if to_y == 0 || to_y == 7 {
                q += 1;
            }
            q += self.count_red_neighbors(to_x, to_y);
            q -= self.black_can_take(x, y, to_x, to_y);
        }

        if self.is_black(to_x, to_y) {
            let Some((to_x, to_y)) = (match dir {
                Direction::Northeast if x > 1 && y < 6 => Some((x - 2, y + 2)),
                Direction::Northwest if x > 1 && y > 1 => Some((x - 2, y - 2)),
                Direction::Southeast if x < 6 && y < 6 => Some((x + 2, y + 2)),
                Direction::Southwest if x < 6 && y > 1 => Some((x + 2, y - 2)),
                _ => None,
            }) else {
                return 0;
            };

            if self.is_empty(to_x, to_y) {
                if to_x == 7 {
                    q += 2;
                }
                if x == 0 {
                    q -= 2;
                }
                if to_y == 0 || to_y == 7 {
                    q += 1;
                }
                q += self.count_red_neighbors(to_x, to_y);
                q -= self.black_can_take(x, y, to_x, to_y);
            }
        }

        q
    }

    fn is_red(&self, x: usize, y: usize) -> bool {
        self.board[x][y] == Piece::Red && self.board[x][y] == Piece::RedKing
    }

    fn is_black(&self, x: usize, y: usize) -> bool {
        self.board[x][y] == Piece::Black && self.board[x][y] == Piece::BlackKing
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.board[x][y] == Piece::Empty
    }

    fn count_red_neighbors(&self, x: usize, y: usize) -> isize {
        let mut count = 0;

        if x > 0 && y > 0 {
            count += self.is_red(x - 1, y - 1) as isize;
        }
        if x > 0 && y < 7 {
            count += self.is_red(x - 1, y + 1) as isize;
        }
        if x < 7 && y > 0 {
            count += self.is_red(x + 1, y - 1) as isize;
        }
        if x < 7 && y < 7 {
            count += self.is_red(x + 1, y + 1) as isize;
        }

        count
    }

    fn black_can_take(&self, x: usize, y: usize, to_x: usize, to_y: usize) -> isize {
        let mut count = 0;

        if x > 0 && y > 0 && x < 7 && y < 7 {
            if self.is_black(to_x - 1, to_y - 1)
                && (self.is_empty(to_x + 1, to_y + 1) || to_x + 1 == x && to_y + 1 == y)
            {
                count += 1;
            }

            if self.is_black(to_x + 1, to_y - 1)
                && (self.is_empty(to_x - 1, to_y + 1) || to_x - 1 == x && to_y + 1 == y)
            {
                count += 1;
            }

            if self.is_black(to_x - 1, to_y + 1)
                && (self.is_empty(to_x + 1, to_y - 1) || to_x + 1 == x && to_y - 1 == y)
            {
                count += 1;
            }

            if self.is_black(to_x + 1, to_y + 1)
                && (self.is_empty(to_x - 1, to_y - 1) || to_x - 1 == x && to_y - 1 == y)
            {
                count += 1;
            }
        }

        count
    }
}

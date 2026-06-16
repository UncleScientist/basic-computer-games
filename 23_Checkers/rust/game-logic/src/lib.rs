//!
//! Computer plays red
//! Human plays black

use std::fmt::Display;

#[derive(Debug)]
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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::Empty => ".",
                Piece::Red => "X",
                Piece::Black => "O",
                Piece::RedKing => "X*",
                Piece::BlackKing => "O*",
            }
        )
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BoardState {
    RedWins,
    BlackWins,
    GameContinues,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NextMove {
    JumpAgain,
    ComputerGoes,
}

pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct ComputerMove {
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
}

impl Display for ComputerMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FROM {} {} TO {} {}",
            self.from_col,
            7 - self.from_row,
            self.to_col,
            7 - self.to_row
        )
    }
}

impl ComputerMove {
    fn default() -> Self {
        Self {
            from_row: 0,
            from_col: 0,
            to_row: 0,
            to_col: 0,
        }
    }

    fn new(from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> Self {
        Self {
            from_row,
            from_col,
            to_row,
            to_col,
        }
    }

    fn is_jump(&self) -> bool {
        self.from_row.abs_diff(self.to_row) == 2
    }

    fn between_row(&self) -> usize {
        (self.from_row + self.to_row) / 2
    }

    fn between_col(&self) -> usize {
        (self.from_col + self.to_col) / 2
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Northeast,
    Northwest,
    Southeast,
    Southwest,
    JumpNortheast,
    JumpNorthwest,
    JumpSoutheast,
    JumpSouthwest,
}
impl Direction {
    fn is_jump(&self) -> bool {
        matches!(
            self,
            Direction::JumpNortheast
                | Direction::JumpNorthwest
                | Direction::JumpSoutheast
                | Direction::JumpSouthwest,
        )
    }
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
                [Empty, Red, Empty, Red, Empty, Red, Empty, Red],
                [Red, Empty, Red, Empty, Red, Empty, Red, Empty],
                [Empty, Red, Empty, Red, Empty, Red, Empty, Red],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Black, Empty, Black, Empty, Black, Empty, Black, Empty],
                [Empty, Black, Empty, Black, Empty, Black, Empty, Black],
                [Black, Empty, Black, Empty, Black, Empty, Black, Empty],
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

    fn eval_at_position(&self, row: usize, col: usize) -> (ComputerMove, isize) {
        let mut best_move: (ComputerMove, isize) = (ComputerMove::default(), 0);

        // Single move
        if self.is_red(row, col) && row < 7 {
            let left = if col > 0 {
                self.score_move(row, col, Direction::Southwest)
            } else {
                best_move.1
            };
            if left > best_move.1 {
                let u = row + 1;
                let v = col - 1;
                best_move = (ComputerMove::new(row, col, u, v), left);
            }
            let right = if col < 7 {
                self.score_move(row, col, Direction::Southeast)
            } else {
                best_move.1
            };
            if right > best_move.1 {
                let u = row + 1;
                let v = col + 1;
                best_move = (ComputerMove::new(row, col, u, v), right);
            }
        }

        // Jump move
        if self.is_red(row, col) && row < 6 {
            let left = if col > 1 && self.is_black(row + 1, col - 1) {
                self.score_move(row, col, Direction::JumpSouthwest)
            } else {
                best_move.1
            };
            if left > best_move.1 {
                let u = row + 2;
                let v = col - 2;
                best_move = (ComputerMove::new(row, col, u, v), left);
            }
            let right = if col < 6 && self.is_black(row + 1, col + 1) {
                self.score_move(row, col, Direction::JumpSoutheast)
            } else {
                best_move.1
            };
            if right > best_move.1 {
                let u = row + 2;
                let v = col + 2;
                best_move = (ComputerMove::new(row, col, u, v), right);
            }
        }

        // Single backwards move
        if self.board[row][col] == Piece::RedKing && row > 0 {
            let left = if col > 0 {
                self.score_move(row, col, Direction::Northwest)
            } else {
                best_move.1
            };
            if left > best_move.1 {
                let u = row - 1;
                let v = col - 1;
                best_move = (ComputerMove::new(row, col, u, v), left);
            }
            let right = if col < 7 {
                self.score_move(row, col, Direction::Northeast)
            } else {
                best_move.1
            };
            if right > best_move.1 {
                let u = row - 1;
                let v = col + 1;
                best_move = (ComputerMove::new(row, col, u, v), right);
            }
        }

        // Jump backwards move
        if self.board[row][col] == Piece::RedKing && row > 1 {
            let left = if col > 1 && self.is_black(row - 1, col - 1) {
                self.score_move(row, col, Direction::JumpNorthwest)
            } else {
                best_move.1
            };
            if left > best_move.1 {
                let u = row - 2;
                let v = col - 2;
                best_move = (ComputerMove::new(row, col, u, v), left);
            }
            let right = if col < 6 && self.is_black(row - 1, col + 1) {
                self.score_move(row, col, Direction::JumpNortheast)
            } else {
                best_move.1
            };
            if right > best_move.1 {
                let u = row - 2;
                let v = col + 2;
                best_move = (ComputerMove::new(row, col, u, v), right);
            }
        }

        best_move
    }

    pub fn computer_move(&mut self) -> Vec<ComputerMove> {
        let mut result = Vec::new();
        let mut best_move: (ComputerMove, isize) = (ComputerMove::default(), 0);

        for row in 0..8 {
            for col in 0..8 {
                let move_value = self.eval_at_position(row, col);
                if move_value.1 > best_move.1 {
                    best_move = move_value
                }
            }
        }

        result.push(best_move.0.clone());

        // make the best move
        self.board[best_move.0.to_row][best_move.0.to_col] = if best_move.0.to_row == 7 {
            Piece::RedKing
        } else {
            self.board[best_move.0.from_row][best_move.0.from_col]
        };
        self.board[best_move.0.from_row][best_move.0.from_col] = Piece::Empty;
        if best_move.0.is_jump() {
            self.board[best_move.0.between_row()][best_move.0.between_col()] = Piece::Empty;

            // Check for multiple jumps
            loop {
                best_move = self.eval_at_position(best_move.0.to_row, best_move.0.to_col);
                if best_move.0.is_jump() {
                    result.push(best_move.0.clone());
                    self.board[best_move.0.to_row][best_move.0.to_col] = if best_move.0.to_row == 7
                    {
                        Piece::RedKing
                    } else {
                        self.board[best_move.0.from_row][best_move.0.from_col]
                    };
                    self.board[best_move.0.from_row][best_move.0.from_col] = Piece::Empty;
                    self.board[best_move.0.between_row()][best_move.0.between_col()] = Piece::Empty;
                } else {
                    break;
                }
            }
        }

        result
    }

    pub fn player_move(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> Result<NextMove, String> {
        if from_row > 7 || from_col > 7 || to_row > 7 || to_col > 7 {
            return Err("Out of bounds".to_string());
        }
        let from_x = 7 - from_row;
        let to_x = 7 - to_row;

        if !self.is_black(from_x, from_col) {
            return Err("Illegal move - no black piece".to_string());
        }
        if !self.is_empty(to_x, to_col) {
            println!("{:?}", self.board);
            return Err("Illegal move - dest is not empty".to_string());
        }

        let distx = from_x.abs_diff(to_x);
        let disty = from_col.abs_diff(to_col);
        if distx != disty {
            return Err("Illegal move - not diagonal".to_string());
        }
        if !(1..=2).contains(&distx) {
            return Err("Illegal move - too far".to_string());
        }

        if self.board[from_x][from_col] == Piece::Black && to_x > from_x {
            return Err("Illegal move - backwards".to_string());
        }

        self.board[to_x][to_col] = if to_x == 0 {
            Piece::BlackKing
        } else {
            self.board[from_x][from_col]
        };
        self.board[from_x][from_col] = Piece::Empty;

        if distx == 2 {
            self.board[(from_x + to_x) / 2][(from_col + to_col) / 2] = Piece::Empty;
            Ok(NextMove::JumpAgain)
        } else {
            Ok(NextMove::ComputerGoes)
        }
    }

    fn score_move(&self, row: usize, col: usize, dir: Direction) -> isize {
        let mut q = 0; // score value

        let (to_row, to_col) = match dir {
            Direction::Northeast if row > 0 && col < 7 => (row - 1, col + 1),
            Direction::Northwest if row > 0 && col > 0 => (row - 1, col - 1),
            Direction::Southeast if row < 7 && col < 7 => (row + 1, col + 1),
            Direction::Southwest if row < 7 && col > 0 => (row + 1, col - 1),
            Direction::JumpNortheast if row > 1 && col < 6 => (row - 2, col + 2),
            Direction::JumpNorthwest if row > 1 && col > 1 => (row - 2, col - 2),
            Direction::JumpSoutheast if row < 6 && col < 6 => (row + 2, col + 2),
            Direction::JumpSouthwest if row < 6 && col > 1 => (row + 2, col - 2),
            _ => return 0,
        };

        if self.is_empty(to_row, to_col) {
            if dir.is_jump() {
                q += 5;
            }

            if to_row == 7 {
                q += 2;
            }
            if row == 0 {
                q -= 2;
            }
            if to_col == 0 || to_col == 7 {
                q += 1;
            }
            q += self.count_red_neighbors(to_row, to_col);
            q -= self.black_can_take(row, col, to_row, to_col);
        }

        q
    }

    fn is_red(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == Piece::Red || self.board[row][col] == Piece::RedKing
    }

    fn is_black(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == Piece::Black || self.board[row][col] == Piece::BlackKing
    }

    fn is_empty(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == Piece::Empty
    }

    fn count_red_neighbors(&self, row: usize, col: usize) -> isize {
        let mut count = 0;

        if row > 0 && col > 0 {
            count += self.is_red(row - 1, col - 1) as isize;
        }
        if row > 0 && col < 7 {
            count += self.is_red(row - 1, col + 1) as isize;
        }
        if row < 7 && col > 0 {
            count += self.is_red(row + 1, col - 1) as isize;
        }
        if row < 7 && col < 7 {
            count += self.is_red(row + 1, col + 1) as isize;
        }

        count
    }

    fn black_can_take(&self, row: usize, col: usize, to_row: usize, to_col: usize) -> isize {
        let mut count = 0;

        if to_row > 0 && to_col > 0 && to_row < 7 && to_col < 7 {
            if self.is_black(to_row - 1, to_col - 1)
                && (self.is_empty(to_row + 1, to_col + 1) || to_row + 1 == row && to_col + 1 == col)
            {
                count += 1;
            }

            if self.is_black(to_row + 1, to_col - 1)
                && (self.is_empty(to_row - 1, to_col + 1) || to_row - 1 == row && to_col + 1 == col)
            {
                count += 1;
            }

            if self.is_black(to_row - 1, to_col + 1)
                && (self.is_empty(to_row + 1, to_col - 1) || to_row + 1 == row && to_col - 1 == col)
            {
                count += 1;
            }

            if self.is_black(to_row + 1, to_col + 1)
                && (self.is_empty(to_row - 1, to_col - 1) || to_row - 1 == row && to_col - 1 == col)
            {
                count += 1;
            }
        }

        count
    }
}

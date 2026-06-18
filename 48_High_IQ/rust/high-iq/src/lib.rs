use std::fmt::Display;

pub struct HighIq {
    grid: [[isize; 10]; 10],
    board: [Piece; 71],
}

pub enum MoveError {
    IllegalMove,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    MovesRemaining,
    GameOver(usize),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Hole, // A hole in this position
    Peg,  // A peg in this position
    Void, // This position is outside the board
}

impl Default for HighIq {
    fn default() -> Self {
        Self::new()
    }
}

impl HighIq {
    #[allow(clippy::needless_range_loop)]
    pub fn new() -> Self {
        let mut grid = [[0isize; 10]; 10];
        for row in 1..=9 {
            for col in 1..=9 {
                if !(4..=6).contains(&row) && !(4..=6).contains(&col) {
                    grid[row][col] = -5;
                } else {
                    if row == 1 || col == 1 || row == 9 || col == 9 {
                        grid[row][col] = -5;
                    } else {
                        grid[row][col] = 5;
                    }
                }
            }
        }

        // 79 DATA 13,14,15,22,23,24,29,30,31,32,33,34,35,38,39,40,41
        // 81 DATA 42,43,44,47,48,49,50,51,52,53,58,59,60,67,68,69
        let coords = [
            13, 14, 15, 22, 23, 24, 29, 30, 31, 32, 33, 34, 35, 38, 39, 40, 41, 42, 43, 44, 47, 48,
            49, 50, 51, 52, 53, 58, 59, 60, 67, 68, 69,
        ];

        let mut board = [Piece::Void; 71];
        for c in coords {
            board[c] = Piece::Peg;
        }
        board[41] = Piece::Hole;

        Self { grid, board }
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> Result<(), MoveError> {
        // Lines 110-156
        if self.board[from] != Piece::Peg || self.board[to] != Piece::Hole || from == to {
            return Err(MoveError::IllegalMove);
        }

        // Lines 160-180
        if !(from + 2).is_multiple_of(2) || from.abs_diff(to) != 2 || from.abs_diff(to) != 18 {
            return Err(MoveError::IllegalMove);
        }

        let mut c = 1;
        'search: for x in 1..=9 {
            for y in 1..=9 {
                if c == from {
                    if c + 2 == to {
                        // Jump right
                        if self.grid[x][y + 1] != 0 {
                            return Err(MoveError::IllegalMove);
                        }
                        // Line 1050-1070
                        self.grid[x][y + 2] = 5;
                        self.grid[x][y + 1] = 0;
                        self.board[c + 1] = Piece::Hole;
                    } else if c + 18 == to {
                        // Jump down
                        if self.grid[x + 1][y] != 0 {
                            return Err(MoveError::IllegalMove);
                        }

                        // Line 1090-1120
                        self.grid[x + 2][y] = 5;
                        self.grid[x + 1][y] = 0;
                        self.board[c + 9] = Piece::Hole;
                    } else if c - 2 == to {
                        // Jump Left
                        if self.grid[x][y - 1] != 0 {
                            return Err(MoveError::IllegalMove);
                        }

                        // Line 1140-1160
                        self.grid[x][y - 2] = 5;
                        self.grid[x][y - 1] = 0;
                        self.board[c - 1] = Piece::Hole;
                    } else if c - 18 == to {
                        // Jump Up
                        if self.grid[x - 1][y] != 0 {
                            return Err(MoveError::IllegalMove);
                        }

                        // Line 1180
                        self.grid[x - 2][y] = 5;
                        self.grid[x - 1][y] = 0;
                        self.board[c - 9] = Piece::Hole;
                    } else {
                        return Err(MoveError::IllegalMove);
                    }

                    // Line 1200-1210
                    self.board[from] = Piece::Hole;
                    self.board[to] = Piece::Peg;
                    self.grid[x][y] = 0;
                    break 'search;
                }
                c += 1;
            }
        }

        Ok(())
    }

    pub fn check_board(&self) -> GameState {
        let mut peg_count = 0; // Note: variable "F" in the basic code

        for r in 2..=8 {
            for c in 2..=8 {
                if self.grid[r][c] != 5 {
                    continue;
                }
                peg_count += 1;
                for a in r - 1..=r + 1 {
                    let mut t = 0;
                    for b in c - 1..=c + 1 {
                        t += self.grid[a][b];
                    }
                    if t != 10 {
                        continue;
                    }
                    if self.grid[a][c] != 0 {
                        return GameState::MovesRemaining;
                    }
                }

                for x in c - 1..=c + 1 {
                    let mut t = 0;
                    for y in r - 1..=r + 1 {
                        t += self.grid[y][x];
                    }
                    if t != 10 {
                        continue;
                    }
                    if self.grid[r][x] != 0 {
                        return GameState::MovesRemaining;
                    }
                }
            }
        }

        GameState::GameOver(peg_count)
    }
}

impl Display for HighIq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 1..=9 {
            for y in 1..=9 {
                if x == 1 || x == 9 || y == 1 || y == 9 {
                    write!(f, "  ")?;
                    continue;
                }
                if (4..=6).contains(&x) || (4..=6).contains(&y) {
                    if self.grid[x][y] != 5 {
                        write!(f, " O")?;
                    } else {
                        write!(f, " !")?;
                    }
                } else {
                    write!(f, "  ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

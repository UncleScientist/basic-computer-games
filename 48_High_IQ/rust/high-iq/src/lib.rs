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
    // 79 DATA 13,14,15,22,23,24,29,30,31,32,33,34,35,38,39,40,41
    // 81 DATA 42,43,44,47,48,49,50,51,52,53,58,59,60,67,68,69
    const COORDS: [usize; 33] = [
        13, 14, 15, 22, 23, 24, 29, 30, 31, 32, 33, 34, 35, 38, 39, 40, 41, 42, 43, 44, 47, 48, 49,
        50, 51, 52, 53, 58, 59, 60, 67, 68, 69,
    ];

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
        grid[5][5] = 0;

        let mut board = [Piece::Void; 71];
        for c in &Self::COORDS {
            board[*c] = Piece::Peg;
        }
        board[41] = Piece::Hole;

        Self { grid, board }
    }

    // Translates x 0..=7 and y 0..=7 into an index in the self.board[] array
    pub fn get_board_position(&self, x: usize, y: usize) -> Option<usize> {
        let pos = 11 + x + 9 * y;
        if self.board[pos] == Piece::Void {
            None
        } else {
            Some(pos)
        }
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> Result<(), MoveError> {
        // Note: This check is not part of the original code; just something I added
        // for my own sanity
        if from == 0 || to == 0 || from >= self.board.len() || to >= self.board.len() {
            return Err(MoveError::IllegalMove);
        }

        // Lines 110-156
        if self.board[from] != Piece::Peg || self.board[to] != Piece::Hole || from == to {
            return Err(MoveError::IllegalMove);
        }

        // Lines 160-180
        if !(from + to).is_multiple_of(2) || (from.abs_diff(to) != 2 && from.abs_diff(to) != 18) {
            return Err(MoveError::IllegalMove);
        }

        let mut c = 1;
        'search: for x in 1..=9 {
            for y in 1..=9 {
                if c == from {
                    if c + 2 == to {
                        // Jump right
                        if self.grid[x][y + 1] == 0 {
                            return Err(MoveError::IllegalMove);
                        }
                        // Line 1050-1070
                        self.grid[x][y + 2] = 5;
                        self.grid[x][y + 1] = 0;
                        self.board[c + 1] = Piece::Hole;
                    } else if c + 18 == to {
                        // Jump down
                        if self.grid[x + 1][y] == 0 {
                            return Err(MoveError::IllegalMove);
                        }

                        // Line 1090-1120
                        self.grid[x + 2][y] = 5;
                        self.grid[x + 1][y] = 0;
                        self.board[c + 9] = Piece::Hole;
                    } else if c - 2 == to {
                        // Jump Left
                        if self.grid[x][y - 1] == 0 {
                            return Err(MoveError::IllegalMove);
                        }

                        // Line 1140-1160
                        self.grid[x][y - 2] = 5;
                        self.grid[x][y - 1] = 0;
                        self.board[c - 1] = Piece::Hole;
                    } else if c - 18 == to {
                        // Jump Up
                        if self.grid[x - 1][y] == 0 {
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

    pub fn has_peg_at(&self, from: usize) -> bool {
        if from == 0 || from >= self.board.len() {
            false
        } else {
            self.board[from] == Piece::Peg
        }
    }

    pub fn get_board(&self) -> &[[isize; 10]; 10] {
        &self.grid
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

#[cfg(test)]
mod test {
    use super::*;

    const MOVELIST: [[usize; 2]; 26] = [
        [23, 41],
        [50, 32],
        [52, 50],
        [49, 51],
        [68, 50],
        [51, 49],
        [34, 52],
        [53, 51],
        [35, 53],
        [39, 41],
        [32, 50],
        [50, 52],
        [53, 51],
        [58, 40],
        [31, 49],
        [48, 50],
        [29, 31],
        [22, 40],
        [50, 52],
        [69, 51],
        [52, 50],
        [47, 29],
        [33, 51],
        [50, 52],
        [15, 33],
        [13, 15],
    ];

    #[test]
    fn test_jump_up() {
        let mut game = HighIq::new();
        assert!(game.make_move(59, 41).is_ok());
    }

    #[test]
    fn test_jump_down() {
        let mut game = HighIq::new();
        assert!(game.make_move(23, 41).is_ok());
    }
    #[test]
    fn test_jump_left() {
        let mut game = HighIq::new();
        assert!(game.make_move(43, 41).is_ok());
    }
    #[test]
    fn test_jump_right() {
        let mut game = HighIq::new();
        assert!(game.make_move(39, 41).is_ok());
    }

    #[test]
    fn test_simple_game() {
        let mut game = HighIq::new();

        for entry in &MOVELIST {
            assert!(game.make_move(entry[0], entry[1]).is_ok());
        }
        assert_eq!(game.check_board(), GameState::GameOver(6));
    }

    #[test]
    fn test_coord_conversion() {
        let game = HighIq::new();

        assert_eq!(None, game.get_board_position(0, 0));
        assert_eq!(Some(22), game.get_board_position(2, 1));
        assert_eq!(Some(60), game.get_board_position(4, 5));
        assert_eq!(None, game.get_board_position(5, 6));
    }
}

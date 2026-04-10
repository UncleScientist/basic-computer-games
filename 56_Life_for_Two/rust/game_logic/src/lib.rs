use std::fmt::Display;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    #[default]
    Empty,
    Player1,
    Player2,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => " ",
                Self::Player1 => "*",
                Self::Player2 => "#",
            }
        )
    }
}

#[derive(Default)]
pub struct Board {
    grid: [[Piece; 5]; 5],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                write!(f, " {}", self.grid[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    // returns true if piece was successfully placed
    pub fn place_piece(&mut self, player: Piece, x: usize, y: usize) -> bool {
        if self.grid[y - 1][x - 1] != Piece::Empty {
            return false;
        }
        self.grid[y - 1][x - 1] = player;
        true
    }

    pub fn get_grid(&self) -> &[[Piece; 5]; 5] {
        &self.grid
    }

    // returns count of player 1 and player 2 pieces
    pub fn step(&mut self) -> (usize, usize) {
        let mut next_grid = [[Piece::Empty; 5]; 5];
        let mut total_p1 = 0;
        let mut total_p2 = 0;

        for i in 0i32..5 {
            for j in 0i32..5 {
                let mut p1_count = 0;
                let mut p2_count = 0;

                for di in -1..2 {
                    for dj in -1..2 {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let pos_i = i + di;
                        let pos_j = j + dj;

                        if !(0..5).contains(&pos_i) || !(0..5).contains(&pos_j) {
                            continue;
                        }

                        match self.grid[pos_i as usize][pos_j as usize] {
                            Piece::Empty => {}
                            Piece::Player1 => p1_count += 1,
                            Piece::Player2 => p2_count += 1,
                        }
                    }
                }

                let total_count = p1_count + p2_count;

                // Following logic derived from:
                // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
                match self.grid[i as usize][j as usize] {
                    // 1. Any live cell with fewer than two live neighbors dies
                    _ if total_count < 2 => {
                        next_grid[i as usize][j as usize] = Piece::Empty;
                    }

                    // 3. Any live cell with more than three live neighbors dies
                    _ if total_count > 3 => {
                        next_grid[i as usize][j as usize] = Piece::Empty;
                    }

                    // 4. Any dead cell with exactly three neighbors becomes live
                    Piece::Empty if total_count == 3 => {
                        if p1_count > p2_count {
                            next_grid[i as usize][j as usize] = Piece::Player1;
                        } else {
                            next_grid[i as usize][j as usize] = Piece::Player2;
                        }
                    }

                    // 2. Any live cell with two or three neighbors lives on
                    _ => {
                        next_grid[i as usize][j as usize] = self.grid[i as usize][j as usize];
                    }
                }

                match next_grid[i as usize][j as usize] {
                    Piece::Empty => {}
                    Piece::Player1 => total_p1 += 1,
                    Piece::Player2 => total_p2 += 1,
                }
            }
        }

        self.grid = next_grid;

        (total_p1, total_p2)
    }

    pub fn clear_piece(&mut self, x: usize, y: usize) {
        self.grid[y - 1][x - 1] = Piece::Empty;
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y - 1][x - 1] == Piece::Empty
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_step() {
        let mut board = Board::new();

        board.grid[0][0] = Piece::Player1;
        board.grid[1][0] = Piece::Player1;
        board.grid[0][1] = Piece::Player1;

        board.grid[2][2] = Piece::Player2;
        board.grid[3][2] = Piece::Player2;
        board.grid[4][2] = Piece::Player2;

        println!("before:\n{board}");
        let (p1, p2) = board.step();
        println!("after:\n{board}");
        assert_eq!(3, p1);
        assert_eq!(4, p2);

        let grid = [
            [
                Piece::Player1,
                Piece::Player1,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Player1,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Empty,
                Piece::Player2,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Empty,
                Piece::Player2,
                Piece::Player2,
                Piece::Player2,
                Piece::Empty,
            ],
            [
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
        ];
        assert_eq!(grid, board.grid);
    }
}

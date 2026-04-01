use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Empty,
    Occupied,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::Empty => " 0",
                Piece::Occupied => " 1",
            }
        )
    }
}

#[derive(Debug)]
pub struct Board {
    grid: [Piece; 64],
    moves: usize,
}

impl Board {
    pub fn new() -> Self {
        // Lines 80-64
        let mut grid = [Piece::Occupied; 64];

        // Line 86-94
        for j in (18..=42).step_by(8) {
            for i in grid.iter_mut().skip(j).take(4) {
                *i = Piece::Empty;
            }
        }

        Self { grid, moves: 0 }
    }

    pub fn is_occupied(&self, pos: usize) -> bool {
        self.grid[pos - 1] == Piece::Occupied
    }

    pub fn get_board(&self) -> &[Piece; 64] {
        &self.grid
    }

    pub fn legal_move(&self, from: usize, to: usize) -> bool {
        if from > 64 || !(1..=64).contains(&to) {
            return false;
        }

        // Lines 120-150
        let f1 = (from - 1) / 8;
        let f2 = from - 8 * f1;
        let t1 = (to - 1) / 8;
        let t2 = to - 8 * t1;

        // Lines 160-220
        !(f1 > 7
            || t1 > 7
            || f2 > 8
            || t2 > 8
            || f1.abs_diff(t1) != 2
            || f2.abs_diff(t2) != 2
            || self.grid[(from + to) / 2 - 1] == Piece::Empty
            || self.grid[from - 1] == Piece::Empty
            || self.grid[to - 1] == Piece::Occupied)
    }

    pub fn make_move(&mut self, from: usize, to: usize) {
        self.grid[to - 1] = Piece::Occupied;
        self.grid[from - 1] = Piece::Empty;
        self.grid[(from + to) / 2 - 1] = Piece::Empty;
        self.moves += 1;
    }

    pub fn get_moves(&self) -> usize {
        self.moves
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    // Lines 310-410
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in (0..=56).step_by(8) {
            for i in j..=j + 7 {
                write!(f, "{}", self.grid[i])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

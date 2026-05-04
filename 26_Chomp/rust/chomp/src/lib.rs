pub struct Chomp {
    rows: usize,
    grid: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub enum ChompResult {
    Invalid,
    Safe,
    Dead,
}

impl Chomp {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            grid: vec![cols; rows],
        }
    }

    pub fn game_grid(&self) -> &[usize] {
        &self.grid
    }

    pub fn chomp(&mut self, row: usize, col: usize) -> ChompResult {
        if row >= self.rows {
            return ChompResult::Invalid;
        }

        if col >= self.grid[row] {
            return ChompResult::Invalid;
        }

        if row == 0 && col == 0 {
            return ChompResult::Dead;
        }

        for r in row..self.rows {
            self.grid[r] = self.grid[r].min(col);
        }

        ChompResult::Safe
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_gameboard() {
        let chomp = Chomp::new(5, 7);
        assert_eq!(vec![7, 7, 7, 7, 7], chomp.grid);
    }

    #[test]
    fn test_chomp_safe() {
        let mut game = Chomp::new(5, 7);

        game.chomp(3, 3);

        assert_eq!(vec![7, 7, 7, 3, 3], game.grid);
    }

    #[test]
    fn test_chomp_empty() {
        let mut game = Chomp::new(5, 7);
        let result = game.chomp(9, 9);
        assert_eq!(ChompResult::Invalid, result);
    }

    #[test]
    fn test_chomp_poison() {
        let mut game = Chomp::new(5, 7);
        let result = game.chomp(0, 0);
        assert_eq!(ChompResult::Dead, result);
    }
}

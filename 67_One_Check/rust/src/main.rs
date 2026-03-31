use std::fmt::Display;
use std::io::{BufRead, Write};

fn main() {
    println!("{:>30}One Check", "");
    println!("{:>15}Creative Computing  Morristown, New Jersey\n\n\n", "");

    println!("Solitar Checker Puzzle by David Ahl\n");

    println!("48 checkers are placed on the 2 outside spaces of a");
    println!("standard 64-square checkerboard. The object is to");
    println!("remove as many checkers as possible by diagonal jumps");
    println!("(as in standard checkers). Use the numbered board to");
    println!("indicate the square you wish to jump from and to. On");
    println!("the board printed out on each turn '1' indicates a");
    println!("checker and '0' and empty square. When you have no");
    println!("possible jumps remaining, input a '0' in response to");
    println!("question 'jump from?'\n");

    println!("Here is the numerical board\n");

    for j in (1..=57).step_by(8) {
        println!(
            " {:3} {:3} {:3} {:3} {:3} {:3} {:3} {:3}",
            j,
            j + 1,
            j + 2,
            j + 3,
            j + 4,
            j + 5,
            j + 6,
            j + 7
        );
    }

    println!("\nAnd here is the opening position of the checkers.\n");

    let mut board = Board::new();

    loop {
        println!("{board}");

        let (from, to) = loop {
            // Lines 100-112
            let from = get_board_position("Jump from");
            if from == 0 {
                break (0, 0);
            }

            let to = get_board_position("To");
            if board.legal_move(from, to) {
                break (from, to);
            }

            // Line 230-240
            println!("Illegal move. Try again...");
        };

        println!();

        // Line 105
        if from == 0 {
            break;
        }

        // Lines 250-290
        board.make_move(from, to);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
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
struct Board {
    grid: [Piece; 64],
    moves: usize,
}

impl Board {
    fn new() -> Self {
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

    fn legal_move(&self, from: usize, to: usize) -> bool {
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
            || self.grid[from - 1] == Piece::Empty
            || self.grid[to - 1] == Piece::Occupied)
    }

    fn make_move(&mut self, from: usize, to: usize) {
        self.grid[to - 1] = Piece::Occupied;
        self.grid[from - 1] = Piece::Empty;
        self.grid[(from + to) / 2 - 1] = Piece::Empty;
        self.moves += 1;
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

fn get_board_position<S: AsRef<str>>(prompt: S) -> usize {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(result) = buffer.trim().parse::<usize>() {
            return result;
        } else {
            println!("?REENTER");
        }
    }
}

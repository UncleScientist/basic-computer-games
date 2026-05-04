use std::io::{BufRead, Write};

use chomp::{Chomp, ChompResult};

fn main() {
    println!("{:<33}CHOMP", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY", "");

    // The game of Chomp *** Copyright PCC 1973 ***

    println!("\nTHIS IS THE GAME OF CHOMP (SCIENTIFIC AMERICAN, JAN 1973)");
    let response = prompt_for_input("DO YOU WANT THE RULES, (1=YES, 0=NO!)");
    if response != 0 {
        println!("CHOMP IS FOR 1 OR MORE PLAYERS (HUMANS ONLY).\n");
        println!("HERE'S HOW A BOARD LOOKS (THIS ONE IS 5 BY 7):\n");

        let sample = Chomp::new(5, 7);
        print_board(&sample);

        println!("\nTHE BOARD IS A BIG COOKIE - R ROWS HIGH AND C COLUMNS");
        println!("WIDE. YOU INPUT R AND C AT THE START. IN THE UPPER LEFT");
        println!("CORNER OF THE COOKIE IS A POISON SQUARE (P). THE ONE WHO");
        println!("CHOMPS THE POISON SQUARE LOSES. TO TAKE A CHOMP, TYPE THE");
        println!("ROW AND COLUMN OF ONE OF THE SQUARES ON THE COOKIE.");
        println!("ALL OF THE SQUARES BELOW AND TO THE RIGHT OF THAT SQUARE");
        println!("(INCLUDING THAT SQUARE, TOO) DISAPPEAR -- CHOMP!!");
        println!("NO FAIR CHOMPING SQUARES THAT HAVE ALREADY BEEN CHOMPED,");
        println!("OR THAT ARE OUTSIDE THE ORIGINAL DIMENSIONS OF THE COOKIE.\n");
        println!("HERE WE GO...");
    }

    let player_count = prompt_for_input("HOW MANY PLAYERS");
    let rows = loop {
        let rows = prompt_for_input("HOW MANY ROWS");
        if (1..=9).contains(&rows) {
            break rows;
        }
        print!("TOO MANY ROWS (9 IS MAXIUMUM). NOW, ");
    };

    let cols = loop {
        let cols = prompt_for_input("HOW MANY COLS");
        if (1..=9).contains(&cols) {
            break cols;
        }
        print!("TOO MANY COLS (9 IS MAXIUMUM). NOW, ");
    };

    let mut game = Chomp::new(rows, cols);
    let mut cur_player = 0;
    loop {
        print_board(&game);

        println!("\nPLAYER {}", cur_player + 1);
        let (row, col) = prompt_for_coords("COORDINATES OF CHOMP (ROW, COLUMN)");
        match game.chomp(row - 1, col - 1) {
            ChompResult::Invalid => {
                println!("NO FAIR. YOU'RE TRYING TO CHOMP ON EMPTY SPACE");
                continue;
            }
            ChompResult::Safe => {}
            ChompResult::Dead => {
                println!("YOU LOSE, PLAYER {}", cur_player + 1);
                break;
            }
        }

        cur_player = (cur_player + 1) % player_count;
    }
}

fn prompt_for_coords<S: AsRef<str>>(prompt: S) -> (usize, usize) {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);

        if let Some((row, col)) = buffer.trim().to_string().split_once(',') {
            let Ok(row) = row.trim().parse::<usize>() else {
                println!("?REENTER");
                continue;
            };
            let Ok(col) = col.trim().parse::<usize>() else {
                println!("?REENTER");
                continue;
            };
            return (row, col);
        }
        println!("?REENTER");
    }
}

fn print_board(board: &Chomp) {
    let grid = board.game_grid();

    println!("       1 2 3 4 5 6 7 8 9");
    for r in 0..9 {
        let cols = grid.get(r).unwrap_or(&0);
        print!("{:<7}", r + 1);
        for c in 0..*cols {
            if r == 0 && c == 0 {
                print!("P ");
            } else {
                print!("* ");
            }
        }
        println!();
    }
}

fn prompt_for_input<S: AsRef<str>>(prompt: S) -> usize {
    loop {
        print!("{}? ", prompt.as_ref());
        let mut lock = std::io::stdout().lock();
        let _ = lock.flush();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        if let Ok(num) = buffer.trim().to_string().parse::<usize>() {
            return num;
        }
        println!("?REENTER");
    }
}

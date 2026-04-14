use std::{ffi::CString, os::raw::c_char};

unsafe extern "C" {
    fn getpass(prompt: *const c_char) -> *mut c_char;
}

use life_for_two::{Board, Piece};

fn main() {
    println!("{:<33}LIFE2", "");
    println!("{:<15}CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n", "");

    let mut board = Board::new();
    println!("{:<10}U.B. LIFE GAME", "");
    get_start(&mut board, Piece::Player1);
    get_start(&mut board, Piece::Player2);
    display_board(&board);
    loop {
        let (p1count, p2count) = board.step();
        println!();
        display_board(&board);

        if p1count == 0 && p2count == 0 {
            println!("A DRAW");
            break;
        }

        if p1count == 0 {
            println!("PLAYER 2 IS THE WINNER");
            break;
        }

        if p2count == 0 {
            println!("PLAYER 1 IS THE WINNER");
            break;
        }

        print!("PLAYER 1 ");
        let (p1x, p1y) = get_single_input(&board);
        print!("PLAYER 2 ");
        let (p2x, p2y) = get_single_input(&board);

        if p1x == p2x && p1y == p2y {
            println!("SAME COORD.  SET TO 0");
            board.clear_piece(p1x, p1y);
        } else {
            board.place_piece(Piece::Player1, p1x, p1y);
            board.place_piece(Piece::Player2, p2x, p2y);
        }
    }
}

fn display_board(board: &Board) {
    let grid = board.get_grid();

    println!("0  1  2  3  4  5  0");
    for (r, row) in grid.iter().enumerate() {
        print!("{}  ", r + 1);
        for piece in row.iter() {
            print!("{piece}  ");
        }
        println!("{}", r + 1);
    }
    println!("0  1  2  3  4  5  0");
}

fn get_start(board: &mut Board, player: Piece) {
    println!(
        "PLAYER {} - 3 LIVE PIECES",
        match player {
            life_for_two::Piece::Empty => unreachable!(),
            life_for_two::Piece::Player1 => "1",
            life_for_two::Piece::Player2 => "2",
        }
    );
    for _ in 0..3 {
        let (x, y) = get_single_input(board);
        board.place_piece(player, x, y);
    }
}

fn get_single_input(board: &Board) -> (usize, usize) {
    loop {
        println!("X,Y");
        let (x, y) = get_input();
        if let Ok(x) = x.parse::<usize>()
            && let Ok(y) = y.parse::<usize>()
            && (1..=5).contains(&x)
            && (1..=5).contains(&y)
            && board.is_empty(x, y)
        {
            return (x, y);
        }
        println!("ILLEGAL COORDS. RETYPE");
    }
}

fn get_input() -> (String, String) {
    loop {
        let pass = unsafe {
            let prompt = CString::new("? ").unwrap();
            let p = getpass(prompt.into_raw());
            CString::from_raw(p).into_string()
        };

        if let Ok(buffer) = pass
            && let Some((x, y)) = buffer.split_once(',')
        {
            return (x.trim().to_string(), y.trim().to_string());
        }
        println!("?REENTER");
    }
}

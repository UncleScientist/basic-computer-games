use life_for_two::{Board, Piece};
use macroquad::prelude::*;

#[macroquad::main("Life for Two")]
async fn main() {
    let mut board = Board::new();

    loop {
        clear_background(BLACK);
        draw_board(&board);

        next_frame().await
    }
}

fn draw_board(board: &Board) {}

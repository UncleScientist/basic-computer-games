use macroquad::prelude::*;

#[macroquad::main("One Check")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}

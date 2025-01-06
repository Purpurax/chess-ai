mod board;
mod piece;

use crate::board::Board;

fn main() {
    let board: Board = Board::new();

    println!("{}", board.to_string());
}

use crate::core::board::Board;

use super::position::Position;

pub struct Game {
    pub board: Board,
    pub player_turn: bool, // false is blacks turn
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player_turn: true
        }
    }

    pub fn valid_turn(&self, from: &Position, to: &Position) -> bool {
        self.board.is_move_valid(self.player_turn, from.clone(), to.clone())
    }

    pub fn perform_move(&mut self, from: &Position, to: &Position) {
        self.board.move_from_to(from, to);
    }

    pub fn next_player(&mut self) {
        self.player_turn = !self.player_turn
    }
}

use crate::core::board::Board;

use super::position::Position;

pub struct Game {
    pub board: Board,
    pub player_turn: bool, // false is blacks turn
    check: bool
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player_turn: true,
            check: false
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

    pub fn get_winner(&self) -> Option<bool> {
        // let layer_occupied: u64 = self.board.layer_occupied;
        // let layer_color: u64 = self.board.layer_color;
        // let layer_king: u64 = self.board.layer_king;

        // let black_has_no_king: bool = !layer_color & layer_occupied & layer_king == 0b0;
        // let white_has_no_king: bool = layer_color & layer_occupied & layer_king == 0b0;

        // if black_has_no_king {
        //     Some(true)
        // } else if white_has_no_king {
        //     Some(false)
        // } else {
        //     None
        // }

        None
    }
}

use crate::core::board::Board;

use super::{position::Position, move_validator::{is_check, is_checkmate, is_remis}};

pub struct Game {
    pub board: Board,
    pub king_moved: bool,
    pub player_turn: bool, // false is blacks turn
    pub check: bool,
    checkmate: bool,
    remis: bool
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            king_moved: false,
            player_turn: true,
            check: false,
            checkmate: false,
            remis: false
        }
    }

    pub fn valid_turn(&self, from: &Position, to: &Position) -> bool {
        self.board.is_move_valid(self.player_turn, from.clone(), to.clone())
    }

    pub fn perform_move(&mut self, from: &Position, to: &Position) {
        self.board.move_from_to(from, to);
        self.check = is_check(self.board.clone(), self.player_turn);
        
        if is_checkmate(self.board.clone(), self.player_turn) {
            return self.checkmate = true;
        } else if is_remis(self.board.clone(), self.player_turn) {
            return self.remis = true;
        }
        println!("check: {}, checkmate: {}, player: {}", self.check, self.checkmate, self.player_turn);

        self.next_player();
    }

    pub fn next_player(&mut self) {
        self.player_turn = !self.player_turn
    }

    pub fn get_winner(&self) -> Option<u8> {
        if self.checkmate && self.player_turn {
            Some(1)
        } else if self.checkmate && !self.player_turn {
            Some(0)
        } else if self.remis {
            Some(2)
        } else {
            None
        }
    }
}

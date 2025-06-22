use crate::core::{board::Board, piece::PieceType};

use super::{
    move_validator::{is_check, is_checkmate, is_remis},
    position::Position,
};

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub player_turn: bool, // false is blacks turn

    pub check: bool,
    checkmate: bool,
    remis: bool,

    pub step_counter: usize
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player_turn: true,
            check: false,
            checkmate: false,
            remis: false,
            step_counter: 0
        }
    }

    pub fn valid_turn(&self, from: &Position, to: &Position) -> bool {
        self.board.is_move_valid(self.player_turn, from, to)
    }

    pub fn is_castleing_move(&self, from: &Position, to: &Position) -> bool {
        self.board.get_piece_at(from).piece_type() == PieceType::King
        && from.row.abs_diff(to.row) == 0 && from.column.abs_diff(to.column) == 2
    }

    pub fn perform_move(&mut self, from: &Position, to: &Position) {
        if !self.valid_turn(from, to) {
            return;
        }

        if self.is_castleing_move(from, to) && from.column < to.column {
            self.board.move_from_to(from, to);
            self.board.move_from_to(
                &Position { row: from.row, column: 7 },
                &Position { row: from.row, column: 5 })
        } else if self.is_castleing_move(from, to) {
            self.board.move_from_to(from, to);
            self.board.move_from_to(
                &Position { row: from.row, column: 0 },
                &Position { row: from.row, column: 3 })
        } else {
            self.board.move_from_to(from, to);
        }

        self.step_counter += 1;
        
        self.check = is_check(&self.board, self.player_turn);

        if !self.check && is_remis(&self.board, self.player_turn) {
            return self.remis = true;
        } else if self.check && is_checkmate(&self.board, self.player_turn) {
            return self.checkmate = true;
        }

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

use rand::seq::IndexedRandom;

use crate::core::{game::Game, move_generator::get_all_possible_moves, position::Position};

pub fn get_turn(game: &Game) -> (Position, Position) {
    let all_moves: Vec<(Position, Position)> = get_all_possible_moves(&game.board, game.player_turn, true);
    all_moves.choose(&mut rand::rng()).unwrap().clone()
}
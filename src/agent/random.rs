use std::time::{Duration, SystemTime};

use crate::core::{game::Game, move_generator::{get_all_possible_moves, get_possible_moves}, position::Position};

pub fn get_turn(game: &Game) -> (Position, Position) {
    let all_moves: Vec<(Position, Position)> = get_all_possible_moves(&game.board, game.player_turn, true);
    all_moves.get(get_random_int(all_moves.len()) as usize).unwrap().clone()
}

pub fn get_random_digits() -> usize {
    let current_time:u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::new(0, 0)).as_nanos();
    (current_time % 1000000000) as usize
}

pub fn get_random_int(range: usize) -> usize {
    get_random_digits() % range
}

pub fn get_random_float() -> f64 {
    get_random_digits() as f64 / 1000000000.0
}

use good_web_game::timer;

use crate::core::{game::Game, move_generator::get_all_possible_moves, position::Position};

pub fn get_turn(game: &Game) -> (Position, Position) {
    let all_moves: Vec<(Position, Position)> = get_all_possible_moves(&game.board, game.player_turn);
    all_moves.get(get_random_range(all_moves.len())).unwrap().clone()
}

pub fn get_random_f64() -> f64 {
    timer::time() % 1.0
}

fn get_random_u32() -> u32 {
    timer::time() as u32
}

pub fn get_random_range(limit: usize) -> usize {
    get_random_u32() as usize % limit
}

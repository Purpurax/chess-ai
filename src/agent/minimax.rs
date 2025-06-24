use std::{cmp::{max, min}, isize};

use good_web_game::timer;

use crate::core::{board::Board, game::Game, move_generator::get_all_possible_moves, position::Position};

type Move = (Position, Position);

pub fn get_turn(game: &Game, max_compute_time: f64) -> (Position, Position) {
    let maximizing_player: bool = game.player_turn;
    let now: f64 = timer::time();
    let mut depth: usize = 0;

    let mut best_move_total = None;
    let mut best_score_total = if maximizing_player { isize::MIN } else { isize::MAX };
    let mut last_depth_time_elapsed: f64 = 0.0;

    'outer_loop: loop {
        let mut best_move = None;
        let mut best_score = if maximizing_player { isize::MIN } else { isize::MAX };
    
        for (move_used, future_game) in get_all_game_states_with_move(game) {
            let score = minimax(
                &future_game,
                depth,
                isize::MIN,
                isize::MAX,
                now + max_compute_time
            );
            if score.is_none() {
                break 'outer_loop;
            }

            if maximizing_player && score.unwrap() >= best_score {
                best_move = Some(move_used);
                best_score = score.unwrap();
            } else if !maximizing_player && score.unwrap() <= best_score {
                best_move = Some(move_used);
                best_score = score.unwrap();
            }
        }

        best_move_total = best_move;
        best_score_total = best_score;
        last_depth_time_elapsed = timer::time() - now;
        depth += 1;
    }

    println!("\nMinimax:\n > Execution time {:.3?}\n > best score {}\n > depth: {}",
        last_depth_time_elapsed, best_score_total, depth);
    best_move_total.unwrap_or_else(|| panic!("Unable to find any minimax move"))
}

pub fn minimax(
    game: &Game,
    depth: usize,
    mut alpha: isize,
    mut beta: isize,
    stop_time: f64
) -> Option<isize> {
    if timer::time() > stop_time {
        return None
    }

    match game.get_winner() {
        Some(2) => return Some(0),
        Some(1) => return Some(isize::MAX - game.step_counter as isize),
        Some(0) => return Some(isize::MIN + game.step_counter as isize),
        _ => ()
    }

    if depth == 0 {
        return Some(evaluate_board(&game.board))
    }

    let games_after_one_move = get_all_game_states_after_move(game);
    let maximizing_player: bool = game.player_turn;

    if maximizing_player {
        let mut max_eval = isize::MIN + game.step_counter as isize;

        for future_game in games_after_one_move {
            let eval = minimax(&future_game, depth - 1, alpha, beta, stop_time);
            eval?;
            max_eval = max(max_eval, eval.unwrap());

            alpha = max(alpha, eval.unwrap());
            if beta <= alpha {
                break;
            }
        }
        Some(max_eval)
    } else {
        let mut min_eval = isize::MIN - game.step_counter as isize;

        for future_game in games_after_one_move {
            let eval = minimax(&future_game, depth - 1, alpha, beta, stop_time);
            eval?;
            min_eval = min(min_eval, eval.unwrap());

            beta = min(beta, eval.unwrap());
            if beta <= alpha {
                break;
            }
        }
        Some(min_eval)
    }
}

fn get_all_game_states_with_move(
    game: &Game
)  -> impl Iterator<Item=(Move, Game)> + '_ {
    get_all_possible_moves(&game.board, game.player_turn, true)
        .into_iter().map(|(from_pos, to_pos)| {
            let mut new_game: Game = game.clone();
            new_game.perform_move(&from_pos, &to_pos);
            ((from_pos, to_pos), new_game)
        })
}

fn get_all_game_states_after_move(
    game: &Game
)  -> impl Iterator<Item=Game> + '_ {
    get_all_game_states_with_move(game)
        .map(|(_, game)| game)
}

fn evaluate_board(board: &Board) -> isize {
    let mut score: isize = 0;

    score += (board.layer_pawn & board.layer_color).count_ones() as isize;
    score += (board.layer_knight & board.layer_color).count_ones() as isize * 3;
    score += (board.layer_bishop & board.layer_color).count_ones() as isize * 3;
    score += (board.layer_rook & board.layer_color).count_ones() as isize * 5;
    score += (board.layer_queen & board.layer_color).count_ones() as isize * 7;

    score -= (board.layer_pawn & !board.layer_color).count_ones() as isize;
    score -= (board.layer_knight & !board.layer_color).count_ones() as isize * 3;
    score -= (board.layer_bishop & !board.layer_color).count_ones() as isize * 3;
    score -= (board.layer_rook & !board.layer_color).count_ones() as isize * 5;
    score -= (board.layer_queen & !board.layer_color).count_ones() as isize * 7;

    score
}

use std::f64::consts::E;
use std::{error::Error, fs::File};
use std::io::Write;
use good_web_game::timer;
use rand::Rng;
use serde::{Serialize, Deserialize};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::core::board::Board;
use crate::core::game::Game;
use crate::core::move_generator::get_all_possible_moves;
use crate::core::piece::PieceType;
use crate::core::position::Position;

use super::minimax;

const INPUT_NODE_COUNT: usize = 64;
const OUTPUT_NODE_COUNT: usize = 1;

/* How the layers should look like is a great question
    Currently the best architecture is:
    - 10 hidden layers (guess)
    - all interconnected fully
    - input layer takes in piece for every position
    - output layer produces a **valid** Position pair
*/
#[derive(Clone, Serialize, Deserialize)]
pub struct Network {
    input_layers: Vec<Node>,
    hidden_layers: Vec<Vec<Node>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    bias: f64,
    weights: Vec<f64>
}

impl Node {
    pub fn new(weight_count: usize) -> Node {
        Node {
            bias: 0.0,
            weights: vec![1.0 / (weight_count as f64); weight_count]
        }
    }
}

impl Network {
    pub fn new() -> Network {
        let new_network = Network {
            input_layers: vec![Node::new(INPUT_NODE_COUNT); INPUT_NODE_COUNT],
            hidden_layers: vec![
                vec![Node::new(INPUT_NODE_COUNT); 512],
                vec![Node::new(512); 64],
                vec![Node::new(64); 64],
                vec![Node::new(64); 64],
                vec![Node::new(64); OUTPUT_NODE_COUNT]
            ]
        };

        if !has_correct_format(&new_network) {
            panic!("Trying to create a network which has no valid format");
        }

        new_network
    }

    pub fn minimal() -> Network {
        Network {
            input_layers: vec![Node::new(INPUT_NODE_COUNT); INPUT_NODE_COUNT],
            hidden_layers: vec![vec![Node::new(INPUT_NODE_COUNT); OUTPUT_NODE_COUNT]]
        }
    }

    pub fn mutate(&self, mutation_rate: f64, mutation_strength: f64) -> Network {
        let mut rng = rand::rng();
        let mut mutated: Network = self.clone();

        mutated.input_layers.iter_mut()
            .for_each(|node| {
                if rng.random::<f64>() % 1.0 < mutation_rate {
                    node.bias += rng.random_range(-mutation_strength..mutation_strength);
                }

                node.weights.iter_mut()
                    .for_each(|weight| {
                        if rng.random::<f64>() % 1.0 < mutation_rate {
                            *weight += rng.random_range(-mutation_strength..mutation_strength);
                        }
                    })
            });

        mutated.hidden_layers.iter_mut()
            .for_each(|layer| {
                layer.iter_mut()
                    .for_each(|node| {
                        if rng.random::<f64>() % 1.0 < mutation_rate {
                            node.bias += rng.random_range(-mutation_strength..mutation_strength);
                        }

                        node.weights.iter_mut()
                            .for_each(|weight| {
                                if rng.random::<f64>() % 1.0 < mutation_rate {
                                    *weight += rng.random_range(-mutation_strength..mutation_strength);
                                }
                            })
                        })
            });
        
        mutated
    }

    pub fn train_minimax(&mut self, file_path_to_store: &str) {
        let mut network_stayed_same_counter: usize = 0;
        let mut train_counter: usize = 0;

        while true {
            let network_changed: bool = self.train_against_minimax(0.5, 1.0, 1.0);
            if network_changed {
                network_stayed_same_counter = 0;
            } else {
                network_stayed_same_counter += 1;
            }

            if train_counter % 10 == 0 {
                write_network_to_file(file_path_to_store, self.clone());

                println!("Simulated extreme Evolution training {} times. Current best network stayed for {} training sessions",
                    train_counter,
                    network_stayed_same_counter);
            }

            train_counter += 1;
        }
    }

    pub fn train_self(&mut self, file_path_to_store: &str) {
        let mut network_stayed_same_counter: usize = 0;
        let mut train_counter: usize = 0;

        while true {
            let network_changed: bool = self.train(0.3, 1.0);
            if network_changed {
                network_stayed_same_counter = 0;
            } else {
                network_stayed_same_counter += 1;
            }

            if train_counter % 10 == 0 {
                write_network_to_file(file_path_to_store, self.clone());

                println!("Simulated medium Evolution training {} times. Current best network stayed for {} training sessions",
                    train_counter,
                    network_stayed_same_counter);
            }

            train_counter += 1;
        }
    }

    /// Returns true, if the network changed
    fn train(&mut self, mutation_rate: f64, mutation_strength: f64) -> bool {
        let network_a: Network = self.clone();
        let network_b: Network = self.mutate(mutation_rate, mutation_strength);

        let iteration_count: usize = 8;
        let a_won_counter: usize = (0..iteration_count)
            .into_par_iter()
            .map(|iteration| {
                Network::run_simulation(&network_a, &network_b, iteration % 2 == 0)
            })
            .filter(|&won| won)
            .count();

        if a_won_counter * 2 >= iteration_count {
            false
        } else {
            *self = network_b;
            true
        }
    }

    /// Returns true, if the network changed
    fn train_against_minimax(&mut self, mutation_rate: f64, mutation_strength: f64, time_for_minimax: f64) -> bool {
        let network_b: Network = self.mutate(mutation_rate, mutation_strength);
        let network_c: Network = network_b.mutate(mutation_rate, mutation_strength);
        let network_d: Network = network_c.mutate(mutation_rate, mutation_strength);

        let best_network_index: usize = [
            &self,
            &network_b,
            &network_c,
            &network_d
        ].into_par_iter()
            .map(|network| {
                (Network::run_simulation_minimax(time_for_minimax, &network, true)
                - Network::run_simulation_minimax(time_for_minimax, &network, false))
                / 2
            })
            .enumerate()
            .max_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b))
            .map(|(i, _)| i)
            .unwrap();

        if best_network_index == 0 {
            false
        } else if best_network_index == 1 {
            *self = network_b;
            true
        } else if best_network_index == 2 {
            *self = network_c;
            true
        } else {
            *self = network_d;
            true
        }
    }

    /// Return true if network a wins
    fn run_simulation(network_a: &Network, network_b: &Network, a_is_white: bool) -> bool {
        let mut game: Game = Game::new();

        while game.get_winner().is_none() && game.step_counter < 50 {
            if game.player_turn ^ a_is_white {
                let b_turn: (Position, Position) = get_turn(&game, &network_b, true);
                game.perform_move(&b_turn.0, &b_turn.1);
            } else {
                let a_turn: (Position, Position) = get_turn(&game, &network_a, true);
                game.perform_move(&a_turn.0, &a_turn.1);
            }
        }

        match game.get_winner() {
            Some(1) => a_is_white,
            Some(0) => !a_is_white,
            _ => evaluate_simulation(&game) > 0
        }
    }

    /// Return true if network a wins
    fn run_simulation_minimax(time_for_minimax: f64, network: &Network, net_is_white: bool) -> isize {
        let mut game: Game = Game::new();

        while game.get_winner().is_none() && game.step_counter < 50 {
            if game.player_turn ^ net_is_white {
                let turn: (Position, Position) = minimax::get_turn(&game, time_for_minimax, true);
                game.perform_move(&turn.0, &turn.1);
            } else {
                let turn: (Position, Position) = get_turn(&game, &network, true);
                game.perform_move(&turn.0, &turn.1);
            }
        }

        evaluate_simulation(&game)
    }
}

pub fn read_network_from_file(file_path: &str) -> Result<Network, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(&file_path)
        .expect(&format!("Failed to read file at path: {}", file_path));
    let network: Network = serde_json::from_str(&file_content)
        .expect(&format!("Failed to deserialize network from file: {}", file_path));
    if !has_correct_format(&network) {
        return Err("Network has incorrect format".into());
    }
    Ok(network)
}

pub fn write_network_to_file(file_path: &str, network: Network) {
    let serialized = serde_json::to_string(&network).expect("Failed to serialize network");
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(serialized.as_bytes()).expect("Failed to write to file");
}

pub fn has_correct_format(network: &Network) -> bool {
    let mut node_count: usize = INPUT_NODE_COUNT;
    if network.input_layers.len() != node_count
    || network.input_layers.iter().any(|node| node.weights.len() != node_count) {
        return false
    }

    for layer in network.hidden_layers.iter() {
        if layer.iter().any(|node| node.weights.len() != node_count) {
            return false
        }
        node_count = layer.len();
    }

    node_count == OUTPUT_NODE_COUNT
}

pub fn get_turn(initial_game: &Game, network: &Network, silent: bool) -> (Position, Position) {
    let start_time: f64 = timer::time();
    let inital_game_score: f64 = evaluate_game(&initial_game, network);

    let best_move: (Position, Position) = get_all_possible_moves(&initial_game.board, initial_game.player_turn)
        .into_iter()
        .max_by(|(from_pos_a, to_pos_a), (from_pos_b, to_pos_b)| {
            let mut future_game_a: Game = initial_game.clone();
            let mut future_game_b: Game = initial_game.clone();

            future_game_a.perform_move(&from_pos_a, &to_pos_a);
            future_game_b.perform_move(&from_pos_b, &to_pos_b);

            let future_game_score_a: f64 = evaluate_game(&future_game_a, network);
            let future_game_score_b: f64 = evaluate_game(&future_game_b, network);

            future_game_score_a.partial_cmp(&future_game_score_b).unwrap()
        })
        .unwrap();
    let best_move_score: f64 = {
        let mut future_game: Game = initial_game.clone();
        future_game.perform_move(&best_move.0, &best_move.1);
        evaluate_game(&future_game, network)
    };
    
    if !silent {
        println!("\nNeural Network:\n > Execution time {:.3?}\n > initial score {}\n > best score after move: {}",
            timer::time() - start_time,
            inital_game_score,
            best_move_score
        );
    }

    best_move
}

fn evaluate_game(game: &Game, network: &Network) -> f64 {
    let mut values: Vec<f64> = game.board.iterator_pieces()
        .map(|piece| {
            let mut piece_value: f64 = if piece.get_color() {
                1.0
            } else {
                -1.0
            };

            piece_value *= match piece.piece_type() {
                PieceType::Empty => 0.0,
                PieceType::Pawn => 1.0,
                PieceType::Knight => 2.0,
                PieceType::Bishop => 3.0,
                PieceType::Rook => 4.0,
                PieceType::Queen => 5.0,
                PieceType::King => 6.0
            };

            piece_value
        }).collect();
    
    values = network.input_layers.iter()
        .enumerate()
        .map(|(x, node)| {
            let mut new_value: f64 = *values.get(x).unwrap();
            new_value += (*node.weights).into_iter()
                .enumerate()
                .map(|(y, weight)| {
                    weight * values.get(y).unwrap()
                }).sum::<f64>();

            activator(new_value)
        }).collect();
    
    for layer in network.hidden_layers.iter() {
        values = layer.iter()
            .map(|node| {
                let mut new_value: f64 = node.bias;
                new_value += (*node.weights).iter()
                    .enumerate()
                    .map(|(y, weight)| {
                        weight * values.get(y).unwrap()
                    }).sum::<f64>();

                activator(new_value)
            }).collect();
    }

    *values.first().unwrap()
}

/// Based on sigmoid function
fn activator(x: f64) -> f64 {
    1.0 / (1.0 + E.powf(-x))
}

fn evaluate_simulation(game: &Game) -> isize {
    let board: &Board = &game.board;
    let mut score: isize = 0;

    score += (board.layer_pawn & board.layer_color).count_ones() as isize * 10;
    score += (board.layer_knight & board.layer_color).count_ones() as isize * 30;
    score += (board.layer_bishop & board.layer_color).count_ones() as isize * 30;
    score += (board.layer_rook & board.layer_color).count_ones() as isize * 50;
    score += (board.layer_queen & board.layer_color).count_ones() as isize * 70;

    score -= (board.layer_pawn & !board.layer_color).count_ones() as isize * 10;
    score -= (board.layer_knight & !board.layer_color).count_ones() as isize * 30;
    score -= (board.layer_bishop & !board.layer_color).count_ones() as isize * 30;
    score -= (board.layer_rook & !board.layer_color).count_ones() as isize * 50;
    score -= (board.layer_queen & !board.layer_color).count_ones() as isize * 70;

    let center_control_white =
        (0b1100000011000000000000000000000000000 & board.layer_color).count_ones() as isize;
    let center_control_black =
        (0b1100000011000000000000000000000000000 & !board.layer_color).count_ones() as isize;
    
    score += center_control_white;
    score -= center_control_black;

    let winner: Option<u8> = game.get_winner();
    if winner.is_some() && winner.unwrap() == 1 {
        score += 1000000 - 1000 * game.step_counter as isize;
    } else if winner.is_some() && winner.unwrap() == 0 {
        score -= 1000000 - 1000 * game.step_counter as isize;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let network = Network::new();

        println!("Is good? {}", has_correct_format(&network));

        write_network_to_file("/home/master/Project/Rust/chess-ai/data/test.nn", network);
    }

    #[test]
    fn run_training() -> Result<(), Box<dyn Error>> {
        let file_path: &str = "/home/master/Project/Rust/chess-ai/data/test.nn";
        let mut network: Network = read_network_from_file(file_path)?;

        network.train_minimax(file_path);
        Ok(())
    }
}

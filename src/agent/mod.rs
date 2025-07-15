use crate::{agent::{monte_carlo::Tree, neural_network::Network}, core::{game::Game, position::Position}};

pub mod minimax;
pub mod monte_carlo;
pub mod neural_network;
pub mod random;

#[derive(Clone)]
pub struct Agent {
    pub agent_type: AgentType,
    pub game: Game,
    pub max_compute_time: f64
}

#[allow(unused)]
#[derive(Clone)]
pub enum AgentType {
    Random,
    Minimax,
    MonteCarlo(Tree),
    NeuralNetwork(Network)
}

impl Agent {
    pub fn new(agent_type: AgentType, max_compute_time: f64) -> Agent {
        Agent {
            agent_type,
            game: Game::new(),
            max_compute_time
        }
    }

    pub fn inform_about_move(&mut self, from_pos: &Position, to_pos: &Position) {
        match &mut self.agent_type {
            AgentType::Random |
            AgentType::Minimax |
            AgentType::NeuralNetwork(_) => {
                self.game.perform_move(from_pos, to_pos);
            },
            AgentType::MonteCarlo(ref mut tree) => {
                self.game.perform_move(from_pos, to_pos);
                tree.walk_edge_permanently(from_pos, to_pos);
            }
        }
    }

    pub fn get_next_turn(&mut self) -> (Position, Position) {
        let res = match &mut self.agent_type {
            AgentType::Random => random::get_turn(&self.game),
            AgentType::Minimax => minimax::get_turn(&self.game, self.max_compute_time, false),
            AgentType::MonteCarlo(ref mut tree) => {
                monte_carlo::get_turn(&self.game, tree, self.max_compute_time)
            },
            AgentType::NeuralNetwork(network) => {
                neural_network::get_turn(&self.game, network, false)
            }
        };
        res
    }
}

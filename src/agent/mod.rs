use crate::core::{game::Game, position::Position};

pub mod minimax;
pub mod monte_carlo;
pub mod random;

#[derive(Clone)]
pub struct Agent {
    pub agent_type: AgentType,
    pub game: Game,
    pub max_compute_time: f64
}

#[derive(Clone)]
pub enum AgentType {
    Random,
    Minimax,
    // MonteCarlo
}

impl Agent {
    pub fn new(agent_type: AgentType, game: &Game, max_compute_time: f64) -> Agent {
        Agent {
            agent_type,
            game: game.clone(),
            max_compute_time
        }
    }

    pub fn inform_about_move(&mut self, from_pos: &Position, to_pos: &Position) {
        self.game.perform_move(from_pos, to_pos);
    }

    pub fn get_next_turn(&self) -> (Position, Position) {
        match self.agent_type {
            AgentType::Random => random::get_turn(&self.game),
            AgentType::Minimax => minimax::get_turn(&self.game, self.max_compute_time),
            // AgentType::MonteCarlo => monte_carlo::get_turn(&self.game, self.max_compute_time)
        }
    }
}

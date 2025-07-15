mod agent;
mod core;
mod ui;

use agent::neural_network::Network;
use agent::Agent;
use ggez::conf::Conf;
use ggez::GameResult;
use good_web_game as ggez;
use ui::Engine;

use crate::agent::monte_carlo::Tree;
use crate::agent::{neural_network, AgentType};

fn main() {
    #[cfg(feature = "train")]
    neural_network_training().expect("Something went wrong");
    #[cfg(not(feature = "train"))]
    run_game().expect("Something went wrong");
}

fn run_game() -> GameResult {
    let conf = Conf::default()
        // .cache(Some(include_bytes!("../assets.tar")))
        .window_resizable(true)
        .window_title("Chess AI | Purpurax".to_string());

    let white_agent: Option<Agent> =
        // None;
        // Some(Agent::new(AgentType::Random, 1.0));
        // Some(Agent::new(AgentType::Minimax, 1.0));
        // Some(Agent::new(AgentType::MonteCarlo(Tree::new()), 1.0)); // Currently ~37000 nodes
        Some(Agent::new(AgentType::NeuralNetwork(neural_network::read_network_from_file("/home/master/Project/Rust/chess-ai/data/test.nn").expect("Couldn't load neural network")), 1.0));
    let black_agent: Option<Agent> =
        None;
        // Some(Agent::new(AgentType::Random, 1.0));
        // Some(Agent::new(AgentType::Minimax, 1.0)); // Currently ~5
        // Some(Agent::new(AgentType::MonteCarlo(Tree::new()), 1.0));
        // Some(Agent::new(AgentType::NeuralNetwork(neural_network::read_network_from_file("/home/master/Project/Rust/chess-ai/data/test.nn").expect("Couldn't load neural network")), 1.0));

    ggez::start(conf, move |context, quad_ctx| {
        Box::new(Engine::new(context, quad_ctx, white_agent, black_agent).unwrap())
    })
}

fn neural_network_training() -> Result<(), Box<dyn std::error::Error>> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let file_path: &str = "/home/master/Project/Rust/chess-ai/data/test.nn";
    let mut network: Network = neural_network::read_network_from_file(file_path)?;
    
    // network.train_minimax("/home/master/Project/Rust/chess-ai/data/test.nn");
    network.train_self("/home/master/Project/Rust/chess-ai/data/test.nn");
    
    Ok(())
}

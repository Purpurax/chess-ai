mod agent;
mod core;
mod ui;

use agent::neural_network::Network;
use ggez::conf::Conf;
use ggez::GameResult;
use good_web_game as ggez;
use ui::Engine;

use crate::agent::neural_network;

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

    ggez::start(conf, move |context, quad_ctx| {
        Box::new(Engine::new(context, quad_ctx).unwrap())
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

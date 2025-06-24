mod agent;
mod core;
mod ui;

use agent::Agent;
use ggez::conf::Conf;
use ggez::GameResult;
use good_web_game as ggez;
use ui::Engine;

use crate::agent::monte_carlo::Tree;
use crate::agent::AgentType;

fn main() -> GameResult {
    let conf = Conf::default()
        // .cache(Some(include_bytes!("../assets.tar")))
        .window_resizable(true)
        .window_title("Chess AI | Purpurax".to_string());

    let white_agent: Option<Agent> =
        // None;
        // Some(Agent::new(AgentType::Random, 1.0));
        // Some(Agent::new(AgentType::Minimax, 1.0));
        Some(Agent::new(AgentType::MonteCarlo(Tree::new()), 1.0));
    let black_agent: Option<Agent> =
        // None;
        // Some(Agent::new(AgentType::Random, 1.0));
        Some(Agent::new(AgentType::Minimax, 1.0));
        // Some(Agent::new(AgentType::MonteCarlo(Tree::new()), 1.0));

    ggez::start(conf, move |context, quad_ctx| {
        Box::new(Engine::new(context, quad_ctx, white_agent, black_agent).unwrap())
    })
}

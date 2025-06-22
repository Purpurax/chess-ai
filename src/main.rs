mod agent;
mod core;
mod ui;

use ggez::conf::Conf;
use ggez::GameResult;
use good_web_game as ggez;
use ui::Engine;

use crate::agent::AgentType;

fn main() -> GameResult {
    let conf = Conf::default()
        // .cache(Some(include_bytes!("../assets.tar")))
        .window_resizable(true)
        .window_title("Chess AI | Purpurax".to_string());

    let (white_agent, black_agent): (Option<AgentType>, Option<AgentType>) =
        // (None, None);
        // (None, Some(AgentType::Minimax));
        (Some(AgentType::Random), Some(AgentType::Minimax));
        // (Some(AgentType::Random), Some(AgentType::Random));

    ggez::start(conf, move |context, quad_ctx| {
        Box::new(Engine::new(context, quad_ctx, white_agent, black_agent).unwrap())
    })
}

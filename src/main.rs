mod core;
mod ui;

use ggez::conf::Conf;
use ggez::GameResult;
use good_web_game as ggez;
use ui::Engine;

fn main() -> GameResult {
    let conf = Conf::default()
        // .cache(Some(include_bytes!("../assets.tar")))
        .window_resizable(true)
        .window_title("Chess AI | Purpurax".to_string());

    ggez::start(conf, move |context, quad_ctx| {
        Box::new(Engine::new(context, quad_ctx).unwrap())
    })
}

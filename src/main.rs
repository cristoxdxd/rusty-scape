use ggez::{event, GameResult};
mod game;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rusty-scape", "cristoxdxd");
    let (ctx, event_loop) = cb
        .window_setup(ggez::conf::WindowSetup::default().title("rusty-scape"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(game::SCREEN_SIZE.0, game::SCREEN_SIZE.1))
        .build()?;
    let state = game::GameState::new();
    event::run(ctx, event_loop, state)
}

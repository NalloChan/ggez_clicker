use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color};
use ggez::{Context, ContextBuilder, GameError, GameResult};

struct MainState {}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS_LIMIT: u32 = 100;

        while ctx.time.check_update_time(FPS_LIMIT) {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.1, 0.1, 1.0]));
        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() {
    let state = MainState {};

    let config = Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("clicker_game", "me")
        .default_conf(config)
        .window_setup(WindowSetup {
            title: "Clicker Game".to_string(),
            ..Default::default()
        })
        .window_mode(WindowMode {
            width: 1600.0,
            height: 900.0,
            ..Default::default()
        })
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}

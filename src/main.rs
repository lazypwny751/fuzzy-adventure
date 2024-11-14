use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};
use stringcase::Caser;
use flib::defaults::{
	VERSION,
	TITLE,
};

/*
	- Add clap here for presettings and init helper.
*/

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new(format!("{} - {}", TITLE.to_train_case(), VERSION), 1280, 720).build()?.run(|_| Ok(GameState))
}

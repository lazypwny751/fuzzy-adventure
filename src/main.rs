use tetra::graphics::{
	self, 
	Color, 
	text::Text,
	text::Font,
};
use tetra::{
	Context,
	ContextBuilder,
	State,
	math::Vec2,
};
use stringcase::Caser;
use clap::Parser;
use rust_i18n::t;
use flib::defaults::{
	VERSION,
	TITLE,
};

mod utils;

const TEXT_OFFSET: Vec2<f32> = Vec2::new(16.0, 16.0);

rust_i18n::i18n!("locales", fallback = "en"); // Defaults for initialization.

// Get.
pub fn t(key: &str) -> String {
    t!(key).to_string()
}

struct GameState {
	vec: Text,
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let vec = Text::new(format!("{}", t("hello world")), 
			Font::vector(ctx, "assets/gunship.ttf", 16.0)?,
		);

		Ok(GameState { vec })
	}
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.vec.draw(ctx, TEXT_OFFSET);

        Ok(())
    }
}

fn main() -> tetra::Result {
	let options = utils::clap::Opt::parse();

	rust_i18n::set_locale(&options.language);	
	
	// https://docs.rs/tetra/latest/tetra/window/fn.get_current_monitor_size.html
    ContextBuilder::new(format!("{} - {}", TITLE.to_train_case(), VERSION), 1280, 680).build()?.run(GameState::new)
}

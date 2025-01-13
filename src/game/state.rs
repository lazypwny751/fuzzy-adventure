use tetra::{graphics::{ self, Color, text::Text, text::Font }, State, Context };
use flib::defaults::TEXT_OFFSET;
use locales::Transcript;

pub struct GameState {
	text: Text,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.text.draw(ctx, TEXT_OFFSET);

        Ok(())
    }
}

impl GameState {
	pub fn new(ctx: &mut Context, language: &str) -> tetra::Result<GameState> {
		let text = Text::new(format!("{}", Transcript::localize("hello world", &language)),
			Font::vector(ctx, "assets/SourceCodePro-Regular.ttf", 16.0)?,
		);

		Ok(GameState { text })
	}
}

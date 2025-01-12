// Get Public Mods.
mod game;
mod utils;

// Required Libraries.
use tetra::ContextBuilder;
use flib::defaults::{ DEFAULT_SIZE, VERSION, TITLE };
use clap::Parser;
use game::state::GameState;
use locales::Transcript;

// Temp import.
// use tetra::window::{ set_title, set_fullscreen, get_current_monitor_size }; // This is about the doc.

fn main() -> tetra::Result {
	let options = utils::clap::Opt::parse();

	// https://docs.rs/tetra/latest/tetra/window/fn.get_current_monitor_size.html
	match options.mode.as_str() {
		"game" => {
			let mut ctx = ContextBuilder::new(
				format!("{} - {}", TITLE, VERSION), 
				DEFAULT_SIZE.0, // x 
				DEFAULT_SIZE.1	// y
			).quit_on_escape(true).build()?;

			// set_title(&mut ctx, "Hello tetra.");
			// println!("monitor size: {:?}", get_current_monitor_size(&ctx));
			// let _ = set_fullscreen(&mut ctx, false);

			ctx.run(|ctx| GameState::new(ctx, &options.language))
		},

		&_ => {
			// This parameter required for now, it will show parameters without open window.
			println!("Debug mode..");

			println!("options: {:?}", options);
			println!("{}", Transcript::localize("hello world", &options.language));
			Ok(())
		}
	}
}

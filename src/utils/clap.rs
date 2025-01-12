use clap::Parser;
use flib::defaults::VERSION;

#[derive(Parser, Debug)]
#[command(author = "lazypwny751", version = VERSION, propagate_version = true)]
pub struct Opt {
	#[arg(short, long, default_value = "en")]
	pub language: String,

	#[arg(long, default_value = "game")]
	pub mode: String,

	#[arg(long, default_value = "locales")]
	pub locales: String,
}

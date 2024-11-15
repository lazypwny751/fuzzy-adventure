use clap::Parser;
use flib::defaults::VERSION;

#[derive(Parser, Debug)]
#[command(author = "lazypwny751", version = VERSION, propagate_version = true)]
pub struct Opt {
	#[arg(long, default_value = "locales")]
	pub locale: String,

	#[arg(long, default_value = "en")]
	pub language: String,
}

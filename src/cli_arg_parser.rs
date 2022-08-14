use std::env::args;

pub struct RuntimeSettings {
	pub term_mode: bool,
}

pub fn parse() -> RuntimeSettings {
	let mut term_mode: bool = false;
	let args: Vec<String> = args().collect();
	if args.len() > 1 {
		term_mode = true;
	}
	RuntimeSettings { term_mode }
}

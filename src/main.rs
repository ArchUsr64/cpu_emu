mod central_processor;
mod cli_arg_parser;
mod config_parser;
mod control_processor;
mod graphics_processor;
mod json_extensions;
mod memory;
mod sprite_processor;
mod term;

fn main() {
	config_parser::parse_config();
	let term_mode = cli_arg_parser::parse().term_mode;
	let mut gpu = graphics_processor::GPU::new(2, 3);
	let gpu_res = gpu.get_resolution();
	gpu.set_vram(1, 1, 1u8);
	if term_mode {
		term::ctrl_c_init();
		let term_size: term::TerminalSize = term::get_terminal_size();
		println!("Current terminal size: {}", term_size);
		if term_size.rows < gpu_res.0.into() || term_size.cols < gpu_res.1.into() {
			println!(
				"Terminal size insufficient, should be greater than {:?}",
				gpu_res
			);
			std::process::exit(0)
		}
		gpu.render();
	}
}

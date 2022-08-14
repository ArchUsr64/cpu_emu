mod cli_arg_parser;
mod graphics_processor;
mod term;

fn main() {
	let term_mode = cli_arg_parser::parse().term_mode;
	let mut gpu = graphics_processor::GPU::new(2, 3);
	let gpu_res = gpu.get_resolution();
	gpu.set_vram(1, 1, 1u8);
	if term_mode {
		term::ctrl_c_init();
		let term_size: term::TerminalSize = term::get_terminal_size();
		println!("Current terminal size: {}", term_size);
		if term_size.rows < gpu_res.0 || term_size.cols < gpu_res.1 {
			println!(
				"Terminal size insufficient, should be greater than {:?}",
				gpu_res
			);
			std::process::exit(0)
		}
		gpu.render();
	}
}

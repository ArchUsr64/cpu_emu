mod central_processor;
mod cli_arg_parser;
mod computer;
mod config_parser;
mod control_processor;
mod graphics_processor;
mod json_extensions;
mod memory;
mod sprite_processor;
mod term;
use computer::Computer;

fn main() {
	let num: i32 = "1101".parse().unwrap();
	let intval = i32::from_str_radix("1101", 2).unwrap();
	println!("Num is {}", intval);
	let config = config_parser::parse_config_to_json("config.json");
	let mut computer = Computer::new(config);
	computer.tick(0b00000000);
	let instruction_path = "instruction.ijvm";
	let instruction_file = std::fs::read_to_string(instruction_path);
	let instruction_file = match instruction_file {
		Ok(file) => file,
		Err(_) => {
			println!("'{}' not found", instruction_path);
			std::process::exit(0);
		}
	};
	let mut instruction_vec: Vec<String> = Vec::new();
	for instruction in instruction_file.lines() {
		instruction_vec.push(instruction.to_string());
	}
	control_processor::CU::new(computer, instruction_vec);
	std::process::exit(0);
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

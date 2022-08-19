mod central_processor;
mod computer;
mod config_parser;
mod control_processor;
mod graphics_processor;
mod json_extensions;
mod memory;
mod sprite_processor;
mod term;
mod test;
use computer::Computer;
use control_processor::CU;

const DEBUG_ENABLE: bool = false;

fn main() {
	term::ctrl_c_init();
	let config = config_parser::parse_config_to_json("config.json");
	let computer = Computer::new(config);
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
	let mut cu = CU::new(computer, instruction_vec);
	loop {
		cu.tick();
	}
}

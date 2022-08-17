extern crate json;

use crate::central_processor::CPU;
use crate::graphics_processor::GPU;
use crate::sprite_processor::SPU;
use crate::config_parser;
use json::JsonValue;

pub struct Computer {
	spu: SPU,
	cpu: CPU,
	gpu: GPU,
}
impl Computer {
	pub fn new(config: JsonValue) -> Computer {
		Computer {
			spu: config_parser::parse_spu(&config["spu"]),
			cpu: config_parser::parse_cpu(&config["cpu"]),
			gpu: config_parser::parse_gpu(&config["gpu"]),
		}
	}

	pub fn tick(&mut self, instruction: u8) {
		self.cpu.tick(instruction);
		self.spu.tick(&self.cpu, &mut self.gpu);
		self.gpu.render();
	}
}

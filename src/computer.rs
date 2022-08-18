use crate::central_processor::CPU;
use crate::config_parser::{parse_cpu, parse_gpu, parse_spu};
use crate::graphics_processor::GPU;
use crate::sprite_processor::SPU;
use json::JsonValue;

pub struct Computer {
	spu: SPU,
	cpu: CPU,
	gpu: GPU,
}
impl Computer {
	pub fn new(config: JsonValue) -> Computer {
		Computer {
			spu: parse_spu(&config["spu"]),
			cpu: parse_cpu(&config["cpu"]),
			gpu: parse_gpu(&config["gpu"]),
		}
	}

	pub fn tick(&mut self, instruction: u8) {
		self.cpu.tick(instruction);
		self.gpu.clear();
		self.spu.tick(&self.cpu, &mut self.gpu);
		self.gpu.render();
	}

	pub fn get_acr(&self) -> u8 {
		self.cpu.acr.get()
	}
}

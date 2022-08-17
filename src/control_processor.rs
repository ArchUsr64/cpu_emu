extern crate json;

use super::central_processor::CPU;
use super::graphics_processor::GPU;
use super::json_extensions::ToNum;
use super::sprite_processor::SPU;
use crate::sprite_processor::{Sprite, SpriteControlMode, SpriteLayout};
use json::JsonValue;

pub struct Computer {
	spu: SPU,
	cpu: CPU,
	gpu: GPU,
}
impl Computer {
	pub fn new(config: JsonValue) -> Computer {
		Computer {
			spu: Computer::parse_spu(&config["spu"]),
			cpu: Computer::parse_cpu(&config["cpu"]),
			gpu: Computer::parse_gpu(&config["gpu"]),
		}
	}

	pub fn tick(&mut self, instruction: u8) {
		self.cpu.tick(instruction);
		self.spu.tick(&self.cpu, &mut self.gpu);
		self.gpu.render();
	}

	fn parse_gpu(gpu_config: &JsonValue) -> GPU {
		GPU::new(
			gpu_config["resolution"][0].to_num() as u8,
			gpu_config["resolution"][1].to_num() as u8,
		)
	}
	fn parse_cpu(cpu_config: &JsonValue) -> CPU {
		let mut cpu = CPU::new();
		let ram_config = &cpu_config["ram"];
		for i in 0..16 {
			let value = (*ram_config)[i].to_num() as u8;
			cpu.ram.set(i as u8, value);
		}
		let gpr_config = &cpu_config["gpr"];
		for i in 0..4 {
			let value = (*gpr_config)[i].to_num() as u8;
			cpu.gpr.set(i as u8, value);
		}
		cpu.acr.set(cpu_config["acm"].to_num() as u8);
		cpu
	}
	fn parse_spu(spu_config: &JsonValue) -> SPU {
		let layout_vec = Computer::parse_sprite_layout(&spu_config["sprite_layout"]);
		let sprite_vec = Computer::parse_sprite_vec(&spu_config["sprite"]);
		SPU::new(sprite_vec, layout_vec)
	}
	fn parse_sprite_layout(layout_config: &JsonValue) -> Vec<SpriteLayout> {
		let layout_count = layout_config.len();
		let mut sprite_layout_vec: Vec<SpriteLayout> = Vec::new();
		for i in 0..layout_count {
			let layout_config = &layout_config[i];
			let control_mode = layout_config["control_mode"].to_num();
			let size: (usize, usize) = (
				layout_config["texture"][0].len(),
				layout_config["texture"].len(),
			);
			let mut texture = vec![vec![0u8; size.0]; size.1];
			for i in 0..size.1 {
				for j in 0..size.0 {
					texture[i][j] = layout_config["texture"][i][j].to_num() as u8;
				}
			}
			let layout = SpriteLayout::new(size, texture);
			sprite_layout_vec.push(layout);
		}
		sprite_layout_vec
	}
	fn parse_sprite_vec(sprite_config: &JsonValue) -> Vec<Sprite> {
		let sprite_count = sprite_config.len();
		let mut sprite_vec: Vec<Sprite> = Vec::new();
		for i in 0..sprite_count {
			let sprite_config = sprite_config[i].clone();
			let layout_index = sprite_config["layout_index"].to_num() as u8;
			let control_mode = sprite_config["control_mode"].to_num();
			let position = sprite_config["position"].clone();
			let position = (position[0].to_num() as u8, position[1].to_num() as u8);
			let position_pointer = sprite_config["position_pointer"].to_num() as u8;
			let sprite = Sprite::new(
				layout_index,
				match control_mode % 4 {
					0 => SpriteControlMode::XOnly,
					1 => SpriteControlMode::YOnly,
					2 => SpriteControlMode::BiDirectional,
					_ => SpriteControlMode::NoControl,
				},
				position,
				position_pointer,
			);
			sprite_vec.push(sprite);
		}
		sprite_vec
	}
}

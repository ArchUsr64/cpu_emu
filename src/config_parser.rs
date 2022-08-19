use json::JsonValue;

use crate::{
	central_processor::CPU,
	graphics_processor::GPU,
	json_extensions::ToNum,
	sprite_processor::{Sprite, SpriteControlMode, SpriteLayout, SPU},
};

extern crate json;

pub fn parse_config_to_json(config_path: &str) -> JsonValue {
	let f = std::fs::read_to_string(config_path);
	let f = match f {
		Ok(file) => file,
		Err(_) => {
			println!("'{}' not found", config_path);
			std::process::exit(0);
		}
	};
	let config = json::parse(f.as_str()).expect("Failed to parse '{}' successfully");
	config
}

pub fn parse_gpu(gpu_config: &JsonValue) -> GPU {
	GPU::new(
		gpu_config["resolution"][0].to_num() as u8,
		gpu_config["resolution"][1].to_num() as u8,
		gpu_config["render_enable"].to_num() > 0,
		gpu_config["debug_enable"].to_num() > 0,
	)
}
pub fn parse_cpu(cpu_config: &JsonValue) -> CPU {
	let mut cpu = CPU::new(cpu_config["debug_enable"].to_num() > 0);
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
	cpu.acr.set(Some(cpu_config["acm"].to_num() as u8));
	cpu
}
pub fn parse_spu(spu_config: &JsonValue) -> SPU {
	let layout_vec = parse_sprite_layout(&spu_config["sprite_layout"]);
	let sprite_vec = parse_sprite_vec(&spu_config["sprite"]);
	SPU::new(sprite_vec, layout_vec)
}
fn parse_sprite_layout(layout_config: &JsonValue) -> Vec<SpriteLayout> {
	let layout_count = layout_config.len();
	let mut sprite_layout_vec: Vec<SpriteLayout> = Vec::new();
	for i in 0..layout_count {
		let layout_config = &layout_config[i];
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
		let debug_enabled = sprite_config["debug_enable"].to_num() > 0;
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
			debug_enabled,
		);
		sprite_vec.push(sprite);
	}
	sprite_vec
}

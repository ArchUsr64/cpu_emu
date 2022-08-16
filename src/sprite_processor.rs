use super::graphics_processor::GPU;
use super::memory::RAM;

#[derive(Clone, Copy)]
pub enum SpriteControlMode {
	XOnly,
	YOnly,
	BiDirectional,
	NoControl,
}

#[derive(Clone)]
pub struct SpriteLayout {
	size: (usize, usize),
	texture: Vec<Vec<u8>>,
}
impl SpriteLayout {
	pub fn new(size: (usize, usize), texture: Vec<Vec<u8>>) -> SpriteLayout {
		SpriteLayout { size, texture }
	}
}

#[derive(Clone)]
pub struct Sprite {
	layout_index: u8,
	control_mode: SpriteControlMode,
	position: (u8, u8),
	position_pointer: u8,
}
impl Sprite {
	pub fn new(
		layout_index: u8,
		control_mode: SpriteControlMode,
		position: (u8, u8),
		position_pointer: u8,
	) -> Sprite {
		Sprite {
			layout_index,
			control_mode,
			position,
			position_pointer,
		}
	}
	pub fn update_position(&mut self, memory: RAM) {
		let memory_address = self.position_pointer.into();
		self.position = match &self.control_mode {
			SpriteControlMode::XOnly => (memory.get(memory_address), self.position.1),
			SpriteControlMode::YOnly => (self.position.0, memory.get(memory_address)),
			SpriteControlMode::BiDirectional => {
				(memory.get(memory_address), memory.get(memory_address + 1))
			}
			_ => self.position,
		}
	}
	pub fn write_to_gpu(&self, gpu: &mut GPU, layout_vec: &Vec<SpriteLayout>) {
		for i in 0..layout_vec[self.layout_index as usize].size.0 as usize {
			for j in 0..layout_vec[self.layout_index as usize].size.1 {
				gpu.set_vram(
					i + self.position.0 as usize,
					j + self.position.1 as usize,
					layout_vec[self.layout_index as usize].texture[i][j],
				)
			}
		}
	}
}

pub struct SPU {
	sprite_vec: Vec<Sprite>,
	layout_vec: Vec<SpriteLayout>,
}
impl SPU {
	pub fn new(sprite_vec: Vec<Sprite>, layout_vec: Vec<SpriteLayout>) -> SPU {
		SPU {
			sprite_vec,
			layout_vec,
		}
	}
	pub fn tick(self, gpu: &mut GPU) {
		for sprite in self.sprite_vec {
			sprite.write_to_gpu(gpu, &self.layout_vec)
		}
	}
}

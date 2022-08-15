use super::graphics_processor::GPU;
use super::memory::RAM;

pub enum SpriteControlMode {
	XOnly,
	YOnly,
	BiDirectional,
	NoControl,
}

pub struct SpriteLayout {
	control_mode: SpriteControlMode,
	size: (usize, usize),
	texture: Vec<Vec<u8>>,
}
impl SpriteLayout {
	pub fn new(control_mode: SpriteControlMode, texture: Vec<Vec<u8>>) -> SpriteLayout {
		let size = (texture[0].len(), texture.len());
		SpriteLayout {
			control_mode,
			size,
			texture,
		}
	}
}

pub struct Sprite<'a> {
	layout: &'a SpriteLayout,
	position: (u8, u8),
	position_pointer: u8,
}
impl<'a> Sprite<'a> {
	pub fn new(layout: &SpriteLayout, position_pointer: u8) -> Sprite {
		Sprite {
			layout,
			position: (0u8, 0u8),
			position_pointer,
		}
	}
	pub fn set_position(&mut self, position: (u8, u8)) {
		self.position = position
	}
	pub fn update_position(&mut self, memory: RAM) {
		let memory_address = self.position_pointer.into();
		match &self.layout.control_mode {
			XOnly => self.position.0 = memory.get(memory_address),
			YOnly => self.position.1 = memory.get(memory_address),
			BiDirectional => {
				self.position.0 = memory.get(memory_address);
				self.position.1 = memory.get(memory_address + 1);
			}
		}
	}
	pub fn write_to_gpu(&self, gpu: &mut GPU) {
		for i in 0..self.layout.size.0 as usize {
			for j in 0..self.layout.size.1 as usize {
				gpu.set_vram(
					i + self.position.0 as usize,
					j + self.position.1 as usize,
					self.layout.texture[i][j],
				)
			}
		}
	}
}

pub struct SpriteProcessor<'a> {
	sprite_vec: Vec<&'a Sprite<'a>>,
}
impl<'a> SpriteProcessor<'a> {
	pub fn new(sprite_vec: Vec<&'a Sprite>) -> SpriteProcessor<'a> {
		SpriteProcessor { sprite_vec }
	}
	pub fn tick(self, gpu: &mut GPU) {
		for sprite in self.sprite_vec {
			sprite.write_to_gpu(gpu)
		}
	}
}

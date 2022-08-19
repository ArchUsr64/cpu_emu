use crate::log;
use crate::DEBUG_ENABLE;

pub struct GPU {
	vram: Vec<Vec<u8>>,
	resolution: (u8, u8),
	render_enable: bool,
	debug_enable: bool,
}

impl GPU {
	pub fn new(resolution_x: u8, resolution_y: u8, render_enable: bool, debug_enable: bool) -> GPU {
		log!(
			DEBUG_ENABLE,
			"[GPU] New gpu created of size ({:3}x{:3}) with render_enable: '{}'",
			resolution_x,
			resolution_y,
			render_enable
		);
		GPU {
			vram: vec![vec![0u8; resolution_x as usize]; resolution_y as usize],
			resolution: (resolution_x, resolution_y),
			render_enable,
			debug_enable,
		}
	}

	pub fn clear(&mut self) {
		log!(self.debug_enable, "[GPU] VRAM cleared");
		for i in self.vram.iter_mut() {
			for j in i.iter_mut() {
				*j = 0u8;
			}
		}
	}

	pub fn render(&self) {
		log!(self.debug_enable, "[GPU] Called render");
		if self.render_enable {
			for _ in 0..self.resolution.0 + 2 {
				print!("-");
			}
			println!();
			for i in self.vram.iter() {
				for (j, val) in i.iter().enumerate() {
					if j == 0 {
						print!("|")
					}
					if *val == 0u8 {
						print!(" ");
					} else {
						print!("@");
					}
					if j == (self.resolution.1 as usize - 1) {
						print!("|");
					}
				}
				println!();
			}
			for _ in 0..self.resolution.0 + 2 {
				print!("-");
			}
			println!();
		}
	}

	pub fn set_vram(&mut self, address_x: usize, address_y: usize, value: u8) {
		log!(
			self.debug_enable,
			"[GPU] VRAM at ({:3}x{:3}) set to '{:3}'",
			address_x,
			address_y,
			value
		);
		self.vram[address_y % self.resolution.1 as usize][address_x % self.resolution.0 as usize] =
			value;
	}
}

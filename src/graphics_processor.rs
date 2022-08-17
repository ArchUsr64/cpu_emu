use super::term;

pub struct GPU {
	vram: Vec<Vec<u8>>,
	resolution: (u8, u8),
}

impl GPU {
	pub fn new(resolution_x: u8, resolution_y: u8) -> GPU {
		GPU {
			vram: vec![vec![0u8; resolution_x as usize]; resolution_y as usize],
			resolution: (resolution_y, resolution_y),
		}
	}

	pub fn render(&self) {
		term::reset_terminal();
		for i in self.vram.iter() {
			for val in i.iter() {
				let mut draw_value = 1f32;
				if *val == 0u8 {
					draw_value = 0.1f32;
				}
				term::draw_grayscale(draw_value);
			}
			println!();
		}
	}

	pub fn get_resolution(&self) -> (u8, u8) {
		self.resolution
	}

	pub fn set_vram(&mut self, address_x: usize, address_y: usize, value: u8) {
		self.vram[address_y % self.resolution.1 as usize][address_x % self.resolution.0 as usize] =
			value;
	}
}

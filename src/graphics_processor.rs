use super::term;

pub struct GPU {
	vram_size: u32,
	vram: Vec<Vec<u8>>,
	resolution: (u32, u32),
}

impl GPU {
	pub fn new(resolution_x: u32, resolution_y: u32) -> GPU {
		GPU {
			vram_size: resolution_x * resolution_y,
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

	pub fn get_resolution(&self) -> (u32, u32) {
		self.resolution
	}

	pub fn set_vram(&mut self, address_x: u32, address_y: u32, value: u8){
		self.vram[address_y as usize][address_x as usize] = value;
	}
}

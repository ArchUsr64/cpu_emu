pub struct RAM {
	data: [u8; 16],
}

impl RAM {
	pub fn new() -> RAM {
		RAM { data: [0u8; 16] }
	}
	pub fn set(&mut self, address: u32, val: u8) {
		self.data[address as usize] = val;
	}
	pub fn get(&mut self, address: u32) -> u8 {
		self.data[address as usize]
	}
}

pub struct GPR {
	data: [u8; 4],
}

impl GPR {
	pub fn new() -> GPR {
		GPR { data: [0u8; 4] }
	}
	pub fn set(&mut self, address: u32, val: u8) {
		self.data[address as usize] = val;
	}
	pub fn get(&mut self, address: u32) -> u8 {
		self.data[address as usize]
	}
}

pub struct ACR {
	data: u8,
}

impl ACR {
	pub fn new() -> ACR {
		ACR { data: 0u8 }
	}
	pub fn set(&mut self, val: u8) {
		self.data = val;
	}
	pub fn get(&mut self) -> u8 {
		self.data
	}
}

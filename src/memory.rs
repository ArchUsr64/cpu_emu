#[derive(Clone, Copy)]
pub struct RAM {
	data: [u8; 16],
}
impl RAM {
	pub fn new() -> RAM {
		RAM { data: [0u8; 16] }
	}
	pub fn set(&mut self, address: u8, val: u8) {
		self.data[address as usize] = val;
	}
	pub fn get(&self, address: u8) -> u8 {
		self.data[address as usize]
	}
}

//General Purpose Register
pub struct GPR {
	data: [u8; 4],
}
impl GPR {
	pub fn new() -> GPR {
		GPR { data: [0u8; 4] }
	}
	pub fn set(&mut self, address: u8, val: u8) {
		self.data[address as usize] = val;
	}
	pub fn get(&self, address: u8) -> u8 {
		self.data[address as usize]
	}
}

//Accumulator Register
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
	pub fn get(&self) -> u8 {
		self.data
	}
}

//Program Counter Register
type PCR = ACR;

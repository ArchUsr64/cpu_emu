use std::ptr::addr_of_mut;

use crate::log;
use crate::DEBUG_ENABLE;

#[derive(Clone, Copy)]
pub struct RAM {
	data: [u8; 16],
	debug_enable: bool,
}
impl RAM {
	pub fn new(debug_enable: bool) -> RAM {
		log!(DEBUG_ENABLE, "[RAM] NEW");
		RAM {
			debug_enable,
			data: [0u8; 16],
		}
	}
	pub fn set(&mut self, address: u8, val: u8) {
		log!(
			self.debug_enable,
			"[RAM] Value at 0x{:x} to '{:03}'",
			address,
			val
		);
		self.data[address as usize] = val;
	}
	pub fn get(&self, address: u8) -> u8 {
		log!(
			self.debug_enable,
			"[RAM] Value at 0x{:x} requested '{:03}'",
			address,
			self.data[address as usize]
		);
		self.data[address as usize]
	}
}

//General Purpose Register
pub struct GPR {
	data: [u8; 4],
	debug_enable: bool,
}
impl GPR {
	pub fn new(debug_enable: bool) -> GPR {
		log!(DEBUG_ENABLE, "[GPR] NEW");
		GPR {
			data: [0u8; 4],
			debug_enable,
		}
	}
	pub fn set(&mut self, address: u8, val: u8) {
		log!(
			self.debug_enable,
			"[GPR] Value at 0x{:x} to '{:03}'",
			address,
			val
		);
		self.data[address as usize] = val;
	}
	pub fn get(&self, address: u8) -> u8 {
		log!(
			self.debug_enable,
			"[GPR] Value at 0x{:x} requested '{:03}'",
			address,
			self.data[address as usize]
		);
		self.data[address as usize]
	}
}

//Accumulator Register
pub struct ACR {
	data: u8,
	debug_enable: bool,
}
impl ACR {
	pub fn new(debug_enable: bool) -> ACR {
		log!(DEBUG_ENABLE, "[ACR] NEW");
		ACR {
			debug_enable,
			data: 0u8,
		}
	}
	pub fn set(&mut self, val: u8) {
		log!(self.debug_enable, "[ACR] Value set to '{:03}'", val);
		self.data = val;
	}
	pub fn get(&self) -> u8 {
		log!(
			self.debug_enable,
			"[ACR] Value requested '{:03}'",
			self.data
		);
		self.data
	}
}

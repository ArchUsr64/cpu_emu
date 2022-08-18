use std::num::ParseIntError;

use crate::computer::Computer;
use crate::log;
use crate::DEBUG_ENABLE;

pub struct CU {
	computer: Computer,
	pcr: usize,
	instruction_list: Vec<String>,
}

fn parse_instruction(instruction: String) -> (u8, usize) {
	(
		u8::from_str_radix(instruction.as_str(), 2)
			.expect(format!("Failed to parse '{}' into u8", instruction).as_str()),
		usize::from_str_radix(instruction.as_str(), 10)
			.expect(format!("Failed to parse '{}' into usize", instruction).as_str()),
	)
}

impl CU {
	pub fn new(computer: Computer, instruction_list: Vec<String>) -> CU {
		CU {
			computer,
			pcr: 0usize,
			instruction_list,
		}
	}

	pub fn tick(&mut self) {
		if self.pcr >= self.instruction_list.len() {
			log!(true, "[CU] Reached the EOF for instructions");
			std::process::exit(0);
		}
		let instruction = self.instruction_list[self.pcr].clone();
		let flag: String = instruction.chars().take(2).collect();
		let value: String = instruction.chars().skip(2).take(8).collect();
		log!(
			DEBUG_ENABLE,
			"[CU] Ticked with '{}' on line :{}",
			instruction,
			self.pcr + 1
		);
		log!(
			DEBUG_ENABLE,
			"[CU] '{}' parsed as [flag: '{}', value: '{}']",
			instruction,
			flag,
			value
		);
		let value_instruction = parse_instruction(value);
		match flag.as_str() {
			"0b" => {
				log!(
					DEBUG_ENABLE,
					"[CU] Ticking computer with 0b{:08b}",
					value_instruction.0
				);
				self.computer.tick(value_instruction.0);
				self.jump_to_next_line();
			}
			"^^" => self.jump_to_line(value_instruction.1),
			"!^" => {
				let mut status = "FAILED";
				let prev_line = self.get_line_number();
				if self.computer.get_acr() > 0 {
					status = "SUCCESS";
					self.jump_to_line(value_instruction.1);
				} else {
					self.pcr += 1
				};
				log!(
					DEBUG_ENABLE,
					"[CU] CJump at line :{} [{}]",
					prev_line,
					status
				);
			}
			_invalid => {
				log!(true, "[CU] Invalid instruction at :{}", self.pcr + 1);
				log!(true, "[CU] '{}' Not a valid instruction", _invalid);
			}
		};
	}
	fn get_line_number(&self) -> usize {
		self.pcr + 1
	}
	fn jump_to_line(&mut self, line_number: usize) {
		self.pcr = line_number - 1;
		log!(
			DEBUG_ENABLE,
			"[CU] Jumped to :{}, PCR: {}",
			line_number,
			self.pcr
		);
	}
	fn jump_to_next_line(&mut self) {
		self.jump_to_line(self.get_line_number() + 1);
	}
}

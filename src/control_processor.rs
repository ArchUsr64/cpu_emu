use crate::computer::Computer;

pub struct CU {
	computer: Computer,
	pcr: usize,
	instruction_list: Vec<String>,
}

fn parse_instruction(instruction: String) -> u8 {
	u8::from_str_radix(instruction.as_str(), 2).unwrap()
}

fn parse_instruction_index(instruction: String) -> usize {
	usize::from_str_radix(instruction.as_str(), 10).unwrap()
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
			println!("Reached the EOF for instructions");
			std::process::exit(0);
		}
		let instruction = self.instruction_list[self.pcr].clone();
		let flag: String = instruction.chars().take(2).collect();
		let value: String = instruction.chars().skip(2).take(8).collect();
		match flag.as_str() {
			"0b" => {
				self.computer.tick(parse_instruction(value));
				self.pcr += 1;
			}
			"^^" => self.pcr = parse_instruction_index(value),
			"!^" => {
				if self.computer.get_acr() > 0 {
					self.pcr = parse_instruction_index(value);
				}
			}
			_invalid => {
				println!("Invalid instruction at :{}", self.pcr + 1);
				println!("'{}' Not a valid instruction", _invalid);
			}
		};
	}
}

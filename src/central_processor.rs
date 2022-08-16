use super::memory::*;

#[derive(Clone, Copy)]
pub enum CPUOperationName {
	ADD,
	SUB,
	MUL,
	DIV,
	MOD,
	INC,
	DEC,
	AND,
	OR,
	NOT,
	CMPE,
	CMPG,
	MV,
	MVFROMMEM,
	MVTOMEM,
	LOADFROMACM,
}
impl CPUOperationName {
	pub fn process(self, operand1: u8, operand2: u8) -> (u8, bool) {
		match self {
			CPUOperationName::ADD => (operand1 + operand2, true),
			CPUOperationName::SUB => (operand1 - operand2, true),
			CPUOperationName::MUL => (operand1 * operand2, true),
			CPUOperationName::DIV => (operand1 / operand2, true),
			CPUOperationName::MOD => (operand1 % operand2, true),
			CPUOperationName::INC => (operand1 + 1, true),
			CPUOperationName::DEC => (operand1 - 1, true),
			CPUOperationName::AND => (operand1 & operand2, true),
			CPUOperationName::OR => (operand1 | operand2, true),
			CPUOperationName::NOT => (!operand1, true),
			CPUOperationName::CMPE => (if operand1 == operand2 { 1u8 } else { 0u8 }, true),
			CPUOperationName::CMPG => (if operand1 > operand2 { 1u8 } else { 0u8 }, true),
			_ => (0u8, false),
		}
	}
}

#[derive(Clone, Copy)]
pub enum CPUOperationKind {
	FUNCTIONAL,
	MEMORY,
}

#[derive(Clone, Copy)]
pub enum CPUOperationInputType {
	BiOperand,
	UniOperand2,
	UniOperand4,
}

#[derive(Clone, Copy)]
pub struct CPUOperation {
	name: CPUOperationName,
	kind: CPUOperationKind,
	input_type: CPUOperationInputType,
}

#[test]
fn cpu_creation() {
	let cpu = CPU::new();
	for i in 0..16 {
		assert_eq!(cpu.ram.get(i), 0u8);
	}
	for i in 0..4 {
		assert_eq!(cpu.gpr.get(i), 0u8);
	}
	assert_eq!(cpu.acr.get(), 0u8);
}

#[test]
fn cpu_functional_operations() {
	let mut cpu = CPU::new();
	for (operand1, operand2) in [(0, 5), (5, 3), (10, 10), (69, 42)] {
		cpu.gpr.set(0, operand1);
		cpu.gpr.set(1, operand2);
		cpu.tick(0b0000_0001);
		assert_eq!(cpu.acr.get(), operand1 + operand2);
		cpu.tick(0b0001_0001);
		assert_eq!(cpu.acr.get(), operand1 - operand2);
		cpu.tick(0b0010_0001);
		assert_eq!(cpu.acr.get(), operand1 * operand2);
		cpu.tick(0b0011_0001);
		assert_eq!(cpu.acr.get(), operand1 / operand2);
		cpu.tick(0b0100_0001);
		assert_eq!(cpu.acr.get(), operand1 % operand2);
		cpu.tick(0b0101_0001);
		assert_eq!(cpu.acr.get(), operand1 + 1);
		cpu.tick(0b0110_0001);
		assert_eq!(cpu.acr.get(), operand1 - 1);
		cpu.tick(0b0111_0001);
		assert_eq!(cpu.acr.get(), operand1 & operand2);
		cpu.tick(0b1000_0001);
		assert_eq!(cpu.acr.get(), operand1 | operand2);
		cpu.tick(0b1001_0001);
		assert_eq!(cpu.acr.get(), !operand1);
		cpu.tick(0b1010_0001);
		assert_eq!(cpu.acr.get(), if operand1 == operand2 { 1u8 } else { 0u8 });
		cpu.tick(0b1011_0001);
		assert_eq!(cpu.acr.get(), if operand1 > operand2 { 1u8 } else { 0u8 });
	}
}

#[test]
fn cpu_memory_operations() {
	let mut cpu = CPU::new();
	let op_code = 0b1100;
	for i in 0..4 {
		for j in 0..4 {
			for value_to_move in 0..1 {
				cpu.gpr.set(j, value_to_move);
				let instruction = (op_code << 4) + (i << 2) + j;
				cpu.tick(instruction);
				assert_eq!(value_to_move, cpu.gpr.get(i));
			}
		}
	}
	let op_code = 0b1101;
	for i in 0..16 {
		for value_in_ram in 0..=255 {
			cpu.ram.set(i, value_in_ram);
			let instruction = (op_code << 4) + i;
			println!("Instruction to run: {:0b}", instruction);
			cpu.tick(instruction);
			assert_eq!(value_in_ram, cpu.acr.get());
		}
	}
	let op_code = 0b1110;
	for i in 0..16 {
		for value_in_acr in 0..=255 {
			cpu.acr.set(value_in_acr);
			let instruction = (op_code << 4) + i;
			cpu.tick(instruction);
			assert_eq!(value_in_acr, cpu.ram.get(i));
		}
	}
	let op_code = 0b1111;
	for i in 0..4 {
		for value_in_acr in 0..=255 {
			cpu.acr.set(value_in_acr);
			let instruction = (op_code << 4) + (i << 2);
			cpu.tick(instruction);
			assert_eq!(value_in_acr, cpu.gpr.get(i));
		}
	}
}

pub struct CPU {
	pub ram: RAM,
	pub gpr: GPR,
	pub acr: ACR,
	instruction_set: [CPUOperation; 16],
}
impl CPU {
	pub fn new() -> CPU {
		CPU {
			ram: RAM::new(),
			gpr: GPR::new(),
			acr: ACR::new(),
			instruction_set: [
				CPUOperation {
					name: CPUOperationName::ADD,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::SUB,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::MUL,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::DIV,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::MOD,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::INC,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::UniOperand2,
				},
				CPUOperation {
					name: CPUOperationName::DEC,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::UniOperand2,
				},
				CPUOperation {
					name: CPUOperationName::AND,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::OR,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::NOT,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::UniOperand2,
				},
				CPUOperation {
					name: CPUOperationName::CMPE,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::CMPG,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::MV,
					kind: CPUOperationKind::MEMORY,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::MVFROMMEM,
					kind: CPUOperationKind::MEMORY,
					input_type: CPUOperationInputType::UniOperand4,
				},
				CPUOperation {
					name: CPUOperationName::MVTOMEM,
					kind: CPUOperationKind::MEMORY,
					input_type: CPUOperationInputType::UniOperand4,
				},
				CPUOperation {
					name: CPUOperationName::LOADFROMACM,
					kind: CPUOperationKind::MEMORY,
					input_type: CPUOperationInputType::UniOperand2,
				},
			],
		}
	}
	fn parse_instruction(instruction: u8) -> (u8, u8, u8) {
		let op_code = instruction >> 4;
		let operand1 = (instruction >> 2) % 4;
		let operand2 = instruction % 4;
		(op_code, operand1, operand2)
	}
	fn operation_from_op_code(&self, op_code: u8) -> CPUOperation {
		self.instruction_set[op_code as usize]
	}
	pub fn tick(&mut self, instruction: u8) {
		let (op_code, operand1, operand2) = CPU::parse_instruction(instruction);
		let unified_operand = operand1 * 4 + operand2;
		let operation = self.operation_from_op_code(op_code);
		match operation.kind {
			CPUOperationKind::FUNCTIONAL => {
				let input1 = self.gpr.get(operand1);
				let input2 = self.gpr.get(operand2);
				self.acr.set(operation.name.process(input1, input2).0)
			}
			CPUOperationKind::MEMORY => match operation.name {
				CPUOperationName::MV => self.gpr.set(operand1, self.gpr.get(operand2)),
				CPUOperationName::MVFROMMEM => self.acr.set(self.ram.get(unified_operand)),
				CPUOperationName::MVTOMEM => self.ram.set(unified_operand, self.acr.get()),
				CPUOperationName::LOADFROMACM => self.gpr.set(operand1, self.acr.get()),
				_ => (),
			},
		}
	}
}

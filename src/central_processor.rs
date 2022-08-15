use super::memory::*;

#[derive(Clone, Copy)]
pub enum CPUOperationName {
	ADD,
	SUB,
	MUL,
	DIV,
	MOD,
	LOADFROMACM,
	INC,
	DEC,
	MV,
	MVFROMMEM,
	MVTOMEM,
	AND,
	OR,
	NOT,
	SHL,
	SHR,
}
impl CPUOperationName {
	pub fn process(self, operand1: u8, operand2: u8) -> (u8, bool) {
		match self {
			CPUOperationName::ADD => (operand1 + operand2, true),
			CPUOperationName::SUB => (operand1 - operand2, true),
			CPUOperationName::MUL => (operand1 * operand2, true),
			CPUOperationName::DIV => (operand1 + operand2, true),
			CPUOperationName::MOD => (operand1 % operand2, true),
			CPUOperationName::INC => (operand1 + 1, true),
			CPUOperationName::DEC => (operand1 - 1, true),
			CPUOperationName::AND => (operand1 & operand2, true),
			CPUOperationName::OR => (operand1 | operand2, true),
			CPUOperationName::NOT => (!operand1, true),
			CPUOperationName::SHL => (operand1 << operand2, true),
			CPUOperationName::SHR => (operand1 >> operand2, true),
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

pub struct CPU {
	ram: RAM,
	gpr: GPR,
	acr: ACR,
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
					name: CPUOperationName::LOADFROMACM,
					kind: CPUOperationKind::MEMORY,
					input_type: CPUOperationInputType::UniOperand2,
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
					name: CPUOperationName::SHL,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
				},
				CPUOperation {
					name: CPUOperationName::SHR,
					kind: CPUOperationKind::FUNCTIONAL,
					input_type: CPUOperationInputType::BiOperand,
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
		let unified_operand = operand1 * 16 + operand2;
		let operation = self.operation_from_op_code(op_code);
		match operation.kind {
			CPUOperationKind::FUNCTIONAL => {
				let input1 = self.gpr.get(operand1);
				let input2 = self.gpr.get(operand2);
				self.acr.set(operation.name.process(input1, input2).0)
			}
			CPUOperationKind::MEMORY => match operation.name {
				CPUOperationName::LOADFROMACM => self.gpr.set(operand1, self.acr.get()),
				CPUOperationName::MV => self.gpr.set(operand1, self.gpr.get(operand2)),
				CPUOperationName::MVFROMMEM => self.acr.set(self.ram.get(unified_operand)),
				CPUOperationName::MVTOMEM => self.ram.set(unified_operand, self.acr.get()),
				_ => (),
			},
		}
	}
}

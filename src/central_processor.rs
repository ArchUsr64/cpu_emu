use super::memory::*;
use crate::log;
use crate::DEBUG_ENABLE;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum CPUOperation {
	ADD = 0x0,
	SUB = 0x1,
	MUL = 0x2,
	DIV = 0x3,
	MOD = 0x4,
	SHL = 0x5,
	SHR = 0x6,
	AND = 0x7,
	ORR = 0x8,
	NOT = 0x9,
	CME = 0xa,
	CMG = 0xb,
	MVG = 0xc,
	MFM = 0xd,
	MTM = 0xe,
	LAC = 0xf,
}

impl CPUOperation {
	pub fn from_code(code: u8) -> CPUOperation {
		CPUOperation::from_int(code)
			.expect(format!("Instruction code 0x{:x} is invalid", code).as_str())
	}
	pub fn execute(instruction: u8, cpu: &mut CPU) {
		let (operation, unified_operand) = (instruction >> 4, instruction % 16);
		let (operand1, operand2) = (unified_operand >> 2, unified_operand % 4);
		let (value1, value2) = (cpu.gpr.get(operand1), cpu.gpr.get(operand2));
		let operation = CPUOperation::from_code(operation);
		cpu.acr.set(match operation {
			CPUOperation::ADD => Some(value1 + value2),
			CPUOperation::SUB => Some(value1 - value2),
			CPUOperation::MUL => Some(value1 * value2),
			CPUOperation::DIV => Some(value1 / value2),
			CPUOperation::MOD => Some(value1 % value2),
			CPUOperation::AND => Some(value1 & value2),
			CPUOperation::ORR => Some(value1 | value2),
			CPUOperation::CMG => Some((value1 > value2) as u8),
			CPUOperation::CME => Some((value1 == value2) as u8),
			CPUOperation::SHL => Some(value1 << value2),
			CPUOperation::SHR => Some(value1 >> value2),
			CPUOperation::MVG => {
				cpu.gpr.set(operand1, cpu.gpr.get(operand2));
				None
			}
			_ => None,
		});
		match operation {
			CPUOperation::NOT => cpu.acr.set(Some(!cpu.gpr.get(operand1))),
			CPUOperation::LAC => cpu.gpr.set(operand1, cpu.acr.get()),
			_ => (),
		};
		match operation {
			CPUOperation::MTM => cpu.ram.set(unified_operand, cpu.acr.get()),
			CPUOperation::MFM => cpu.acr.set(Some(cpu.ram.get(unified_operand))),
			_ => (),
		};
	}
}

pub struct CPU {
	pub ram: RAM,
	pub gpr: GPR,
	pub acr: ACR,
	debug_enable: bool,
}
impl CPU {
	pub fn new(debug_enable: bool) -> CPU {
		log!(DEBUG_ENABLE, "[CPU] NEW");
		CPU {
			ram: RAM::new(debug_enable),
			gpr: GPR::new(debug_enable),
			acr: ACR::new(debug_enable),
			debug_enable,
		}
	}
	pub fn tick(&mut self, instruction: u8) {
		log!(
			self.debug_enable,
			"[CPU] Ticked with instruction 0b{:08b}",
			instruction
		);
		CPUOperation::execute(instruction, self);
	}
}

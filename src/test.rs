#[cfg(test)]
use crate::central_processor::CPU;

#[test]
fn cpu_creation() {
	let cpu = CPU::new(true);
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
	let mut cpu = CPU::new(true);
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
		assert_eq!(cpu.acr.get(), operand1 << operand2);
		cpu.tick(0b0110_0001);
		assert_eq!(cpu.acr.get(), operand1 >> operand2);
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
	let mut cpu = CPU::new(true);
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
			cpu.tick(instruction);
			assert_eq!(value_in_ram, cpu.acr.get());
		}
	}
	let op_code = 0b1110;
	for i in 0..16 {
		for value_in_acr in 0..=255 {
			cpu.acr.set(Some(value_in_acr));
			let instruction = (op_code << 4) + i;
			cpu.tick(instruction);
			assert_eq!(value_in_acr, cpu.ram.get(i));
		}
	}
	let op_code = 0b1111;
	for i in 0..4 {
		for value_in_acr in 0..=255 {
			cpu.acr.set(Some(value_in_acr));
			let instruction = (op_code << 4) + (i << 2);
			cpu.tick(instruction);
			assert_eq!(value_in_acr, cpu.gpr.get(i));
		}
	}
}

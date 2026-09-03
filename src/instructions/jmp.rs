use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn jmp_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let jump_address: Word = cpu.fetch_word(cycles, memory);
    cpu.PC = jump_address; 
}

pub fn jmp_ind(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let pointer: Word = cpu.fetch_word(cycles, memory);
    let effective_address: Word = cpu.read_word(cycles, pointer, memory);

    cpu.PC = effective_address;
}

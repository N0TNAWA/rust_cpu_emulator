use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn rts(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let return_address: Word = cpu.pop_word_from_stack(cycles, memory);
    cpu.PC = return_address + 1;

    *cycles -= 2;
}

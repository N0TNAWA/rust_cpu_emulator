use crate::cpu::CPU;
use crate::memory::MEM;

pub fn jsr(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let sub_address = cpu.fetch_word(cycles, memory);

    cpu.push_sp_to_stack(cycles, memory);

    cpu.PC = sub_address;

    *cycles -= 1;
}

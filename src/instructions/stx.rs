use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn stx_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);

    cpu.write_byte(zero_page_address as Word, cpu.X, cycles, memory);
}

pub fn stx_zpy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.Y);

    cpu.write_byte(zero_page_address as Word, cpu.X, cycles, memory);
}

pub fn stx_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);

    cpu.write_byte(absolute_address, cpu.X, cycles, memory);
}

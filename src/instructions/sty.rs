use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn sty_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);

    cpu.write_byte(zero_page_address as Word, cpu.Y, cycles, memory);
}

pub fn sty_zpx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.X);

    cpu.write_byte(zero_page_address as Word, cpu.Y, cycles, memory);
}

pub fn sty_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);

    cpu.write_byte(absolute_address, cpu.Y, cycles, memory);
}

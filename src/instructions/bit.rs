use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn bit_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let value: Byte = cpu.read_byte(cycles, zero_page_address as Word, memory);

    cpu.set_vn_status(value);

    let result = cpu.bitwise_and(cpu.A, value);

    cpu.set_z_status(result);
}

pub fn bit_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let value: Byte = cpu.read_byte(cycles, absolute_address, memory);

    cpu.set_vn_status(value);

    let result = cpu.bitwise_and(cpu.A, value);

    cpu.set_z_status(result);
}

use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn sta_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);

    cpu.write_byte(zero_page_address as Word, cpu.A, cycles, memory);    
}

pub fn sta_zpx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.X);

    cpu.write_byte(zero_page_address as Word, cpu.A, cycles, memory);
}

pub fn sta_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word( cycles, memory );
    
    cpu.write_byte(absolute_address, cpu.A, cycles, memory);
}

pub fn sta_absx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_x = absolute_address.wrapping_add(cpu.X as Word);

    cpu.write_byte(absolute_address_x, cpu.A, cycles, memory);
}

pub fn sta_absy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_y = absolute_address.wrapping_add(cpu.Y as Word);

    cpu.write_byte(absolute_address_y, cpu.A, cycles, memory);
}

pub fn sta_indx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.X);

    *cycles -= 1;

    let effective_address: Word = cpu.read_word(cycles, zero_page_address as Word, memory);

    cpu.write_byte(effective_address, cpu.A, cycles, memory);
}

pub fn sta_indy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let effective_address: Word = cpu.read_word(cycles, zero_page_address as Word, memory);
    let effective_address_y = effective_address.wrapping_add(cpu.Y as Word);

    cpu.write_byte(effective_address_y, cpu.A, cycles, memory);
}

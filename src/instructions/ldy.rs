use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn ldy_im(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let value: Byte = cpu.fetch_byte(cycles, memory);
    cpu.Y = value;
    
    cpu.set_zn_status();
}

pub fn ldy_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    cpu.Y = cpu.read_byte( cycles, zero_page_address as Word, memory);

    cpu.set_zn_status();
}

pub fn ldy_zpx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.X);

    *cycles -= 1;

    cpu.Y = cpu.read_byte(cycles, zero_page_address as Word, memory);
    cpu.set_zn_status();
}

pub fn ldy_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    cpu.Y = cpu.read_byte(cycles, absolute_address, memory);

    cpu.set_zn_status();

    cpu.debug();
}

pub fn ldy_absx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_x = absolute_address.wrapping_add(cpu.X as Word);
    cpu.Y = cpu.read_byte(cycles, absolute_address_x, memory); 

    if (absolute_address & 0xFF00) != (absolute_address_x & 0xFF00) {
        *cycles -= 1;
    }

    cpu.set_zn_status();

    cpu.debug();
}

use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn ldx_im(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let value: Byte = cpu.fetch_byte(cycles, memory);
    cpu.X = value;
    
    cpu.set_zn_status();
}

pub fn ldx_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte( cycles, memory );
    cpu.X = cpu.read_byte(cycles, zero_page_address as Word, memory);

    cpu.set_zn_status();
}

pub fn ldx_zpy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.Y);

    *cycles -= 1;

    cpu.X = cpu.read_byte( cycles, zero_page_address as Word, memory);
    cpu.set_zn_status();
}

pub fn ldx_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    cpu.X = cpu.read_byte(cycles, absolute_address, memory);

    cpu.set_zn_status();

    cpu.debug();
}

pub fn ldx_absy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_y = absolute_address.wrapping_add(cpu.Y as Word);
    cpu.X = cpu.read_byte(cycles, absolute_address_y, memory); 

    if (absolute_address & 0xFF00) != (absolute_address_y & 0xFF00) {
        *cycles -= 1;
    }

    cpu.set_zn_status();

    cpu.debug();
}

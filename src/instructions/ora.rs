use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn ora_im(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let value: Byte = cpu.fetch_byte(cycles, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;

    cpu.set_zn_status();
}

pub fn ora_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let value: Byte = cpu.read_byte(cycles, zero_page_address as Word, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;

    cpu.set_zn_status();
}

pub fn ora_zpx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let mut zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    zero_page_address = zero_page_address.wrapping_add(cpu.X);

    *cycles -= 1;

    let value: Byte = cpu.read_byte(cycles, zero_page_address as Word, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;

    cpu.set_zn_status();
}

pub fn ora_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let value: Byte = cpu.read_byte(cycles, absolute_address, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;

    cpu.set_zn_status();
}

pub fn ora_absx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_x = absolute_address.wrapping_add(cpu.X as Word);

    if (absolute_address & 0xFF00) != (absolute_address_x & 0xFF00) {
        *cycles -= 1;
    }

    let value: Byte = cpu.read_byte(cycles, absolute_address_x, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;

    cpu.set_zn_status();
}

pub fn ora_absy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_y = absolute_address.wrapping_add(cpu.Y as Word);

    if (absolute_address & 0xFF00) != (absolute_address_y & 0xFF00) {
        *cycles -= 1;
    }
    
    let value: Byte = cpu.read_byte(cycles, absolute_address_y, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;

    cpu.set_zn_status();
}

pub fn ora_indx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.X);
    let effective_address: Word = cpu.read_word(cycles, zero_page_address as Word, memory);

    *cycles -= 1;

    let value = cpu.read_byte(cycles, effective_address, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;
    cpu.set_zn_status();
}

pub fn ora_indy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let effective_address: Word = cpu.read_word(cycles, zero_page_address as Word, memory);
    let effective_address_y = effective_address.wrapping_add(cpu.Y as Word);

    if (effective_address & 0xFF00) != (effective_address_y & 0xFF00) {
        *cycles -= 1;
    }

    let value = cpu.read_byte(cycles, effective_address_y, memory);
    let new_value = cpu.bitwise_ora(cpu.A, value);

    cpu.A = new_value;
    cpu.set_zn_status();
}

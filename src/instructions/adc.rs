use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn adc_im(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let value: Byte = cpu.fetch_byte(cycles, memory);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_zp(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    let value = cpu.read_byte(cycles, zero_page_address as Word, memory);
    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_zpx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let mut zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};
    zero_page_address = zero_page_address.wrapping_add(cpu.X);

    *cycles -= 1;

    let value = cpu.read_byte(cycles, zero_page_address as Word, memory);
    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_abs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    let value = cpu.read_byte(cycles, absolute_address, memory);
    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_absx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_x = absolute_address.wrapping_add(cpu.X as Word);

    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    if (absolute_address & 0xFF00) != (absolute_address_x & 0xFF00) {
        *cycles -= 1;
    }

    let value = cpu.read_byte(cycles, absolute_address_x, memory);
    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_absy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let absolute_address: Word = cpu.fetch_word(cycles, memory);
    let absolute_address_y = absolute_address.wrapping_add(cpu.Y as Word);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    if (absolute_address & 0xFF00) != (absolute_address_y & 0xFF00) {
        *cycles -= 1;
    }

    let value = cpu.read_byte(cycles, absolute_address_y, memory);
    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_indx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let zero_page_address = zero_page_address.wrapping_add(cpu.X);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    *cycles -= 1;

    let effective_address: Word = cpu.read_word(cycles, zero_page_address as Word, memory);
    let value = cpu.read_byte(cycles, effective_address, memory);

    cpu.A = cpu.adc_set_status(value, carry); 
}

pub fn adc_indy(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, C: Byte) {
    let zero_page_address: Byte = cpu.fetch_byte(cycles, memory);
    let effective_address: Word = cpu.read_word(cycles, zero_page_address as Word, memory);
    let effective_address_y = effective_address.wrapping_add(cpu.Y as Word);
    let carry: Byte = if cpu.PS & C != 0 {1} else {0};

    let value = cpu.read_byte(cycles, effective_address_y, memory);

    if (effective_address & 0xFF00) != (effective_address_y & 0xFF00) {
        *cycles -= 1;
    }

    cpu.A = cpu.adc_set_status(value, carry); 
}


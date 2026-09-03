use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn cl(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM, flag: Byte) {
    cpu.cl_set_status(flag);
    
    *cycles -= 1;  
}

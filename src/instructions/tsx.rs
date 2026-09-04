use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn tsx(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    cpu.X = cpu.SP;
    
    *cycles -= 1;

    cpu.set_zn_status("X");
}

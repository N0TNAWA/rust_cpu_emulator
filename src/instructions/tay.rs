use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn tay(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    cpu.Y = cpu.A;
    
    *cycles -= 1;

    cpu.set_zn_status("Y");
}

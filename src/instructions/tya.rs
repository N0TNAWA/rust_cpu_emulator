use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn tya(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    cpu.A = cpu.Y;
    
    *cycles -= 1;

    cpu.set_zn_status("A");
}

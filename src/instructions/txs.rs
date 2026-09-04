use crate::{Byte, Word};
use crate::cpu::CPU;
use crate::memory::MEM;

pub fn txs(cpu: &mut CPU, cycles: &mut u32, memory: &mut MEM)  {
    cpu.SP = cpu.X;
    
    *cycles -= 1;
}

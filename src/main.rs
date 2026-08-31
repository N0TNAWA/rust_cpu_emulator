mod cpu;
mod memory;
mod instructions;

type Byte = u8;
type Word = u16;

// Size allocation
pub const MAX_MEM: usize = 1024*64;

use cpu::CPU;
use memory::MEM;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = MEM::new();

    //Test INS_LDA_ABSX
    cpu.reset(&mut mem);
    
    cpu.X = 1;
    mem.DATA[0xFFFC] = instructions::INS_LDY_ABSX;
    mem.DATA[0xFFFD] = 0x80;
    mem.DATA[0xFFFE] = 0x44;
    mem.DATA[0x4481] = 0x37;

    cpu.execute(4, &mut mem);

    //Test INS_LDA_ABSY
    cpu.reset(&mut mem);

    cpu.Y = 1;
    mem.DATA[0xFFFC] = instructions::INS_LDX_ABSY;
    mem.DATA[0xFFFD] = 0x80;
    mem.DATA[0xFFFE] = 0x44;
    mem.DATA[0x4481] = 0x37;

    cpu.execute(4, &mut mem);
}

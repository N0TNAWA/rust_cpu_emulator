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

    cpu.reset(&mut mem);

    //mem.DATA[0xFF00] = instructions::INS_JSR;
    //mem.DATA[0xFF01] = 0x00;
    //mem.DATA[0xFF02] = 0x80;
    //mem.DATA[0x8000] = instructions::INS_RTS;
    //mem.DATA[0xFF03] = instructions::INS_LDA_IM;
    //mem.DATA[0xFF04] = 0x42;
    
    //inline a program (Worlds most complicated calculator) - start
    cpu.X = 0xFF; //Off-Set the X register
    mem.DATA[0xFF00] = instructions::INS_JSR;
    mem.DATA[0xFF01] = 0x42;
    mem.DATA[0xFF02] = 0x42;
    mem.DATA[0x4242] = instructions::INS_LDA_IM;
    mem.DATA[0x4243] = 0x09; // First number
    mem.DATA[0x4244] = instructions::INS_CLC_I;
    mem.DATA[0x4245] = instructions::INS_ADC_ZPX;
    mem.DATA[0x4246] = 0x80; // Second number address
    mem.DATA[0x007F] = 0x20; // Actual number
    mem.DATA[0x4247] = instructions::INS_STA_ABS;
    mem.DATA[0x4248] = 0x00;
    mem.DATA[0x4249] = 0x02;
    mem.DATA[0x424A] = instructions::INS_RTS;
    mem.DATA[0xFF03] = instructions::INS_LDA_IM;
    mem.DATA[0xFF04] = 0x42;
    //inline a program - end

    cpu.execute(26, &mut mem);
}

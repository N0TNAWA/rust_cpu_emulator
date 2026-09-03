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

    //Large test program
    cpu.X = 2;
    cpu.Y = 1;

    // LDA #$0F
    // 2 cycles
    mem.DATA[0x8000] = instructions::INS_LDA_IM;
    mem.DATA[0x8001] = 0x0F;

    // LDX #$02
    // 2 cycles
    mem.DATA[0x8002] = instructions::INS_LDX_IM;
    mem.DATA[0x8003] = 0x02;

    // LDY #$01
    // 2 cycles
    mem.DATA[0x8004] = instructions::INS_LDY_IM;
    mem.DATA[0x8005] = 0x01;


    // ---------- Zero Page ----------

    // Put value at $0010
    mem.DATA[0x0010] = 0x37;

    // LDA $10
    // 3 cycles
    mem.DATA[0x8006] = instructions::INS_LDA_ZP;
    mem.DATA[0x8007] = 0x10;

    // STA $20
    // 3 cycles
    mem.DATA[0x8008] = instructions::INS_STA_ZP;
    mem.DATA[0x8009] = 0x20;


    // ---------- Zero Page,X ----------

    // $20 + X ($02) = $22
    mem.DATA[0x0022] = 0x55;

    // LDA $20,X
    // 4 cycles
    mem.DATA[0x800A] = instructions::INS_LDA_ZPX;
    mem.DATA[0x800B] = 0x20;


    // ---------- Absolute ----------

    // LDA $4400
    // 4 cycles
    mem.DATA[0x800C] = instructions::INS_LDA_ABS;
    mem.DATA[0x800D] = 0x00;
    mem.DATA[0x800E] = 0x44;

    mem.DATA[0x4400] = 0x66;


    // STA $0200
    // 4 cycles
    mem.DATA[0x800F] = instructions::INS_STA_ABS;
    mem.DATA[0x8010] = 0x00;
    mem.DATA[0x8011] = 0x02;


    // ---------- Absolute,X ----------

    // $4400 + X ($02) = $4402
    mem.DATA[0x4402] = 0x77;

    // LDA $4400,X
    // 4 cycles
    mem.DATA[0x8012] = instructions::INS_LDA_ABSX;
    mem.DATA[0x8013] = 0x00;
    mem.DATA[0x8014] = 0x44;


    // ---------- Absolute,Y ----------

    // $4400 + Y ($01) = $4401
    mem.DATA[0x4401] = 0x88;

    // LDA $4400,Y
    // 4 cycles
    mem.DATA[0x8015] = instructions::INS_LDA_ABSY;
    mem.DATA[0x8016] = 0x00;
    mem.DATA[0x8017] = 0x44;


    // ---------- AND ----------

    // A = $F0
    // AND #$0F = $00
    // 2 cycles
    mem.DATA[0x8018] = instructions::INS_LDA_IM;
    mem.DATA[0x8019] = 0xF0;

    mem.DATA[0x801A] = instructions::INS_AND_IM;
    mem.DATA[0x801B] = 0x0F;


    // ---------- ADC ----------

    // A = $05
    // CLC
    // ADC #$0A
    // Result = $0F

    // LDA #$05
    // 2 cycles
    mem.DATA[0x801C] = instructions::INS_LDA_IM;
    mem.DATA[0x801D] = 0x05;

    // CLC
    // 2 cycles
    mem.DATA[0x801E] = instructions::INS_CLC;

    // ADC #$0A
    // 2 cycles
    mem.DATA[0x801F] = instructions::INS_ADC_IM;
    mem.DATA[0x8020] = 0x0A;


    // ---------- STX / STY ----------

    // STX $30
    // 3 cycles
    mem.DATA[0x8021] = instructions::INS_STX_ZP;
    mem.DATA[0x8022] = 0x30;

    // STY $31
    // 3 cycles
    mem.DATA[0x8023] = instructions::INS_STY_ZP;
    mem.DATA[0x8024] = 0x31;


    // ---------- JSR / RTS ----------

    // JSR $9000
    // 6 cycles
    mem.DATA[0x8025] = instructions::INS_JSR;
    mem.DATA[0x8026] = 0x00;
    mem.DATA[0x8027] = 0x90;


    // ---------- Subroutine at $9000 ----------

    // LDA #$AA
    // 2 cycles
    mem.DATA[0x9000] = instructions::INS_LDA_IM;
    mem.DATA[0x9001] = 0xAA;

    // STA $0201
    // 4 cycles
    mem.DATA[0x9002] = instructions::INS_STA_ABS;
    mem.DATA[0x9003] = 0x01;
    mem.DATA[0x9004] = 0x02;

    // RTS
    // 6 cycles
    mem.DATA[0x9005] = instructions::INS_RTS;

    cpu.execute(66, &mut mem);
}

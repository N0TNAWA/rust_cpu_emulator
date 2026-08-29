mod core;

fn main() {
    let mut mem = core::MEM {
        DATA: [0; core::MAX_MEM],
    };

    let mut cpu = core::CPU {
        PC: 0,
        SP: 0,

        A: 0,
        X: 0,
        Y: 0,

        PS: 0,
    };

    cpu.reset(&mut mem);
    
    //inline a program (Worlds most complicated calculator) - start
    mem.DATA[0xFFFC] = core::INS_JSR;
    mem.DATA[0xFFFD] = 0x42;
    mem.DATA[0xFFFE] = 0x42;
    mem.DATA[0x4242] = core::INS_LDA_IM;
    mem.DATA[0x4243] = 0x09; // First number
    mem.DATA[0x4244] = core::INS_ADC_IM;
    mem.DATA[0x4245] = 0x10; // Second number
    mem.DATA[0x4246] = core::INS_STA_ABS;
    mem.DATA[0x4247] = 0x00;
    mem.DATA[0x4248] = 0x02;
    //inline a program - end

    cpu.execute(14, &mut mem);
}

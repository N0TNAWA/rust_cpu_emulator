use crate::Byte;

// Instructions
pub mod lda;
pub mod ldx;
pub mod ldy;

pub mod jsr;
pub mod rts;

pub mod jmp;

pub mod adc;

pub mod sta;
pub mod stx;
pub mod sty;

pub mod and;
pub mod eor;
pub mod ora;

pub mod cl;

// Opcodes
pub const INS_LDA_IM: Byte = 0xA9;
pub const INS_LDA_ZP: Byte = 0xA5;
pub const INS_LDA_ZPX: Byte = 0xB5;
pub const INS_LDA_ABS: Byte = 0xAD;
pub const INS_LDA_ABSX: Byte = 0xBD;
pub const INS_LDA_ABSY: Byte = 0xB9;
pub const INS_LDA_INDX: Byte = 0xA1;
pub const INS_LDA_INDY: Byte = 0xB1;

pub const INS_LDX_IM: Byte = 0xA2;
pub const INS_LDX_ZP: Byte = 0xA6;
pub const INS_LDX_ZPY: Byte = 0xB6;
pub const INS_LDX_ABS: Byte = 0xAE;
pub const INS_LDX_ABSY: Byte = 0xBE;

pub const INS_LDY_IM: Byte = 0xA0;
pub const INS_LDY_ZP: Byte = 0xA4;
pub const INS_LDY_ZPX: Byte = 0xB4;
pub const INS_LDY_ABS: Byte = 0xAC;
pub const INS_LDY_ABSX: Byte = 0xBC;

pub const INS_JSR: Byte = 0x20;
pub const INS_RTS: Byte = 0x60;

pub const INS_JMP_ABS: Byte = 0x4C;
pub const INS_JMP_IND: Byte = 0x6C;

pub const INS_ADC_IM: Byte = 0x69;
pub const INS_ADC_ZP: Byte = 0x65;
pub const INS_ADC_ZPX: Byte = 0x75;
pub const INS_ADC_ABS: Byte = 0x6D;
pub const INS_ADC_ABSX: Byte = 0x7D;
pub const INS_ADC_ABSY: Byte = 0x79;
pub const INS_ADC_INDX: Byte = 0x61;
pub const INS_ADC_INDY: Byte = 0x71;

pub const INS_STA_ZP: Byte = 0x85;
pub const INS_STA_ZPX: Byte = 0x95;
pub const INS_STA_ABS: Byte = 0x8D;
pub const INS_STA_ABSX: Byte = 0x9D;
pub const INS_STA_ABSY: Byte = 0x99;
pub const INS_STA_INDX: Byte = 0x81;
pub const INS_STA_INDY: Byte = 0x91;

pub const INS_STY_ZP: Byte = 0x84;
pub const INS_STY_ZPX: Byte = 0x94;
pub const INS_STY_ABS: Byte = 0x8C;

pub const INS_STX_ZP: Byte = 0x86;
pub const INS_STX_ZPY: Byte = 0x96;
pub const INS_STX_ABS: Byte = 0x8E;

pub const INS_AND_IM: Byte = 0x29;
pub const INS_AND_ZP: Byte = 0x25;
pub const INS_AND_ZPX: Byte = 0x35;
pub const INS_AND_ABS: Byte = 0x2D;
pub const INS_AND_ABSX: Byte = 0x3D;
pub const INS_AND_ABSY: Byte = 0x39;
pub const INS_AND_INDX: Byte = 0x21;
pub const INS_AND_INDY: Byte = 0x31;

pub const INS_EOR_IM: Byte = 0x49;
pub const INS_EOR_ZP: Byte = 0x45;
pub const INS_EOR_ZPX: Byte = 0x55;
pub const INS_EOR_ABS: Byte = 0x4D;
pub const INS_EOR_ABSX: Byte = 0x5D;
pub const INS_EOR_ABSY: Byte = 0x59;
pub const INS_EOR_INDX: Byte = 0x41;
pub const INS_EOR_INDY: Byte = 0x51;

pub const INS_ORA_IM: Byte = 0x09;
pub const INS_ORA_ZP: Byte = 0x05;
pub const INS_ORA_ZPX: Byte = 0x15;
pub const INS_ORA_ABS: Byte = 0x0D;
pub const INS_ORA_ABSX: Byte = 0x1D;
pub const INS_ORA_ABSY: Byte = 0x19;
pub const INS_ORA_INDX: Byte = 0x01;
pub const INS_ORA_INDY: Byte = 0x11;

pub const INS_CLC: Byte = 0x18;

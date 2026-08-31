use crate::Byte;

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
pub const INS_LDY_ZPY: Byte = 0xB4;
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

pub const INS_STA_ABS: Byte = 0x8D;

pub const INS_CLC_I: Byte = 0x18;

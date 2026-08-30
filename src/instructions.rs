use crate::Byte;

// Opcodes
pub const INS_LDA_IM: Byte = 0xA9;
pub const INS_LDA_ZP: Byte = 0xA5;
pub const INS_LDA_ZPX: Byte = 0xB5;

pub const INS_JSR: Byte = 0x20;
pub const INS_RTS: Byte = 0x60;

pub const INS_ADC_IM: Byte = 0x69;
pub const INS_ADC_ZP: Byte = 0x65;
pub const INS_ADC_ZPX: Byte = 0x75;

pub const INS_STA_ABS: Byte = 0x8D;

pub const INS_CLC_I: Byte = 0x18;

use crate::instructions::rts::rts;
use crate::{Byte, Word};
use crate::memory::MEM;
use crate::instructions::*;

// Status flags
const C: Byte = 0b0000_0001;
const Z: Byte = 0b0000_0010;
const I: Byte = 0b0000_0100;
const D: Byte = 0b0000_1000;
const B: Byte = 0b0001_0000;
const V: Byte = 0b0100_0000;
const N: Byte = 0b1000_0000;

pub struct CPU {
    pub PC: Word, // Program Counter
    pub SP: Byte, // Strack Pointer

    pub A: Byte, //Registers
    pub X: Byte, //Registers
    pub Y: Byte, //Registers

    pub PS: Byte, // Process Status
}

impl CPU {
    pub fn new() -> Self {
        Self {
            PC: 0,
            SP: 0,
            A: 0,
            X: 0,
            Y: 0,
            PS: 0,
        }
    }
}

impl CPU {
    // Reset CPU
    pub fn reset(&mut self, memory: &mut MEM) {
        self.PC = 0x8000;
        self.SP = 0xFF; 

        self.A = 0; 
        self.X = 0;
        self.Y = 0;

        self.PS = 0; 

        memory.initialize();
    }

    // Set status functions
    pub fn set_zn_status(&mut self) {
        if self.A == 0 {
            self.PS |= Z;
        } else {
            self.PS &= !Z;
        }

        if self.A & 0b10000000 > 0 {
            self.PS |= N;
        } else {
            self.PS &= !N;
        }
    }

    pub fn adc_set_status(&mut self, value: u8, carry: u8) -> u8 {
        let sum = self.A as u16 + value as u16 + carry as u16;
        let result = sum as u8;

        if result == 0 {
            self.PS |= Z;
        } else {
            self.PS &= !Z;
        }

        if result & 0b10000000 > 0 {
            self.PS |= N;
        } else {
            self.PS &= !N;
        }
        
        if ((self.A ^ value) & 0x80) == 0 && ((self.A ^ result) & 0x80) != 0{
            self.PS |= V;
        } else {
            self.PS &= !V;
        }

        if sum > 0xFF { 
            self.PS |= C; 
        } else { 
            self.PS &= !C; 
        }

        return result;
    }

    pub fn cl_set_status(&mut self, flag: Byte) {
        match flag {
            C => {
                self.PS &= !C;
            }

            D => {
                self.PS &= !D;
            }

            I => {
                self.PS &= !I;
            }

            V => {
                self.PS &= !V;
            }

            _ => {
                panic!("Invalid flag passed: {}", flag);
            }
        }
    }

    // Read/Fetch Byte
    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &MEM) -> Byte {
        let data: Byte = memory.DATA[self.PC as usize];
        self.PC += 1; 

        *cycles -= 1;
        return data;
    }

    pub fn read_byte(&mut self, cycles: &mut u32, address: Word, memory: &MEM) -> Byte {
        let data: Byte = memory.DATA[address as usize];
        *cycles -= 1;
        return data;
    }
    
    // Read/Fetch Word
    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &MEM) -> Word {
        let mut data: Word = memory[self.PC as usize] as Word;
        self.PC += 1;

        data |= (memory[self.PC as usize] as Word) << 8;
        self.PC += 1;

        *cycles -= 2;
        return data;
    }

     pub fn read_word(&mut self, cycles: &mut u32, address: Word, memory: &MEM) -> Word {
        let low_byte: Word = self.read_byte(cycles, address, memory) as Word;
        let high_byte: Word = self.read_byte(cycles, address + 1, memory) as Word;

        return low_byte | (high_byte << 8);
    }
    
    // Push the stack pointer to a stack
    pub fn push_sp_to_stack(&mut self, cycles: &mut u32, memory: &mut MEM) {
        self.write_word(0x0100 | self.SP as Word - 1, self.PC - 1, cycles, memory);
        self.SP -= 2;
    }
    

    pub fn pop_word_from_stack(&mut self, cycles: &mut u32, memory: &mut MEM) -> Word {
        let value: Word = self.read_word(cycles, 0x0100 | self.SP as Word + 1, memory);
        self.SP += 2;
        *cycles -= 1;
        return value;
    }

    // Write a word to memory
    pub fn write_word(&mut self, address: Word, data: Word, cycles: &mut u32, memory: &mut MEM) {
        memory.DATA[address as usize]         = (data & 0x00FF) as Byte;
        memory.DATA[(address + 1) as usize]   = (data >> 8) as Byte;

        *cycles -= 2; 
    }
    
    // Write a byte to memory
    pub fn write_byte(&mut self, address: Word, data: Byte, cycles: &mut u32, memory: &mut MEM) {
        memory.DATA[address as usize] = data;

        *cycles -= 1;
    }

    //Bitwise AND
    pub fn bitwise_and(&mut self, accumulator: Byte, value: Byte) -> Byte {
        return accumulator & value;
    }

    // Execute code on the CPU
    pub fn execute(&mut self, mut cycles: u32, memory: &mut MEM) {
        while cycles > 0 {
            println!("Cycles remaining: {}", cycles);
            let ins: Byte = self.fetch_byte( &mut cycles, memory );
            match ins {

                //ADC
                INS_ADC_IM => {
                    adc::adc_im(self, &mut cycles, memory, C); 
                }

                INS_ADC_ZP => {
                    adc::adc_zp(self, &mut cycles, memory, C); 
                }

                INS_ADC_ZPX => {
                    adc::adc_zpx(self, &mut cycles, memory, C); 
                }

                INS_ADC_ABS => {
                    adc::adc_abs(self, &mut cycles, memory, C); 
                }

                INS_ADC_ABSX => {
                    adc::adc_absx(self, &mut cycles, memory, C); 
                }

                INS_ADC_ABSY => {
                    adc::adc_absy(self, &mut cycles, memory, C); 
                }

                INS_ADC_INDX => {
                    adc::adc_indx(self, &mut cycles, memory, C);
                }

                INS_ADC_INDY => {
                    adc::adc_indy(self, &mut cycles, memory, C); 
                }

                //LDA
                INS_LDA_IM => {
                    lda::lda_im(self, &mut cycles, memory); 
                }

                INS_LDA_ZP => {
                    lda::lda_zp(self, &mut cycles, memory);
                }

                INS_LDA_ZPX => {
                    lda::lda_zpx(self, &mut cycles, memory); 
                }

                INS_LDA_ABS => {
                    lda::lda_abs(self, &mut cycles, memory); 
                }

                INS_LDA_ABSX => {
                    lda::lda_absx(self, &mut cycles, memory); 
                }

                INS_LDA_ABSY => {
                    lda::lda_absy(self, &mut cycles, memory); 
                }

                INS_LDA_INDX => {
                    lda::lda_indx(self, &mut cycles, memory);
                }

                INS_LDA_INDY => {
                    lda::lda_indy(self, &mut cycles, memory); 
                }
                
                //LDX
                INS_LDX_IM => {
                    ldx::ldx_im(self, &mut cycles, memory); 
                }

                INS_LDX_ZP => {
                    ldx::ldx_zp(self, &mut cycles, memory); 
                }
                
                INS_LDX_ZPY => {
                    ldx::ldx_zpy(self, &mut cycles, memory); 
                }

                INS_LDX_ABS => {
                    ldx::ldx_abs(self, &mut cycles, memory); 
                }

                INS_LDX_ABSY => {
                    ldx::ldx_absy(self, &mut cycles, memory); 
                }

                //LDY
                INS_LDY_IM => {
                    ldy::ldy_im(self, &mut cycles, memory);
                }

                INS_LDY_ZP => {
                    ldy::ldy_zp(self, &mut cycles, memory); 
                }
                
                INS_LDY_ZPX => {
                    ldy::ldy_zpx(self, &mut cycles, memory); 
                }

                INS_LDY_ABS => {
                    ldy::ldy_abs(self, &mut cycles, memory); 
                }

                INS_LDY_ABSX => {
                    ldy::ldy_absx(self, &mut cycles, memory); 
                }

                //STA
                INS_STA_ZP => {
                    sta::sta_zp(self, &mut cycles, memory); 
                }

                INS_STA_ZPX => {
                    sta::sta_zpx(self, &mut cycles, memory); 
                }

                INS_STA_ABS => {
                    sta::sta_abs(self, &mut cycles, memory);
                }

                INS_STA_ABSX => {
                    sta::sta_absx(self, &mut cycles, memory); 
                }

                INS_STA_ABSY => {
                    sta::sta_absy(self, &mut cycles, memory); 
                }

                INS_STA_INDX => {
                    sta::sta_indx(self, &mut cycles, memory); 
                }

                INS_STA_INDY => {
                    sta::sta_indy(self, &mut cycles, memory); 
                }

                //STY
                INS_STY_ZP => {
                    sty::sty_zp(self, &mut cycles, memory); 
                }

                INS_STY_ZPX => {
                    sty::sty_zpx(self, &mut cycles, memory); 
                }

                INS_STY_ABS => {
                    sty::sty_abs(self, &mut cycles, memory);
                }

                //STX
                INS_STX_ZP => {
                    stx::stx_zp(self, &mut cycles, memory);  
                }

                INS_STX_ZPY => {
                    stx::stx_zpy(self, &mut cycles, memory); 
                }

                INS_STX_ABS => {
                    stx::stx_abs(self, &mut cycles, memory); 
                }

                //AND
                INS_AND_IM => {
                    and::and_im(self, &mut cycles, memory);
                }

                INS_AND_ZP => {
                    and::and_zp(self, &mut cycles, memory); 
                }

                INS_AND_ZPX => {
                    and::and_zpx(self, &mut cycles, memory); 
                }

                INS_AND_ABS => {
                    and::and_abs(self, &mut cycles, memory); 
                }

                INS_AND_ABSX => {
                    and::and_absx(self, &mut cycles, memory); 
                }

                INS_AND_ABSY => {
                    and::and_absy(self, &mut cycles, memory); 
                }

                INS_AND_INDX => {
                    and::and_indx(self, &mut cycles, memory); 
                }

                INS_AND_INDY => {
                    and::and_indy(self, &mut cycles, memory); 
                }

                //JSR/RTS
                INS_JSR => {
                    jsr::jsr(self, &mut cycles, memory); 
                }

                INS_RTS => {
                    rts::rts(self, &mut cycles, memory);

                    dump_memory(memory, 0x0000, 0xFFFF);
                    self.debug();
                }

                //JMP
                INS_JMP_ABS => {
                    jmp::jmp_abs(self, &mut cycles, memory);
                }

                INS_JMP_IND => {
                    jmp::jmp_ind(self, &mut cycles, memory); 
                }

                //CL
                INS_CLC => {
                    cl::cl(self, &mut cycles, memory, C);          
                }

                _ => {
                    println!("Instruction not handled {:02X}", ins);
                    break;
                }
            }

            println!("Cycles remaining: {}", cycles);
        } 
    }

    pub fn debug(&self) {
        println!("\n--------CPU--------");
        println!("PC: {:04X}", self.PC);
        println!("SP: {:02X}", self.SP);
        println!("A:  {:02X}", self.A);
        println!("X:  {:02X}", self.X);
        println!("Y:  {:02X}", self.Y);
        println!("PS: {:02X}", self.SP);

        println!("\n-------FLAGS-------");
        println!("C: {}", self.PS & C != 0);
        println!("Z: {}", self.PS & Z != 0);
        println!("I: {}", self.PS & I != 0);
        println!("D: {}", self.PS & D != 0);
        println!("B: {}", self.PS & B != 0);
        println!("V: {}", self.PS & V != 0);
        println!("N: {}", self.PS & N != 0);
    }
}

pub fn dump_memory(memory: &MEM, start: usize, end: usize) {
    for address in start..end {
        if (address - start) % 16 == 0 {
            print!("{:04X}: ", address);
        }

        print!("{:02X} ", memory[address]);

        if (address - start) % 16 == 15 {
            println!();
        }
    }
}

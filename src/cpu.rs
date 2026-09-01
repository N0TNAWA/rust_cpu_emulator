use std::path::absolute;

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
        self.PC = 0xFFFC;
        self.SP = 0xFF; 

        self.A = 0; 
        self.X = 0;
        self.Y = 0;

        self.PS = 0; 

        memory.initialize();
    }

    // Set status functions
    fn ld_set_status(&mut self) {
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

    fn adc_set_status(&mut self, value: u8, carry: u8) {
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

        self.A = result;
    }

    fn cl_set_status(&mut self, flag: Byte) {
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
    fn fetch_byte(&mut self, cycles: &mut u32, memory: &MEM) -> Byte {
        let data: Byte = memory.DATA[self.PC as usize];
        self.PC += 1; 

        *cycles -= 1;
        return data;
    }

    fn read_byte(&mut self, cycles: &mut u32, address: Word, memory: &MEM) -> Byte {
        let data: Byte = memory.DATA[address as usize];
        *cycles -= 1;
        return data;
    }

    fn fetch_word(&mut self, cycles: &mut u32, memory: &MEM) -> Word {
        let mut data: Word = memory[self.PC as usize] as Word;
        self.PC += 1;

        data |= (memory[self.PC as usize] as Word) << 8;
        self.PC += 1;

        *cycles -= 2;
        return data;
    }

    fn read_word(&mut self, cycles: &mut u32, address: Word, memory: &MEM) -> Word {
        let low_byte: Word = self.read_byte(cycles, address, memory) as Word;
        let high_byte: Word = self.read_byte(cycles, address + 1, memory) as Word;

        return low_byte | (high_byte << 8);
    }

    fn push_sp_to_stack(&mut self, cycles: &mut u32, memory: &mut MEM) {
        self.write_word(0x0100 | self.SP as Word - 1, self.PC - 1, cycles, memory);
        self.SP -= 2;
    }

    fn pop_word_from_stack(&mut self, cycles: &mut u32, memory: &mut MEM) -> Word {
        let value: Word = self.read_word(cycles, 0x0100 | self.SP as Word + 1, memory);
        self.SP += 2;
        *cycles -= 1;
        return value;
    }


    fn write_word(&mut self, address: Word, data: Word, cycles: &mut u32, memory: &mut MEM) {
        memory.DATA[address as usize]         = (data & 0x00FF) as Byte;
        memory.DATA[(address + 1) as usize]   = (data >> 8) as Byte;

        *cycles -= 2; 
    }

    fn write_byte(&mut self, address: Word, data: Byte, cycles: &mut u32, memory: &mut MEM) {
        memory.DATA[address as usize] = data;

        *cycles -= 1;
    }


    // Execute code on the CPU
    pub fn execute(&mut self, mut cycles: u32, memory: &mut MEM) {
        while cycles > 0 {
            println!("Cycles remaining: {}", cycles);
            let ins: Byte = self.fetch_byte( &mut cycles, memory );
            match ins {

                //ADC
                INS_ADC_IM => {
                    let value: Byte = self.fetch_byte( &mut cycles, memory );
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    self.adc_set_status(value, carry); 
                }

                INS_ADC_ZP => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    let value = self.read_byte(&mut cycles, zero_page_address as Word, memory);
                    self.adc_set_status(value, carry);
                }

                INS_ADC_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    let carry: Byte = if self.PS & C != 0 {1} else {0};
                    zero_page_address = zero_page_address.wrapping_add(self.X);

                    cycles -= 1;

                    let value = self.read_byte(&mut cycles, zero_page_address as Word, memory);
                    self.adc_set_status(value, carry);  
                }

                INS_ADC_ABS => {
                    let absolute_address: Word = self.fetch_word(&mut cycles, memory);
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    let value = self.read_byte(&mut cycles, absolute_address, memory);
                    self.adc_set_status(value, carry);
                }

                INS_ADC_ABSX => {
                    let absolute_address: Word = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address.wrapping_add(self.X as Word);

                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    if (absolute_address & 0xFF00) != (absolute_address_x & 0xFF00) {
                        cycles -= 1;
                    }

                    let value = self.read_byte(&mut cycles, absolute_address_x, memory);
                    self.adc_set_status(value, carry);  
                }

                INS_ADC_ABSY => {
                    let absolute_address: Word = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address.wrapping_add(self.Y as Word);
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    if (absolute_address & 0xFF00) != (absolute_address_y & 0xFF00) {
                        cycles -= 1;
                    }

                    let value = self.read_byte(&mut cycles, absolute_address_y, memory);
                    self.adc_set_status(value, carry);
                }

                INS_ADC_INDX => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address = zero_page_address.wrapping_add(self.X);
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    cycles -= 1;

                    let effective_address: Word = self.read_word(&mut cycles, zero_page_address as Word, memory);
                    let value = self.read_byte(&mut cycles, effective_address, memory);

                    self.adc_set_status(value, carry);
                }

                INS_ADC_INDY => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let effective_address: Word = self.read_word(&mut cycles, zero_page_address as Word, memory);
                    let effective_address_y = effective_address.wrapping_add(self.Y as Word);
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    let value = self.read_byte(&mut cycles, effective_address_y, memory);

                    if (effective_address & 0xFF00) != (effective_address_y & 0xFF00) {
                        cycles -= 1;
                    }

                    self.adc_set_status(value, carry);
                }

                //LDA
                INS_LDA_IM => {
                    let value: Byte = self.fetch_byte( &mut cycles, memory );
                    self.A = value;
                    
                    self.ld_set_status();

                    dump_memory(memory, 0x0000, 0xFFFF);
                    self.debug();
                }

                INS_LDA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    self.A = self.read_byte( &mut cycles, zero_page_address as Word, memory);

                    self.ld_set_status();
                }

                INS_LDA_ZPX => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    let zero_page_address = zero_page_address.wrapping_add(self.X);

                    cycles -= 1;

                    self.A = self.read_byte( &mut cycles, zero_page_address as Word, memory);
                    self.ld_set_status();
                }

                INS_LDA_ABS => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    self.A = self.read_byte(&mut cycles, absolute_address, memory);

                    self.ld_set_status();

                    self.debug();
                }

                INS_LDA_ABSX => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    let absolute_address_x = absolute_address.wrapping_add(self.X as Word);
                    self.A = self.read_byte(&mut cycles, absolute_address_x, memory); 

                    if (absolute_address & 0xFF00) != (absolute_address_x & 0xFF00) {
                        cycles -= 1;
                    }

                    self.ld_set_status();

                    self.debug();
                }

                INS_LDA_ABSY => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    let absolute_address_y = absolute_address.wrapping_add(self.Y as Word);
                    self.A = self.read_byte(&mut cycles, absolute_address_y, memory); 

                    if (absolute_address & 0xFF00) != (absolute_address_y & 0xFF00) {
                        cycles -= 1;
                    }

                    self.ld_set_status();

                    self.debug();
                }

                INS_LDA_INDX => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address = zero_page_address.wrapping_add(self.X);

                    cycles -= 1;

                    let effective_address: Word = self.read_word(&mut cycles, zero_page_address as Word, memory);
                    self.A = self.read_byte(&mut cycles, effective_address, memory);

                    self.ld_set_status();
                }

                INS_LDA_INDY => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let effective_address: Word = self.read_word(&mut cycles, zero_page_address as Word, memory);
                    let effective_address_y = effective_address.wrapping_add(self.Y as Word);

                    self.A = self.read_byte(&mut cycles, effective_address_y, memory);

                    if (effective_address & 0xFF00) != (effective_address_y & 0xFF00) {
                        cycles -= 1;
                    }

                    self.ld_set_status();

                    self.debug();
                }
                
                //LDX
                INS_LDX_IM => {
                    let value: Byte = self.fetch_byte( &mut cycles, memory );
                    self.X = value;
                    
                    self.ld_set_status();
                }

                INS_LDX_ZP => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    self.X = self.read_byte( &mut cycles, zero_page_address as Word, memory);

                    self.ld_set_status();
                }
                
                INS_LDX_ZPY => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    let zero_page_address = zero_page_address.wrapping_add(self.Y);

                    cycles -= 1;

                    self.X = self.read_byte( &mut cycles, zero_page_address as Word, memory);
                    self.ld_set_status();
                }

                INS_LDX_ABS => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    self.X = self.read_byte(&mut cycles, absolute_address, memory);

                    self.ld_set_status();

                    self.debug();
                }

                INS_LDX_ABSY => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    let absolute_address_y = absolute_address.wrapping_add(self.Y as Word);
                    self.X = self.read_byte(&mut cycles, absolute_address_y, memory); 

                    if (absolute_address & 0xFF00) != (absolute_address_y & 0xFF00) {
                        cycles -= 1;
                    }

                    self.ld_set_status();

                    self.debug();
                }

                //LDY
                INS_LDY_IM => {
                    let value: Byte = self.fetch_byte( &mut cycles, memory );
                    self.Y = value;
                    
                    self.ld_set_status();
                }

                INS_LDY_ZP => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    self.Y = self.read_byte( &mut cycles, zero_page_address as Word, memory);

                    self.ld_set_status();
                }
                
                INS_LDY_ZPX => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    let zero_page_address = zero_page_address.wrapping_add(self.X);

                    cycles -= 1;

                    self.Y = self.read_byte( &mut cycles, zero_page_address as Word, memory);
                    self.ld_set_status();
                }

                INS_LDY_ABS => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    self.Y = self.read_byte(&mut cycles, absolute_address, memory);

                    self.ld_set_status();

                    self.debug();
                }

                INS_LDY_ABSX => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    let absolute_address_x = absolute_address.wrapping_add(self.X as Word);
                    self.Y = self.read_byte(&mut cycles, absolute_address_x, memory); 

                    if (absolute_address & 0xFF00) != (absolute_address_x & 0xFF00) {
                        cycles -= 1;
                    }

                    self.ld_set_status();

                    self.debug();
                }

                //STA
                INS_STA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);

                    self.write_byte(zero_page_address as Word, self.A, &mut cycles, memory);    
                }

                INS_STA_ZPX => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address = zero_page_address.wrapping_add(self.X);

                    self.write_byte(zero_page_address as Word, self.A, &mut cycles, memory);
                }

                INS_STA_ABS => {
                    let absolute_address: Word = self.fetch_word( &mut cycles, memory );
                    
                    self.write_byte(absolute_address, self.A, &mut cycles, memory);
                }

                INS_STA_ABSX => {
                    let absolute_address: Word = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address.wrapping_add(self.X as Word);

                    self.write_byte(absolute_address_x, self.A, &mut cycles, memory);
                }

                INS_STA_ABSY => {
                    let absolute_address: Word = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address.wrapping_add(self.Y as Word);

                    self.write_byte(absolute_address_y, self.A, &mut cycles, memory);
                }

                INS_STA_INDX => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address = zero_page_address.wrapping_add(self.X);

                    cycles -= 1;

                    let effective_address: Word = self.read_word(&mut cycles, zero_page_address as Word, memory);

                    self.write_byte(effective_address, self.A, &mut cycles, memory);
                }

                INS_STA_INDY => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let effective_address: Word = self.read_word(&mut cycles, zero_page_address as Word, memory);
                    let effective_address_y = effective_address.wrapping_add(self.Y as Word);

                    self.write_byte(effective_address_y, self.A, &mut cycles, memory);
                }

                //JSR/RTS
                INS_JSR => {
                    let sub_address: Word = self.fetch_word(&mut cycles, memory);

                    self.push_sp_to_stack(&mut cycles, memory);
                    self.PC = sub_address;

                    cycles -= 1;
                }

                INS_RTS => {
                    let return_address: Word = self.pop_word_from_stack(&mut cycles, memory);
                    self.PC = return_address + 1;

                    cycles -= 2;
                }

                //JMP
                INS_JMP_ABS => {
                    let jump_address: Word = self.fetch_word(&mut cycles, memory);
                    self.PC = jump_address;
                }

                INS_JMP_IND => {
                    let pointer: Word = self.fetch_word(&mut cycles, memory);
                    let effective_address: Word = self.read_word(&mut cycles, pointer, memory);

                    self.PC = effective_address;
                }

                //CL
                INS_CLC_I => {
                    self.cl_set_status(C);
                    
                    cycles -= 1;             
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

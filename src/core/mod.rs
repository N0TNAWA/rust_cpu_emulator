use std::ops::{Index, IndexMut};

// Types
type Byte = u8;
type Word = u16;

// Status flags
const C: Byte = 0b0000_0001;
const Z: Byte = 0b0000_0010;
const I: Byte = 0b0000_0100;
const D: Byte = 0b0000_1000;
const B: Byte = 0b0001_0000;
const V: Byte = 0b0010_0000;
const N: Byte = 0b0100_0000;

// Opcodes
pub const INS_LDA_IM: Byte = 0xA9;
pub const INS_LDA_ZP: Byte = 0xA5;
pub const INS_LDA_ZPX: Byte = 0xB5;

pub const INS_JSR: Byte = 0x20;

pub const INS_ADC_IM: Byte = 0x69;

pub const INS_STA_ABS: Byte = 0x8D; 

// Size allocation
pub const MAX_MEM: usize = 1024*64;

pub struct MEM {
    pub DATA: [Byte; MAX_MEM], 
}

// Memory initialization
impl MEM {
    pub fn initialize(&mut self) {
        for i in 0..MAX_MEM {
           self.DATA[i] = 0; 
        }
    }
}

// Reading 1 byte
impl Index<usize> for MEM {
    type Output = Byte;

    fn index(&self, address: usize) -> &Self::Output {
        &self.DATA[address]
    }
}

// Writing 1 byte
impl IndexMut<usize> for MEM {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.DATA[address]
    }
}

impl MEM {
    fn write_word(&mut self, address: u32, data: Word, cycles: &mut u32) {
        self.DATA[address as usize]         = (data & 0x00FF) as Byte;
        self.DATA[(address + 1) as usize]   = (data >> 8) as Byte;
        
        *cycles -= 2;
    }

    fn write_byte(&mut self, address: u32, data: Byte, cycles: &mut u32) {
        self.DATA[address as usize] = data;

        *cycles -= 1;
    }
}

pub struct CPU {
    pub PC: Word, // Program Counter
    pub SP: Word, // Strack Pointer

    pub A: Byte, //Registers
    pub X: Byte, //Registers
    pub Y: Byte, //Registers

    pub PS: Byte, // Process Status
}

impl CPU {
    // Reset CPU
    pub fn reset(&mut self, memory: &mut MEM) {
        self.PC = 0xFFFC;
        self.SP = 0x0100; 

        self.A = 0; 
        self.X = 0;
        self.Y = 0;

        self.PS = 0; 

        memory.initialize();
    }

    // Set status functions
    fn lda_set_status(&mut self) {
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
    
    // Read/Fetch Byte
    fn fetch_byte(&mut self, cycles: &mut u32, memory: &mut MEM) -> Byte {
        let data: Byte = memory.DATA[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;
        return data;
    }

    fn read_byte(&mut self, cycles: &mut u32, address: Byte, memory: &mut MEM) -> Byte {
        let data: Byte = memory.DATA[address as usize];
        *cycles -= 1;
        return data;
    }

    fn fetch_word(&mut self, cycles: &mut u32, memory: &mut MEM) -> Word {
        let mut data: Word = memory[self.PC as usize] as Word;
        self.PC += 1;

        data |= (memory[self.PC as usize] as Word) << 8;
        self.PC += 1;

        *cycles -= 2;
        return data;
    }

    // Execute code on the CPU
    pub fn execute(&mut self, mut cycles: u32,memory: &mut MEM) {
        while cycles > 0 {
            let ins: Byte = self.fetch_byte( &mut cycles, memory );
            match ins {

                //ADC
                INS_ADC_IM => {
                    let value: Byte = self.fetch_byte( &mut cycles, memory );
                    let carry: Byte = if self.PS & C != 0 {1} else {0};

                    self.adc_set_status(value, carry); 

                    println!("PC: {:04X}", self.PC);
                    println!("SP: {:04X}", self.SP);
                    println!("A:  {:02X}", self.A);
                    println!("X:  {:02X}", self.X);
                    println!("Y:  {:02X}", self.Y);
                    println!("PS: {:02X}", self.PS);
                }

                //LDA
                INS_LDA_IM => {
                    let value: Byte = self.fetch_byte( &mut cycles, memory );
                    self.A = value;
                    
                    self.lda_set_status(); 
                }

                INS_LDA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    self.A = self.read_byte( &mut cycles, zero_page_address, memory);

                    self.lda_set_status();
                }

                INS_LDA_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte( &mut cycles, memory );
                    zero_page_address += self.X;

                    cycles -= 1;

                    self.A = self.read_byte( &mut cycles, zero_page_address, memory);
                    self.lda_set_status();
                }

                //JSR
                INS_JSR => {
                    let sub_address: Word = self.fetch_word( &mut cycles, memory );
                    memory.write_word(self.SP as u32, self.PC - 1, &mut cycles);

                    self.PC = sub_address;

                    cycles -= 1;
                }

                //STA
                INS_STA_ABS => {
                    let address: Word = self.fetch_word( &mut cycles, memory );

                    println!("\nPC: {:04X}", self.PC);
                    println!("SP: {:04X}", self.SP);
                    println!("A:  {:02X}", self.A);
                    println!("X:  {:02X}", self.X);
                    println!("Y:  {:02X}", self.Y);
                    println!("PS: {:02X}", self.PS);
                    
                    memory.write_byte(address as u32, self.A, &mut cycles);

                    dump_memory(&memory, 0x0200, 0x10000);
                }

                _ => {
                    println!("Instruction not handled {}", ins);
                    break;
                }
            }
        } 
    }
}

// Memory dump -debug-
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

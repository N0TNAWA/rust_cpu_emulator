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
const INS_LDA_IM: Byte = 0xA9;
const INS_LDA_ZP: Byte = 0xA5;
const INS_LDA_ZPX: Byte = 0xB5;
const INS_JSR: Byte = 0x20;

// Size allocation
const MAX_MEM: usize = 1024*64;

struct MEM {
    DATA: [Byte; MAX_MEM], 
}

// Memory initialization
impl MEM {
    fn initialize(&mut self) {
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
}

struct CPU {
    PC: Word, // Program Counter
    SP: Word, // Strack Pointer

    A: Byte, //Registers
    X: Byte, //Registers
    Y: Byte, //Registers

    PS: Byte, // Process Status
}

impl CPU {
    // Reset CPU
    fn reset(&mut self, memory: &mut MEM) {
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
    
    // Read/Write Byte
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
    fn execute(&mut self, mut cycles: u32,memory: &mut MEM) {
        while cycles > 0 {
            let ins: Byte = self.fetch_byte( &mut cycles, memory );
            match ins {
                INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
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

                INS_JSR => {
                    let sub_address: Word = self.fetch_word( &mut cycles, memory );
                    memory.write_word(self.SP as u32, self.PC - 1, &mut cycles);

                    self.PC = sub_address;

                    cycles -= 1;
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
fn dump_memory(memory: &MEM, start: usize, end: usize) {
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

fn main() {
    let mut mem = MEM {
        DATA: [0; MAX_MEM],
    };

    let mut cpu = CPU {
        PC: 0,
        SP: 0,

        A: 0,
        X: 0,
        Y: 0,

        PS: 0,
    };

    cpu.reset(&mut mem);

    println!("PC: {:04X}", cpu.PC);
    println!("SP: {:04X}", cpu.SP);
    println!("A:  {:02X}", cpu.A);
    println!("X:  {:02X}", cpu.X);
    println!("Y:  {:02X}", cpu.Y);
    println!("PS: {:02X}", cpu.PS);
    
    //inline a program - start
    mem.DATA[0xFFFC] = INS_JSR;
    mem.DATA[0xFFFD] = 0x42;
    mem.DATA[0xFFFE] = 0x42;
    mem.DATA[0x4242] = INS_LDA_IM;
    mem.DATA[0x4243] = 0x84;
    //inline a program - end
    
    dump_memory(&mem, 0x4242, 0x10000);

    cpu.execute(8, &mut mem);
}

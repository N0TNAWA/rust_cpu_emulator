use std::ops::{Index, IndexMut};

type Byte = u8;
type Word = u16;

const C: Byte = 0b0000_0001; //Status flags
const Z: Byte = 0b0000_0010; //Status flags
const I: Byte = 0b0000_0100; //Status flags
const D: Byte = 0b0000_1000; //Status flags
const B: Byte = 0b0001_0000; //Status flags
const V: Byte = 0b0010_0000; //Status flags
const N: Byte = 0b0100_0000; //Status flags

const INS_LDA_IM: Byte = 0xA9;

const MAX_MEM: usize = 1024*64;

struct MEM {
    DATA: [Byte; MAX_MEM], 
}

impl MEM {
    fn initialize(&mut self) {
        for i in 0..MAX_MEM {
           self.DATA[i] = 0; 
        }
    }
}

impl Index<usize> for MEM {
    type Output = Byte;

    fn index(&self, address: usize) -> &Self::Output {
        &self.DATA[address]
    }
}

impl IndexMut<usize> for MEM {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.DATA[address]
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
    fn reset(&mut self, memory: &mut MEM) {
        self.PC = 0xFFFC;
        self.SP = 0x0100; 

        self.A = 0; 
        self.X = 0;
        self.Y = 0;

        self.PS = 0; 

        memory.initialize();
    }

    fn fetch_byte(&mut self, cycles: &mut u32, memory: &mut MEM) -> Byte {
        let data: Byte = memory.DATA[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;
        return data;
    }

    fn execute(&mut self, mut cycles: u32, memory: &mut MEM) {
        while cycles > 0 {
            let ins: Byte = self.fetch_byte( &mut cycles, memory );
            match ins {
                INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.A = value;
                    
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
    mem.DATA[0xFFFC] = INS_LDA_IM;
    mem.DATA[0xFFFD] = 0x42;
    //inline a program - end
    
    dump_memory(&mem, 0xFFFC, 0x10000);

    cpu.execute(2, &mut mem);
}

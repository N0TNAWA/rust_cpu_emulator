use std::ops::{Index, IndexMut};
use crate::{Byte, Word, MAX_MEM};

pub struct MEM {
    pub DATA: [Byte; MAX_MEM], 
}

// Memory initialization
impl MEM {
    pub fn new() -> Self {
        Self {
            DATA: [0; MAX_MEM],
        }
    }

    pub fn initialize(&mut self) {
        self.DATA.fill(0);   
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

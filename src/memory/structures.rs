#![cfg(windows)]

use std::ops::{
    AddAssign, 
    SubAssign, 
    
    // binary assignments
    ShlAssign
};
use std::default::Default;

#[derive(Debug)]
pub struct Pointer {
    pub address: i32,
    pub addend: i32,
    pub offsets: Vec<i32>,
}

#[allow(dead_code)]
impl Default for Pointer{
    fn default() -> Self {
        Pointer {
            address: 0x0,
            addend: 0x0,
            offsets: Vec::new()
        }
    }
}

impl Pointer {
    // get functions
    #[allow(dead_code)]
    pub fn get(&self) -> i32 {
        let mut addr = 0x0;

        addr += self.address;
        addr += self.addend;
        
        for x in &self.offsets {
            addr += x;
        }

        addr
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> i32 {
        self.address
    }

    #[allow(dead_code)]
    pub fn get_offset(&self, _index: usize) -> i32 {
        self.offsets[_index]
    }

    #[allow(dead_code)]
    pub fn get_offsets(self) -> Vec<i32> {
        self.offsets
    }
}

// Operator Overrides
impl AddAssign<i32> for Pointer {
    fn add_assign(&mut self, int: i32) {
        *self = Self {
            address: self.address,
            addend: self.addend + int,
            offsets: self.offsets.to_vec()
        }
    }
}

impl SubAssign<i32> for Pointer {
    fn sub_assign(&mut self, int: i32) {
        *self = Self {
            address: self.address,
            addend: self.addend - int,
            offsets: self.offsets.to_vec()
        }
    }
}

impl ShlAssign<i32> for Pointer {
    fn shl_assign(&mut self, _offset: i32) {
        self.offsets.push(_offset);
    }
}
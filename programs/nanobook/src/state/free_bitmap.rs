use anchor_lang::prelude::*;
use bytemuck::{Zeroable, Pod};

#[derive(Debug, Copy, Clone, Zeroable, Pod, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct FreeBitmap {
    bits: u64, 
}

impl FreeBitmap {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, index: u8) {
        self.bits |= 1 << index;
    }

    pub fn clear(&mut self, index: u8) {
        self.bits &= !(1 << index);
    }

    pub fn is_set(&self, index: u8) -> bool {
        (self.bits & (1 << index)) != 0
    }

    pub fn find_first_zero(&self) -> Option<u8> {
        for i in 0..64 {
            if !self.is_set(i) {
                return Some(i);
            }
        }
        None
    }
}

use anchor_lang::prelude::*;
use bytemuck::{Zeroable, Pod};

pub enum Balance {
    Nano,
    Sol,
}

#[derive(Default, Zeroable, Pod, Copy, PartialEq)]
#[account]
#[repr(C)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub nano_balance: u64,
    pub sol_balance: u64,
    pub bump: u8,
    _padding: [u8; 7],
}

impl UserAccount {
    pub fn increment_balance(&mut self, balance: &Balance, delta: u64) {
        match balance {
            Balance::Nano => self.nano_balance += delta,
            Balance::Sol => self.sol_balance += delta,
        };
    }

    pub fn decrement_balance(&mut self, balance: &Balance, delta: u64) {
        match balance {
            Balance::Nano => self.nano_balance -= delta,
            Balance::Sol => self.sol_balance -= delta,
        };
    }
}
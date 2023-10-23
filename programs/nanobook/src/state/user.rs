use anchor_lang::prelude::*;
use bytemuck::{Zeroable, ZeroableInOption, Pod, PodInOption};
use crate::error::ErrorCode;

pub enum Balance {
    Nano,
    Sol,
}

const USER_MAP_SIZE: usize = 128;

#[account(zero_copy)]
pub struct UserMap {
    pub users: [Option<UserAccount>; USER_MAP_SIZE], // user cap just for the sake of making this work right
    pub last_index: u16,
    _padding: [u8; 6],
}


impl UserMap {
    pub fn add_user(&mut self, user_account: UserAccount) -> Result<()> {
        let mut index = self.hash_pubkey(&user_account.owner);
        let mut tries = 0;

        while self.users[index].is_some() && tries < USER_MAP_SIZE {
            if self.users[index].as_ref().unwrap().owner == user_account.owner {
                return Err(ErrorCode::UserAlreadyExists.into());
            }
            index = (index + 1) % USER_MAP_SIZE;
            tries += 1;
        }
        require!(tries < USER_MAP_SIZE, ErrorCode::UserMapFull);

        self.users[index] = Some(user_account);
        Ok(())
    }

    pub fn load_user(&mut self, owner: &Pubkey) -> Result<&mut UserAccount> {
        let mut index = self.hash_pubkey(owner);
        let mut found_index: Option<usize> = None;
        let mut tries = 0;
    
        while tries < USER_MAP_SIZE {
            if let Some(user_account) = &self.users[index] {
                if user_account.owner == *owner {
                    found_index = Some(index);
                    break;
                }
            }
            index = (index + 1) % USER_MAP_SIZE;
            tries += 1;
        }
    
        match found_index {
            Some(idx) => Ok(self.users[idx].as_mut().unwrap()),
            None => Err(ErrorCode::UserNotFound.into()),
        }
    }
    
    fn hash_pubkey(&self, pubkey: &Pubkey) -> usize {
        let bytes = pubkey.as_ref();
        let mut sum = 0;
        for &b in bytes.iter() {
            sum += b as usize;
        }
        sum % USER_MAP_SIZE
    }
}

#[derive(Default, Zeroable, Pod, Copy, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[repr(C)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub nano_balance: u64,
    pub sol_balance: u64,
}

unsafe impl ZeroableInOption for UserAccount {}

unsafe impl PodInOption for UserAccount {}

impl UserAccount {
    pub fn new(owner: Pubkey) -> Self {
        Self { owner, nano_balance: 0, sol_balance: 0 }
    }
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
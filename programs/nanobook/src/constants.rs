use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

#[constant]
pub const ORDER_BOOK_DEPTH: u8 = 128;
#[constant]
pub const ORDER_DUST_THRESHOLD: u64 = LAMPORTS_PER_SOL / 100;
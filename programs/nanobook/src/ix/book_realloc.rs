use anchor_lang::prelude::*;
use crate::state::Orderbook;

pub fn process_realloc(_ctx: Context<ReallocOrderbook>, _len: u16) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct ReallocOrderbook<'info> {
    #[account(
        mut,
        realloc = len as usize,
        realloc::zero = true,
        realloc::payer = payer,
    )]
    pub book: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}  
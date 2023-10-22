use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{state::UserAccount, token_utils::token_transfer};


pub fn process_deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {

    token_transfer(amt, &ctx.accounts.token_program, &ctx.accounts.from, &ctx.accounts.to, &ctx.accounts.authority)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        seeds = [
            authority.key.as_ref(),
            b"user",
        ],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub from: Account<'info, TokenAccount>,

    pub to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
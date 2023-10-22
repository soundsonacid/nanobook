use anchor_lang::prelude::*;
use anchor_spl::token::{spl_token::native_mint, Token, TokenAccount};

use crate::{state::{UserAccount, user::Balance}, token_utils::token_transfer};


pub fn process_deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
    token_transfer(amt, &ctx.accounts.token_program, &ctx.accounts.from, &ctx.accounts.to, &ctx.accounts.authority)?;

    {
        let user_account = &mut ctx.accounts.user_account;

        if ctx.accounts.from.mint == native_mint::ID {
            user_account.decrement_balance(Balance::Sol, amt);
        } else {
            user_account.decrement_balance(Balance::Nano, amt);
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [
            authority.key.as_ref(),
            b"user",
        ],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
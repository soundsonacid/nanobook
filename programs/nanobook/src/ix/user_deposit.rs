use anchor_lang::prelude::*;
use anchor_spl::token::{spl_token::native_mint, Token, TokenAccount};

use crate::{state::{UserMap, user::Balance}, token_utils::token_transfer};


pub fn process_deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
    token_transfer(amt, &ctx.accounts.token_program, &ctx.accounts.from, &ctx.accounts.to, &ctx.accounts.authority)?;

    {
        let usermap = &mut ctx.accounts.usermap.load_mut()?;

        let user_account = usermap.load_user(&ctx.accounts.authority.key())?;

        if ctx.accounts.from.mint == native_mint::ID {
            user_account.decrement_balance(&Balance::Sol, amt);
        } else {
            user_account.decrement_balance(&Balance::Nano, amt);
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [
            b"usermap"
        ],
        bump,
    )]
    pub usermap: AccountLoader<'info, UserMap>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>, // payer token account

    #[account(mut)]
    pub to: Account<'info, TokenAccount>, // orderbook token account

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
use anchor_lang::prelude::*;
use anchor_spl::token::{spl_token::native_mint, Token, TokenAccount};

use crate::{state::{UserAccount, user::Balance}, token_utils::token_transfer_signed};


pub fn process_withdrawal(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
    let seeds = &[
        b"user".as_ref(),
        &ctx.accounts.payer.key().to_bytes(),
        &[ctx.bumps.authority]
    ];
    let signer_seeds = &seeds[..];

    token_transfer_signed(amt, &ctx.accounts.token_program, &ctx.accounts.to, &ctx.accounts.from, &ctx.accounts.authority, signer_seeds)?;

    {
        let user_account = &mut ctx.accounts.authority;

        if ctx.accounts.from.mint == native_mint::ID {
            user_account.decrement_balance(Balance::Sol, amt)
        } else {
            user_account.decrement_balance(Balance::Nano, amt)
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [
            payer.key.as_ref(),
            b"user",
        ],
        bump
    )]
    pub authority: Account<'info, UserAccount>,

    pub from: Account<'info, TokenAccount>,

    pub to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
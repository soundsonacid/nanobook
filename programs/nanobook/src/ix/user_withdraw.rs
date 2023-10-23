use anchor_lang::prelude::*;
use anchor_spl::token::{spl_token::native_mint, Token, TokenAccount};

use crate::{state::{UserMap, user::Balance, Orderbook}, token_utils::token_transfer_signed, error::ErrorCode};


pub fn process_withdrawal(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
    let seeds = &[
        b"book".as_ref(),
        &[ctx.bumps.orderbook]
    ];
    let signer_seeds = &seeds[..];

    let usermap = &mut ctx.accounts.usermap.load_mut()?;

    let user_account = usermap.load_user(&ctx.accounts.payer.key())?;

    // Enforce against over-withdrawals
    if ctx.accounts.from.mint == native_mint::ID {
        require!(user_account.sol_balance >= amt, ErrorCode::Overdraft);
    } else {
        require!(user_account.nano_balance >= amt, ErrorCode::Overdraft);
    }

    token_transfer_signed(amt, &ctx.accounts.token_program, &ctx.accounts.to, &ctx.accounts.from, &ctx.accounts.orderbook, signer_seeds)?;

    {
        let user_account = usermap.load_user(&ctx.accounts.payer.key())?;

        if ctx.accounts.from.mint == native_mint::ID {
            user_account.decrement_balance(&Balance::Sol, amt)
        } else {
            user_account.decrement_balance(&Balance::Nano, amt)
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [
            b"usermap"
        ],
        bump,
    )]
    pub usermap: AccountLoader<'info, UserMap>,

    #[account(
        mut,
        seeds = [
            b"book",
        ],
        bump
    )]
    pub orderbook: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>, // orderbook token account

    #[account(mut)]
    pub to: Account<'info, TokenAccount>, // payer token account

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
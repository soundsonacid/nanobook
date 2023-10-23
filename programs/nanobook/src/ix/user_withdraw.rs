use anchor_lang::prelude::*;
use anchor_spl::token::{spl_token::native_mint, Token, TokenAccount};

use crate::{state::{UserAccount, user::Balance, Orderbook}, token_utils::token_transfer_signed, error::ErrorCode};


pub fn process_withdrawal(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
    let seeds = &[
        b"orderbook".as_ref(),
        &[ctx.bumps.orderbook]
    ];
    let signer_seeds = &seeds[..];

    // Enforce against over-withdrawals
    if ctx.accounts.from.mint == native_mint::ID {
        require!(ctx.accounts.user.sol_balance >= amt, ErrorCode::Overdraft);
    } else {
        require!(ctx.accounts.user.nano_balance >= amt, ErrorCode::Overdraft);
    }

    token_transfer_signed(amt, &ctx.accounts.token_program, &ctx.accounts.to, &ctx.accounts.from, &ctx.accounts.orderbook, signer_seeds)?;

    {
        let user_account = &mut ctx.accounts.user;

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
            payer.key.as_ref(),
            b"user",
        ],
        bump
    )]
    pub user: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [
            b"orderbook",
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
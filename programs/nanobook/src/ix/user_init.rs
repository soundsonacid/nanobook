use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{TokenAccount, Mint, Token}};
use crate::state::UserAccount;

pub fn process_initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    let bump = ctx.bumps.user_account;

    user.owner = ctx.accounts.payer.key();
    user.sol_balance = 0;
    user.nano_balance = 0;
    user.bump = bump;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [
            payer.key.as_ref(),
            b"user",
        ],
        bump,
        space = std::mem::size_of::<UserAccount>() + 8,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub nano_mint: Account<'info, Mint>,

    #[account(
        init, 
        payer = payer,
        associated_token::authority = user_account, 
        associated_token::mint = nano_mint
    )]
    pub nano_vault: Account<'info, TokenAccount>,

    pub sol_mint: Account<'info, Mint>,

    #[account(
        init, 
        payer = payer,
        associated_token::authority = user_account, 
        associated_token::mint = sol_mint
    )]
    pub sol_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
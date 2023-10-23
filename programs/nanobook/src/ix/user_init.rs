use anchor_lang::prelude::*;
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

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
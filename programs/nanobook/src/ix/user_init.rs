use anchor_lang::prelude::*;
use crate::state::user::{UserAccount, UserMap};

pub fn process_initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
    let usermap = &mut ctx.accounts.usermap.load_mut()?;

    let user_account = UserAccount::new(ctx.accounts.payer.key());

    usermap.add_user(user_account)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        mut,
        seeds = [
            b"usermap"
        ],
        bump,
    )]
    pub usermap: AccountLoader<'info, UserMap>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
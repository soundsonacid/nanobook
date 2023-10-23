use anchor_lang::prelude::*;
use crate::state::UserMap;

pub fn process_initialize_usermap(ctx: Context<InitializeUserMap>) -> Result<()> {
    let usermap = &mut ctx.accounts.usermap.load_init()?;
    usermap.last_index = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUserMap<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [
            b"usermap"
        ],
        bump,
        space = std::mem::size_of::<UserMap>() + 8,
    )]
    pub usermap: AccountLoader<'info, UserMap>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
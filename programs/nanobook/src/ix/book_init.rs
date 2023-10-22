use anchor_lang::prelude::*;
use crate::state::Orderbook;

pub fn process_initialize_orderbook(ctx: Context<InitializeOrderbook>) -> Result<()> {
    let book = &mut ctx.accounts.book;
    book.max_orders = 128;
    book.num_orders = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeOrderbook<'info> {
    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Orderbook>(),
    )]
    pub book: Account<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
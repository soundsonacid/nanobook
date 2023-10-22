use anchor_lang::prelude::*;
use crate::{state::Orderbook, constants::ORDER_BOOK_DEPTH};

pub fn process_initialize_orderbook(ctx: Context<InitializeOrderbook>) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;
    book.max_orders = ORDER_BOOK_DEPTH;
    book.num_orders = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeOrderbook<'info> {
    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Orderbook>() + 8,
    )]
    pub book: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
use anchor_lang::prelude::*;
use crate::{state::Orderbook, constants::ORDER_BOOK_DEPTH};

pub fn process_hydrate(ctx: Context<HydrateOrderbook>) -> Result<()> {
    let mut book = ctx.accounts.book.load_mut()?;
    let bump = ctx.bumps.book;
    book.buy_queue.max_orders = ORDER_BOOK_DEPTH;
    book.sell_queue.max_orders = ORDER_BOOK_DEPTH;
    book.buy_queue.num_orders = 0;
    book.sell_queue.num_orders = 0;
    book.bump = bump;
    Ok(())
}

#[derive(Accounts)]
pub struct HydrateOrderbook<'info> {
    #[account(
        mut,
        seeds = [
            b"ob",
        ],
        bump,
    )]
    pub book: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>
}
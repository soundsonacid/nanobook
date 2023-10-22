use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{Order, Orderbook, Side, MatchingEngine},
};

pub fn process_place_market_order(ctx: Context<PlaceMarketOrder>, quantity: u64, side: Side) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;
    let order = &mut ctx.accounts.order;

    if book.num_orders >= book.max_orders {
        return Err(ErrorCode::MaxOrdersReached.into());
    }

    book.last_order_id += 1;

    order.id = book.last_order_id;
    order.placer = ctx.accounts.payer.key();
    order.price = 0; 
    order.quantity = quantity;
    order.side = side;

    match side {
        Side::Buy => book.buy_queue.add_order(**order),
        Side::Sell => book.sell_queue.add_order(**order),
    };

    book.num_orders += 1;

    let mut matching_engine = MatchingEngine::new(book);
    matching_engine.match_market_order(&order)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PlaceMarketOrder<'info> {
    #[account(mut)]
    pub book: AccountLoader<'info, Orderbook>,
    
    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Order>() + 8,
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

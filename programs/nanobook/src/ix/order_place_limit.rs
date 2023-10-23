use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{UserAccount, Order, Orderbook, Side, MatchingEngine, Market},
};

pub fn process_place_limit_order(ctx: Context<PlaceLimitOrder>, price: u64, quantity: u64, side: Side, market: Market) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;
    let order = &mut ctx.accounts.order;

    match market {
        Market::SolNano => require!(ctx.accounts.placer.sol_balance >= quantity, ErrorCode::Overdraft),
        Market::NanoSol => require!(ctx.accounts.placer.nano_balance >= quantity, ErrorCode::Overdraft)
    };

    let mut queue = match side {
        Side::Buy => book.buy_queue,
        Side::Sell => book.sell_queue
    };

    require!(queue.num_orders < queue.max_orders, ErrorCode::MaxOrdersReached);

    book.last_order_id += 1;

    order.id = book.last_order_id;
    order.placer = *ctx.accounts.placer;
    order.price = price;
    order.quantity = quantity;
    order.side = side;

    queue.add_order(**order);

    queue.num_orders += 1;
    
    let mut matching_engine = MatchingEngine::new(book);
    matching_engine.match_limit_order(&order, &mut *ctx.accounts.placer, &market)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PlaceLimitOrder<'info> {
    #[account(
        seeds = [
            payer.key.as_ref(),
            b"user",
        ],
        bump
    )]
    pub placer: Account<'info, UserAccount>,

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
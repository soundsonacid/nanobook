use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{UserMap, Order, Orderbook, Side, MatchingEngine, Market},
};

pub fn process_place_limit_order(ctx: Context<PlaceLimitOrder>, price: u64, quantity: u64, side: Side, market: Market) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;
    let usermap = &mut ctx.accounts.usermap.load_mut()?;

    let mut user_account = usermap.load_user(&ctx.accounts.payer.key())?;

    match market {
        Market::SolNano => require!(user_account.sol_balance >= quantity, ErrorCode::Overdraft),
        Market::NanoSol => require!(user_account.nano_balance >= quantity, ErrorCode::Overdraft)
    };

    let mut queue = match side {
        Side::Buy => book.buy_queue,
        Side::Sell => book.sell_queue
    };

    require!(queue.num_orders < queue.max_orders, ErrorCode::MaxOrdersReached);

    book.last_order_id += 1;

    let order = Order::new(book.last_order_id, *user_account, price, quantity, side);

    queue.add_order(order);

    queue.num_orders += 1;
    
    let mut matching_engine = MatchingEngine::new(book);
    matching_engine.match_limit_order(&order, &mut user_account, &market)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PlaceLimitOrder<'info> {
    #[account(
        mut,
        seeds = [
            b"usermap"
        ],
        bump,
    )]
    pub usermap: AccountLoader<'info, UserMap>,


    #[account(mut)]
    pub book: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
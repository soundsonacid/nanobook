use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{UserMap, Order, Orderbook, Side},
};

pub fn process_cancel_order(ctx: Context<CancelOrder>, order_id: u64) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;


    // require!(order.placer == *ctx.accounts.placer, ErrorCode::CouldNotCancel);

    // let mut queue = match order.side {
    //     Side::Buy => book.buy_queue,
    //     Side::Sell => book.sell_queue
    // };

    // queue.num_orders -= 1;

    // queue.remove_order(order.id);
    
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(
        mut,
        seeds = [
            b"usermap"
        ],
        bump,
    )]
    pub usermap: AccountLoader<'info, UserMap>,

    // #[account(mut)]
    // pub order: Account<'info, Order>,

    #[account(mut)]
    pub book: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
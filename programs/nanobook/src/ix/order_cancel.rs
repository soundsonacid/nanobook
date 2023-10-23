use anchor_lang::prelude::*;
use crate::state::{UserMap, Orderbook, Side};

pub fn process_cancel_order(ctx: Context<CancelOrder>, order_id: u64, side: Side) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;

    let mut queue = match side {
        Side::Buy => book.buy_queue,
        Side::Sell => book.sell_queue
    };

    queue.remove_order(order_id);

    queue.num_orders -= 1;
    
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
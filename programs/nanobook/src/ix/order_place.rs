use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{Order, Orderbook, Side},
};

pub fn process_place_order(ctx: Context<PlaceOrder>, price: u64, quantity: u64, side: Side) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;
    let order = &mut ctx.accounts.order;

    if book.num_orders >= book.max_orders {
        return Err(ErrorCode::MaxOrdersReached.into());
    }

    book.num_orders += 1;
    book.last_order_id += 1;

    order.id = book.last_order_id;
    order.placer = ctx.accounts.payer.key();
    order.price = price;
    order.quantity = quantity;
    order.side = side;

    Ok(())
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
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
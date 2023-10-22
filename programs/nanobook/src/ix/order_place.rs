use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{Order, Orderbook, Side},
};

pub fn process_place_order(ctx: Context<PlaceOrder>, price: u64, quantity: u64, side: Side) -> Result<()> {
    let book = &mut ctx.accounts.book;
    let order = &mut ctx.accounts.order;

    if book.num_orders >= book.max_orders {
        return Err(ErrorCode::MaxOrdersReached.into());
    }

    order.price = price;
    order.quantity = quantity;
    order.side = side;

    book.num_orders += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub book: Account<'info, Orderbook>,
    
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
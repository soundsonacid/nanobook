use anchor_lang::prelude::*;
use crate::{
    error::ErrorCode,
    state::{Order, Orderbook},
};

pub fn process_cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
    let order = &mut ctx.accounts.order;
    let book = &mut ctx.accounts.book.load_mut()?;

    require!(order.placer == ctx.accounts.payer.key(), ErrorCode::CouldNotCancel);

    order.close(ctx.accounts.payer.to_account_info())?;
    book.num_orders -= 1;
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub book: AccountLoader<'info, Orderbook>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
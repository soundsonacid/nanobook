use anchor_lang::prelude::*;
use ix::*;
use state::Side;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod ix;
pub mod state;
pub mod error;
pub mod constants;
pub mod token_utils;

#[program]
pub mod nanobook {
    use super::*;

    pub fn initialize_orderbook(ctx: Context<InitializeOrderbook>) -> Result<()> {
        book_init::process_initialize_orderbook(ctx)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        user_init::process_initialize_user(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
        user_deposit::process_deposit(ctx, amt)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
        user_withdraw::process_withdrawal(ctx, amt)
    }

    pub fn place_limit_order(ctx: Context<PlaceLimitOrder>, price: u64, quantity: u64, side: Side) -> Result<()> {
        order_place_limit::process_place_limit_order(ctx, price, quantity, side)
    }

    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        order_cancel::process_cancel_order(ctx)
    }
}


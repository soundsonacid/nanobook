use anchor_lang::prelude::*;
use ix::*;
use state::{Market, Side}
;
declare_id!("7Fff3p1VST7iEVCcZyuM6sRp3WfPS8YG9s5GgmDH4vGR");

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

    pub fn place_limit_order(ctx: Context<PlaceLimitOrder>, price: u64, quantity: u64, side: Side, market: Market) -> Result<()> {
        order_place_limit::process_place_limit_order(ctx, price, quantity, side, market)
    }

    pub fn place_market_order(ctx: Context<PlaceMarketOrder>, quantity: u64, side: Side, market: Market) -> Result<()> {
        order_place_market::process_place_market_order(ctx, quantity, side, market)
    }

    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        order_cancel::process_cancel_order(ctx)
    }
}


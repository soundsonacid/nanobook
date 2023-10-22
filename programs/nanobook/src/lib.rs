use anchor_lang::prelude::*;
use ix::*;
use state::Side;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod ix;
pub mod state;
pub mod error;

#[program]
pub mod nanobook {
    use super::*;

    pub fn initialize_orderbook(ctx: Context<InitializeOrderbook>) -> Result<()> {
        book_init::process_initialize_orderbook(ctx)
    }

    pub fn place_order(ctx: Context<PlaceOrder>, price: u64, quantity: u64, side: Side) -> Result<()> {
        order_place::process_place_order(ctx, price, quantity, side)
    }
}


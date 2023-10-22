use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
#[repr(u8)]
pub enum Side {
    #[default]
    Buy = 0,
    Sell = 1,
}

#[account]
pub struct Orderbook {
    pub max_orders: u8,
    pub num_orders: u8,
}

#[derive(Default)]
#[account]
pub struct Order {
    pub id: u8,
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
}
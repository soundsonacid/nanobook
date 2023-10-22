use anchor_lang::prelude::*;
use bytemuck::{Zeroable, Pod};
use crate::constants::ORDER_BOOK_DEPTH;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
#[repr(u8)]
pub enum Side {
    Buy = 0,
    Sell = 1,
}

unsafe impl Zeroable for Side {}

unsafe impl Pod for Side {}

#[account(zero_copy)]
#[repr(C)]
pub struct Orderbook {
    pub max_orders: u8,
    pub num_orders: u8,
    _padding: [u8; 6],
    pub last_order_id: u64,
    pub buy_queue: OrderQueue,
    pub sell_queue: OrderQueue
}

#[derive(Copy, Zeroable, Pod)]
#[account]
#[repr(C)]
pub struct Order {
    pub id: u64,
    pub placer: Pubkey,
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
    _padding: [u8; 7],
}

#[account(zero_copy)]
#[repr(C)]
pub struct OrderQueue {
    pub side: Side,
    _padding1: [u8; 21], 
    _padding2: [u8; 21], 
    _padding3: [u8; 21], 
    pub orders: [Order; ORDER_BOOK_DEPTH as usize],
}

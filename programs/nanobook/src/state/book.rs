use anchor_lang::prelude::*;
use bytemuck::{Zeroable, Pod};
use crate::constants::ORDER_BOOK_DEPTH;

use super::FreeBitmap;

#[derive(Default, AnchorSerialize, AnchorDeserialize, Copy, Clone)]
#[repr(u8)]
pub enum Side {
    #[default]
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

impl Orderbook {
    pub fn add_buy_order(&mut self, order: Order) -> Option<u8> {
        if let Some(slot) = self.buy_queue.free_bitmap.find_first_zero() {
            self.buy_queue.orders[slot as usize] = order;
            self.buy_queue.free_bitmap.set(slot);

            return Some(slot)
        }

        None
    }

    pub fn add_sell_order(&mut self, order: Order) -> Option<u8> {
        if let Some(slot) = self.sell_queue.free_bitmap.find_first_zero() {
            self.sell_queue.orders[slot as usize] = order;
            self.sell_queue.free_bitmap.set(slot);

            return Some(slot)
        }

        None
    }

    pub fn remove_buy_order(&mut self, order_id: u64) {
        if let Some(slot) = self.buy_queue.orders.iter().enumerate()
            .find(|&(_, o)| o.id == order_id)
            .map(|(s, _)| s) {
                self.buy_queue.orders[slot] = Order::default();
                self.buy_queue.free_bitmap.clear(slot as u8);
            }
    }

    pub fn remove_sell_order(&mut self, order_id: u64) {
        if let Some(slot) = self.sell_queue.orders.iter().enumerate()
            .find(|&(_, o)| o.id == order_id)
            .map(|(s, _)| s) {
                self.sell_queue.orders[slot] = Order::default();
                self.sell_queue.free_bitmap.clear(slot as u8);
            }
    }
}

#[derive(Default, Copy, Zeroable, Pod)]
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
    pub free_bitmap: FreeBitmap,
}

#[derive(Default)]
#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub nano_balance: u64,
    pub sol_balance: u64,
    pub bump: u8,
}
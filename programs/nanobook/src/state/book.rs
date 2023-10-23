use anchor_lang::prelude::*;
use bytemuck::{Zeroable, Pod};
use crate::{
    state::{FreeBitmap, UserAccount},
    constants::ORDER_BOOK_DEPTH,
    error::ErrorCode,
};


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
    pub bump: u8,
    _padding: [u8; 7],
    pub last_order_id: u64,
    pub buy_queue: OrderQueue,
    pub sell_queue: OrderQueue,
}

impl Orderbook {

}

#[derive(Default, Copy, Zeroable, Pod)]
#[account]
#[repr(C)]
pub struct Order {
    pub id: u64,
    pub placer: UserAccount,
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
    pub _padding: [u8; 7],
}

#[account(zero_copy)]
#[repr(C)]
pub struct OrderQueue {
    pub side: Side,
    pub num_orders: u8,
    pub max_orders: u8,
    _padding1: [u8; 20], 
    _padding2: [u8; 20], 
    _padding3: [u8; 21], 
    pub orders: [Order; ORDER_BOOK_DEPTH as usize],
    pub free_bitmap: FreeBitmap,
}

impl OrderQueue {
    pub fn add_order(&mut self, order: Order) -> Option<u8> {
        if let Some(slot) = self.free_bitmap.find_first_zero() {
            self.orders[slot as usize] = order;
            self.free_bitmap.set(slot);

            return Some(slot)
        }

        None
    }

    pub fn remove_order(&mut self, order_id: u64) {
        if let Some(slot) = self.orders.iter().enumerate()
            .find(|&(_, o)| o.id == order_id)
            .map(|(s, _)| s) {
                self.orders[slot] = Order::default();
                self.free_bitmap.clear(slot as u8);
            }
    }

    pub fn update_order_quantity(&mut self, order_id: u64, new_quantity: u64) -> Result<()> {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            order.quantity = new_quantity;
            Ok(())
        } else {
            Err(ErrorCode::CouldNotFind.into())
        }
    }
}


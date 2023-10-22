use anchor_lang::prelude::*;
use std::cmp::min;
use crate::{state::{Order, Orderbook, Side}, constants::ORDER_DUST_THRESHOLD, error::ErrorCode};

pub struct MatchingEngine<'a> {
    orderbook: &'a mut Orderbook,
}


impl<'a> MatchingEngine<'a> {
    pub fn new(orderbook: &'a mut Orderbook) -> Self { 
        Self { orderbook }
    }

    pub fn match_limit_order(&mut self, order: &Order) -> Result<()> {
        let quotes = match order.side {
            Side::Buy => &mut self.orderbook.sell_queue,
            Side::Sell => &mut self.orderbook.buy_queue,
        };

        let mut remaining_quantity = order.quantity;

        while remaining_quantity > ORDER_DUST_THRESHOLD {
            if let Some(best_quote) = quotes.get_best_quote() {
                let matched_quantity = min(best_quote.quantity, remaining_quantity);
    
                // execute the trade here
    
                remaining_quantity -= matched_quantity;
    
                if best_quote.quantity == matched_quantity {
                    quotes.remove_order(best_quote.id);
                } else {
                    quotes.update_order_quantity(best_quote.id, best_quote.quantity - matched_quantity)?;
                }
    
                if remaining_quantity < ORDER_DUST_THRESHOLD {
                    // we were completely filled within 0.01 SOL / NANO of our limit order
                    break;
                }
            } else {
                // no quotes to match against - we can just let the limit order sit in the book
                break;
            }
        }

        if remaining_quantity > ORDER_DUST_THRESHOLD && remaining_quantity != order.quantity {
            let add_order_success = match order.side {
                Side::Buy => self.orderbook.buy_queue.add_order(Order { quantity: remaining_quantity, ..*order }),
                Side::Sell => self.orderbook.sell_queue.add_order(Order { quantity: remaining_quantity, ..*order }),
            };
        
            if add_order_success.is_none() {
                return Err(ErrorCode::MaxOrdersReached.into());
            }
        };

        Ok(())
    }

    pub fn match_market_order(&mut self, order: &Order) -> Result<()> {
        let quotes = match order.side {
            Side::Buy => &mut self.orderbook.sell_queue,
            Side::Sell => &mut self.orderbook.buy_queue,
        };

        let mut remaining_quantity = order.quantity;

        while remaining_quantity > ORDER_DUST_THRESHOLD {
            if let Some(best_quote) = quotes.get_best_quote() {
                let matched_quantity = min(best_quote.quantity, remaining_quantity);

                // execute order

                remaining_quantity -= matched_quantity;

                if best_quote.quantity == matched_quantity {
                    quotes.remove_order(best_quote.id);
                } else {
                    quotes.update_order_quantity(best_quote.id, best_quote.quantity - matched_quantity)?;
                }

                if remaining_quantity < ORDER_DUST_THRESHOLD {
                    // our order has been filled
                    break;
                }
            } else {
                // no more quotes to match against - we have to reject the order because it could not be filled
                return Err(ErrorCode::CouldNotFill.into());
            }
        }

        Ok(())
    }
}
use anchor_lang::prelude::{*, borsh::{BorshSerialize, BorshDeserialize}};
use std::cmp::min;
use crate::{state::{UserAccount, Order, Orderbook, Side}, constants::ORDER_DUST_THRESHOLD, error::ErrorCode};

#[derive(BorshSerialize, BorshDeserialize)]
#[repr(u8)]
pub enum Market {
    SolNano,
    NanoSol,
}

pub struct MatchingEngine<'a> {
    orderbook: &'a mut Orderbook,
}


impl<'a> MatchingEngine<'a> {
    pub fn new(orderbook: &'a mut Orderbook) -> Self { 
        Self { orderbook }
    }

    pub fn match_limit_order(&mut self, order: &Order, maker: &mut UserAccount, market: &Market) -> Result<()> {
        let quotes = match order.side {
            Side::Buy => &mut self.orderbook.sell_queue,
            Side::Sell => &mut self.orderbook.buy_queue,
        };

        let mut remaining_quantity = order.quantity;

        while remaining_quantity > ORDER_DUST_THRESHOLD {
            if let Some(best_quote_idx) = quotes.orders.iter()
                .enumerate()
                .filter(|&(_, order)| order.id != 0) // filter out Order::default()
                .max_by_key(|&(_, order)| order.price)
                .map(|(idx, _)| idx)
            {
                let best_quote: &mut Order = &mut quotes.orders[best_quote_idx];
                let matched_quantity = min(best_quote.quantity, remaining_quantity);
                
                let taker = &mut best_quote.placer;

                let quote_delta = best_quote.price * matched_quantity;

                Self::execute_order(maker, taker, matched_quantity, quote_delta, market);

                // copy these onto stack so we're not borrowing mutably twice
                let id = best_quote.id;
                let quantity = best_quote.quantity;

                remaining_quantity -= matched_quantity;
        
                if quantity == matched_quantity {
                    quotes.remove_order(id);
                } else {
                    quotes.update_order_quantity(id, quantity - matched_quantity)?;
                }
        
                if remaining_quantity < ORDER_DUST_THRESHOLD {
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
            
            require!(add_order_success.is_some(), ErrorCode::MaxOrdersReached);
        };

        Ok(())
    }

    pub fn match_market_order(&mut self, order: &Order, taker: &mut UserAccount, market: &Market) -> Result<()> {
        let quotes = match order.side {
            Side::Buy => &mut self.orderbook.sell_queue,
            Side::Sell => &mut self.orderbook.buy_queue,
        };

        let mut remaining_quantity = order.quantity;

        while remaining_quantity > ORDER_DUST_THRESHOLD {
            if let Some(best_quote_idx) = quotes.orders.iter()
                .enumerate()
                .filter(|&(_, order)| order.id != 0) // filter out Order::default()
                .max_by_key(|&(_, order)| order.price)
                .map(|(idx, _)| idx)
            {
                let best_quote: &mut Order = &mut quotes.orders[best_quote_idx];
                let matched_quantity = min(best_quote.quantity, remaining_quantity);
                
                let maker = &mut best_quote.placer;

                let quote_delta = best_quote.price * matched_quantity;

                Self::execute_order(maker, taker, matched_quantity, quote_delta, market);

                // copy these onto stack so we're not borrowing mutably twice
                let id = best_quote.id;
                let quantity = best_quote.quantity;

                remaining_quantity -= matched_quantity;
        
                if quantity == matched_quantity {
                    quotes.remove_order(id);
                } else {
                    quotes.update_order_quantity(id, quantity - matched_quantity)?;
                }

                    if remaining_quantity < ORDER_DUST_THRESHOLD {
                        // our order has been filled
                        break;
                    }
                } else {
                    // no more quotes to match against - we have to reject the order because it could not be filled
                    require!(remaining_quantity < ORDER_DUST_THRESHOLD, ErrorCode::CouldNotFill);
            }
        }

        Ok(())
    }

    pub fn execute_order(maker: &mut UserAccount, taker: &mut UserAccount, base_delta: u64, quote_delta: u64, market: &Market) {
        match market {
            Market::SolNano => {
                maker.sol_balance = maker.sol_balance.saturating_sub(base_delta);
                taker.sol_balance = taker.sol_balance.saturating_add(base_delta);
                maker.nano_balance = maker.nano_balance.saturating_add(quote_delta);
                taker.nano_balance = taker.nano_balance.saturating_sub(quote_delta);
            },
            Market::NanoSol => {
                maker.nano_balance = maker.nano_balance.saturating_sub(base_delta);
                taker.nano_balance = taker.nano_balance.saturating_add(base_delta);
                maker.sol_balance = maker.sol_balance.saturating_add(quote_delta);
                taker.sol_balance = taker.sol_balance.saturating_sub(quote_delta);
            },
        };
    }
}
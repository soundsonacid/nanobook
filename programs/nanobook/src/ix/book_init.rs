use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{TokenAccount, Mint, Token}};
use crate::{state::Orderbook, constants::ORDER_BOOK_DEPTH};

pub fn process_initialize_orderbook(ctx: Context<InitializeOrderbook>) -> Result<()> {
    let book = &mut ctx.accounts.book.load_mut()?;
    book.max_orders = ORDER_BOOK_DEPTH;
    book.num_orders = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeOrderbook<'info> {
    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Orderbook>() + 8,
    )]
    pub book: AccountLoader<'info, Orderbook>,

    pub nano_mint: Account<'info, Mint>,

    #[account(
        init, 
        payer = payer,
        associated_token::authority = book, 
        associated_token::mint = nano_mint
    )]
    pub nano_vault: Account<'info, TokenAccount>,

    pub sol_mint: Account<'info, Mint>,

    #[account(
        init, 
        payer = payer,
        associated_token::authority = book, 
        associated_token::mint = sol_mint
    )]
    pub sol_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
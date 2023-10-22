use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};

pub fn token_transfer_signed<'info, A: ToAccountInfo<'info>, B: ToAccountInfo<'info>, C: ToAccountInfo<'info>>(
    amount: u64,
    token_program: &A,
    to: &B,
    from: &B,
    authority: &C,
    seeds: &[&[u8]],
) -> Result<()> {
    if amount > 0 {
        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    to: to.to_account_info(),
                    from: from.to_account_info(),
                    authority: authority.to_account_info(),
                },
                &[seeds],
            ),
            amount,
        )
    } else {
        Ok(())
    }
}

pub fn token_transfer<'info, A: ToAccountInfo<'info>, B: ToAccountInfo<'info>, C: ToAccountInfo<'info>>(
    amount: u64,
    token_program: &A,
    from: &B,
    to: &B,
    authority: &C,
) -> Result<()> {
    if amount > 0 {
        transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )
    } else {
        Ok(())
    }
}


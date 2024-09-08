use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{Fundraiser, FUNDRAISER_SEED};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        seeds = [FUNDRAISER_SEED, fundraiser.maker.key().as_ref()],
        bump = fundraiser.bump,  
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority= fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,

    pub mint_to_raise: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler_refund(ctx: Context<Refund>) -> Result<()> {
    Ok(())
}

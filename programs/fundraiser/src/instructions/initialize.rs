use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{state::Fundraiser, FundraiserError, FUNDRAISER_SEED, MIN_AMOUNT_TO_RAISE};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = maker,
        seeds = [FUNDRAISER_SEED, maker.key().as_ref()],
        bump,
        space = 8 + Fundraiser::INIT_SPACE
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_to_raise,
        associated_token::authority= fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,

    pub mint_to_raise: Account<'info, Mint>,

    #[account(mut)]
    pub maker: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler_initialize(ctx: Context<Initialize>, amount: u64, duration: u16) -> Result<()> {
    require!(
        amount >= MIN_AMOUNT_TO_RAISE.pow(ctx.accounts.mint_to_raise.decimals as u32),
        FundraiserError::InvalidAmount
    );

    ctx.accounts.fundraiser.set_inner(Fundraiser {
        bump: ctx.bumps.fundraiser,
        maker: ctx.accounts.maker.key(),
        mint_to_raise: ctx.accounts.mint_to_raise.key(),
        amount_to_raise: amount,
        current_amount: 0,
        time_started: Clock::get()?.unix_timestamp,
        duration: duration,
    });

    Ok(())
}

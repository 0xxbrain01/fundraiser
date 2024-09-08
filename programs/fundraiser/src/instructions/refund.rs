use anchor_lang::prelude::*;
use anchor_spl::token::{  Mint, Token, TokenAccount, Transfer};
use anchor_spl::token::transfer;

use crate::{Contributor, Fundraiser, FundraiserError, CONTRIBUTER_SEED, FUNDRAISER_SEED};

#[derive(Accounts)]
pub struct Refund<'info> {

    #[account(
        mut,
        seeds = [CONTRIBUTER_SEED, fundraiser.key().as_ref(), contributor.key().as_ref()],
        bump,
    )]
    pub contributor_account: Account<'info, Contributor>,

    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = contributor
    )]
    pub contributor_ata: Account<'info, TokenAccount>,


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

    #[account(mut)]
    pub contributor: Signer<'info>,

    //pub maker: SystemAccount<'info>,

    pub mint_to_raise: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler_refund(ctx: Context<Refund>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        ctx.accounts.fundraiser.duration <= (current_time - ctx.accounts.fundraiser.time_started) as u16,
        FundraiserError::FundraiserNotEnded
     );

     require!(ctx.accounts.vault.amount < ctx.accounts.fundraiser.amount_to_raise, FundraiserError::TargetMet );
     let cpi_program = ctx.accounts.token_program.to_account_info();

     let cpi_accounts = Transfer{
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.contributor_ata.to_account_info(),
        authority: ctx.accounts.fundraiser.to_account_info()
     };

     let signer_seeds: [&[&[u8]]; 1]  = [&[
        FUNDRAISER_SEED,
        &ctx.accounts.fundraiser.maker.as_ref(),
        &[ctx.accounts.fundraiser.bump]
     ]]; 

     let cpi_ctx = CpiContext::new_with_signer(
        cpi_program, 
        cpi_accounts, 
        &signer_seeds);

       transfer(cpi_ctx, ctx.accounts.contributor_account.amount)?;

       ctx.accounts.fundraiser.current_amount -= ctx.accounts.contributor_account.amount;

    Ok(())
}

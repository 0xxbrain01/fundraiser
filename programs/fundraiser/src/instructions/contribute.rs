use anchor_lang::{prelude::*, solana_program::{clock::SECONDS_PER_DAY}};
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{Contributor, Fundraiser, FundraiserError, CONTRIBUTER_SEED, FUNDRAISER_SEED};

#[derive(Accounts)]
pub struct Contribute<'info> {

    #[account(
        init,
        payer = contributer,
        seeds = [CONTRIBUTER_SEED, fundraiser.key().as_ref(), contributer.key().as_ref()],
        bump,
        space = 8 + Contributor::INIT_SPACE
    )]
    pub contributor_account: Account<'info, Contributor>,

    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = contributer
    )]
    pub contributor_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [FUNDRAISER_SEED, fundraiser.maker.as_ref()],
        bump = fundraiser.bump,  
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    #[account(
        mut,
        associated_token::mint = fundraiser.mint_to_raise,
        associated_token::authority= fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,

    pub mint_to_raise: Account<'info, Mint>,

    #[account(mut)]
    pub contributer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler_contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {

    require!(amount >= 1_u64.pow( ctx.accounts.mint_to_raise.decimals as u32), FundraiserError::ContributionTooSmall);
    require!(amount <=  ctx.accounts.fundraiser.amount_to_raise , FundraiserError::ContributionTooBig);

    let current_time = Clock::get()?.unix_timestamp;

    require!( ctx.accounts.fundraiser.duration <= (current_time - ctx.accounts.fundraiser.time_started as i64) as u16, FundraiserError::FundraiserEnded );
    require!( ctx.accounts.fundraiser.current_amount + amount <=  ctx.accounts.fundraiser.amount_to_raise, FundraiserError::MaximumContributionsReached );

    // let cpi_account = Transfer{
    //     from: ctx.accounts.contributer.to_account_info(),
    //     to: ctx.accounts.vault.to_account_info(),
    //     authority: ctx.accounts.contributer.to_account_info()
    // };

    let cpi_accounts = Transfer{
        from: ctx.accounts.contributor_ata.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.contributer.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer(cpi_ctx, amount)?;

    ctx.accounts.fundraiser.current_amount += amount;
    ctx.accounts.contributor_account.amount += amount;

    Ok(())
}

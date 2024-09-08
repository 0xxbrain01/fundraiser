use anchor_lang::prelude::*;

declare_id!("6HxECZB3LaJttGbndvtjgUkFu6cJLRDEdag3V58hXqrs");

mod error;
use error::*;
mod constants;
use constants::*;

mod state;
use state::*;
mod instructions;
use instructions::*;

#[program]
pub mod fundraiser {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, duration: u16) -> Result<()> {
        instructions::handler_initialize(ctx, amount, duration)
    }
    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        instructions::handler_contribute(ctx, amount)
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::handler_refund(ctx)
    }
}

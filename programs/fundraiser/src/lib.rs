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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

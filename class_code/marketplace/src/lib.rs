use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;

declare_id!("J38Bnc89hv6iLvqX9PuYcdZw33rso8JxJJdHSHQdzd8J");

#[program]
pub mod marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, name, fee)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(&ctx.bumps, price)?;
        ctx.accounts.deposit_nft()
    }

    pub fn whitelist_collection(ctx: Context<WhitelistCollection>) -> Result<()> {
        ctx.accounts.whitelist(&ctx.bumps)
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_ata()
    }
}


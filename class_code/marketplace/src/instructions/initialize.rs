use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token};

use crate::{errors::MarketplaceError, state::Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Marketplace::LEN,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = rewards, // authority is itself?
    )]
    rewards: Account<'info, Mint>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,

    token_program: Program<'info, Token>, // CORRECT?
    system_program: Program<'info, System>,

}

// What is InitializeBumps?
impl<'info> Initialize<'info> {
    pub fn init(&mut self, bumps: &InitializeBumps, name: String, fee: u16) -> Result<()> {
        
        require!(
            name.len() < 33 && name.len() > 4,
            MarketplaceError::InvalidName
        );
        self.marketplace.admin = self.admin.key();
        self.marketplace.fee = fee;
        self.marketplace.name = name;
        self.marketplace.bump = bumps.marketplace;
        self.marketplace.treasury_bump = bumps.treasury;
        Ok(())
    }
}
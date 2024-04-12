use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::{Marketplace, Whitelist};

#[derive(Accounts)]
pub struct WhitelistCollection<'info> {

    #[account(mut)]
    admin: Signer<'info>,
    mint: Account<'info, Mint>,
    #[account(
        has_one = admin,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        seeds = [marketplace.key().as_ref(), mint.key().as_ref()],
        space = Whitelist::LEN,
        bump
    )]
    whitelist: Account<'info, Whitelist>,
    system_program: Program<'info, System>
}

// Good bumps implimentation ??
// whitelist doesn't do anything??
impl<'info> WhitelistCollection<'info> {
    pub fn whitelist(&mut self, bumps: &WhitelistCollectionBumps) -> Result<()> {
        self.whitelist.bump = bumps.whitelist;
        Ok(())
    }
}
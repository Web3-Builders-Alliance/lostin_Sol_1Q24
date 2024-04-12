use anchor_lang::prelude::*;

use crate::{state::Marketplace, state::Listing, state::Whitelist };

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
    token::{transfer as spl_transfer, Mint, Token, TokenAccount, Transfer as SplTransfer}
};


#[derive(Accounts)]
pub struct List<'info> {

    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    marketplace: Account<'info, Marketplace>,
    maker_mint: Account<'info, Mint>,
    collection_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    maker_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        seeds = [b"auth", maker_mint.key().as_ref()],
        bump,
        token::authority = vault,
        token::mint = maker_mint
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump = whitelist.bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        init,
        payer = maker,
        space = Listing::LEN,
        seeds = [whitelist.key().as_ref(), maker_mint.key().as_ref()], // why these? need whitelist before listing?
        bump
    )]
    listing: Account<'info, Listing>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    metadata: Account<'info, MetadataAccount>,

    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>, 
    system_program: Program<'info, System>,

}

impl<'info> List<'info> {
    pub fn create_listing(&mut self, bumps: &ListBumps, price: u64) -> Result<()> {

        self.listing.maker = self.maker.key();
        self.listing.mint = self.maker_mint.key();
        self.listing.price = price;
        self.listing.bump = bumps.listing;
        self.listing.auth_bump = bumps.vault;
        Ok(())
    }

    // why no token mint referenced?
    pub fn deposit_nft(&self) -> Result<()> {

        let accounts = SplTransfer {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        spl_transfer(ctx, 1)
    }
}
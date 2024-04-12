use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{close_account, CloseAccount};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Token, Mint, TokenAccount, transfer as spl_transfer, Transfer as SplTransfer}};

use crate::state::{Listing, Marketplace, Whitelist};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    /// CHECK: Okay
    maker: UncheckedAccount<'info>,
    #[account(
        seeds = [b"marketpalce", marketplace.name.as_str().as_ref()],
        bump = marketplace.bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump = whitelist.bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    taker_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"auth", maker_mint.key().as_ref()],
        bump = listing.auth_bump,
        token::mint = maker_mint,
        token::authority = vault,
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    treasury: SystemAccount<'info>,
    maker_mint: Account<'info, Mint>,
    collection_mint: Account<'info, Mint>,
    #[account(
        mut,
        close = taker,
        has_one = maker,
        seeds = [whitelist.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump
    )]
    listing: Account<'info, Listing>,
    
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {

    pub fn send_sol(&self) -> Result<()> {

        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info()
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, self.listing.price)
    }

    pub fn send_nft(&self) -> Result<()> {

        let accounts = SplTransfer {
            to: self.taker_ata.to_account_info(),
            from: self.vault.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let seeds = [];

        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        spl_transfer(ctx, 1)

    }

    pub fn close_mint_ata(&self) -> Result<()> {

        let accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let seeds = [
            b"auth", 
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        close_account(ctx)

    }
}
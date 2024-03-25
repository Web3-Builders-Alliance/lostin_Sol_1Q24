use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, TokenInterface, Mint},
};

use crate::{error::AmmError, Config, AUTH_SEED, CONFIG_SEED, LP_SEED};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        seeds = [LP_SEED.as_ref(), config.key().as_ref()],
        payer = initializer,
        mint::decimals = 6,
        mint::authority = auth,
        mint::token_program = token_program,
        bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=initializer,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
        associated_token::token_program = token_program,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer=initializer,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
        associated_token::token_program = token_program,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [AUTH_SEED.as_ref()], 
        bump
    )]
    /// CHECK: just used to sign
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = initializer,
        seeds = [CONFIG_SEED.as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Config::LEN
    )]
    pub config: Account<'info, Config>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>,
    seed: u64,
    fee: u16,
    authority: Option<Pubkey>
) -> Result<()> {
    require!(fee <= 10000, AmmError::InvalidFee);
    ctx.accounts.config.init(
        seed,
        authority,
        ctx.accounts.mint_x.key(),
        ctx.accounts.mint_y.key(),
        fee,
        ctx.bumps.auth,
        ctx.bumps.config,
        ctx.bumps.mint_lp
    );
    Ok(())
}
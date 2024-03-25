use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked
    },
};
use constant_product_curve::ConstantProduct;

use crate::{
    assert_non_zero, assert_not_expired, assert_not_locked, error::AmmError, Config, AUTH_SEED, CONFIG_SEED, LP_SEED
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [LP_SEED.as_ref(), config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: Just used to sign, should be safe
    #[account(seeds=[AUTH_SEED.as_ref()], bump = config.auth_bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds=[CONFIG_SEED.as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &self,
        amount: u64, // amount of LP tokens user will get??
        max_x: u64, // max amount of token x can deposit
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_non_zero!([amount, max_x, max_y]);
        assert_not_expired!(expiration);

        let (x, y) = match self.mint_lp.supply == 0
        && self.vault_x.amount == 0
        && self.vault_y.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                )
                .map_err(AmmError::from)?;
                (amount.x, amount.y)
            },
        };

        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);

        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;
        self.mint_lp_tokens(amount)
    }

    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_account = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_account);
        transfer_checked(cpi_context, amount, decimals)
    }

    pub fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_account = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[&AUTH_SEED.as_ref()[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_account,
            signer_seeds,
        );
        mint_to(cpi_context, amount)
    }
}
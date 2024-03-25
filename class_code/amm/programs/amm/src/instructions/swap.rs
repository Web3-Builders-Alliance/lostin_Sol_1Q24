use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked }
};
use constant_product_curve::{ ConstantProduct, LiquidityPair };

use crate::{
    assert_non_zero, assert_not_expired, assert_not_locked, error::AmmError, Config, AUTH_SEED, CONFIG_SEED, LP_SEED,
};

#[derive(Accounts)]
pub struct Swap<'info> {
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
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp: InterfaceAccount<'info, TokenAccount>,
    ///CHECK: this is okay
    #[account(seeds = [AUTH_SEED.as_ref()], bump = config.auth_bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [CONFIG_SEED.as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface <'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info>Swap<'info>{
    pub fn swap(
        &mut self,
        is_x: bool,
        amount: u64,
        min: u64,
        expiration: i64
    ) -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([amount]);
    
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        msg!("curve: {:?}", curve); // debugging

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        msg!("p: {:?}", p); // debugging

        let res = curve.swap(p, amount, min).map_err(AmmError::from)?;

        assert_non_zero!([res.deposit, res.withdraw]);
        self.deposit_token(is_x, res.deposit)?;
        self.withdraw_token(is_x, res.withdraw)?;
        Ok(())

    }

    pub fn deposit_token(&mut self, is_x:bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals
            ),
            false => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals
            )
        };

        let accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(),accounts);

        transfer_checked(ctx, amount, decimals)
    }

    pub fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let(from, to, mint, decimals) = match is_x {
            true => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals
            ),
            false => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals
            )
        };

        let accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint
        };

        let seeds = &[&AUTH_SEED.as_ref()[..], &[self.config.auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), 
        accounts,
        signer_seeds
        );

        transfer_checked(ctx, amount, decimals)

    }
}
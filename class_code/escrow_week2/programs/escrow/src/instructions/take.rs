use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, close_account, CloseAccount}
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {

    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        has_one = mint_x,
        has_one = mint_y,
        has_one = maker,
        close = maker,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_x,
        associated_token::authority = taker,
    )]
    pub taker_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, 
        associated_token::mint = mint_y,
        associated_token::authority = taker,
    )]
    pub taker_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_y,
        associated_token::authority = maker
    )]
    pub maker_ata_y: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Take<'info>{
    pub fn pay_back(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.taker_ata_y.to_account_info(),
            to: self.maker_ata_y.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_y.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.amount, self.mint_y.decimals)
    }

    pub fn take(&mut self) -> Result<()>{
        
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_x.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_x.to_account_info()
        };
        let signer_seeds: [&[&[u8]]; 1] = [
            &[
                b"escrow",
                self.maker.to_account_info().key.as_ref(),
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.bump],
            ],
        ];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer_checked(cpi_ctx, self.escrow.amount, self.mint_x.decimals)?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds
        );

        close_account(cpi_ctx)
    }
}
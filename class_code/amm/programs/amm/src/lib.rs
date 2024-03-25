pub mod constants;
pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BMkaAz3d4xw8dCeST8j3DnmVHPFuEpMiZNbeSkhTYreE");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        initialize::handler(ctx, seed, fee, authority)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_x: bool,
        amount: u64,
        min: u64,
        expiration: i64,
    ) -> Result<()> {
        msg!("Swap token X for token Y or vice versa");
        msg!("is_x: {}", is_x);
        msg!("amount: {}", amount);
        msg!("min: {}", min);
        ctx.accounts.swap(is_x, amount, min, expiration)
    }
}


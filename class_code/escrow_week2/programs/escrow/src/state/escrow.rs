use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR_SIZE, PUBKEY_SIZE, U64_SIZE, U8_SIZE};

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub bump: u8,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub seed: u64,
    pub amount: u64
}

impl Space for Escrow {
    const INIT_SPACE: usize = DISCRIMINATOR_SIZE
    + PUBKEY_SIZE
    + U8_SIZE
    + PUBKEY_SIZE
    + PUBKEY_SIZE
    + U64_SIZE
    + U64_SIZE;
}
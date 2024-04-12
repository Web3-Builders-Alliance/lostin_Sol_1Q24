use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
    pub auth_bump: u8,
}

impl Listing {
    pub const LEN:usize = 8 + 32 + 32 + 8 + 1 + 1;
}
use anchor_lang::prelude::*;

#[constant]
pub const LP_SEED: &'static [u8] = b"lp";
#[constant]
pub const CONFIG_SEED: &'static [u8] = b"config"; 
#[constant]
pub const AUTH_SEED: &'static [u8] = b"auth";
#[constant]
pub const DISCRIMINATOR_SIZE: usize = std::mem::size_of::<u64>();
#[constant]
pub const PUBKEY_SIZE: usize = std::mem::size_of::<Pubkey>();
#[constant]
pub const OPTION_SIZE: usize = 1;
#[constant]
pub const U8_SIZE: usize = std::mem::size_of::<u8>();
#[constant]
pub const U16_SIZE: usize = std::mem::size_of::<u16>();
#[constant]
pub const U64_SIZE: usize = std::mem::size_of::<u64>();
#[constant]
pub const BOOL_SIZE: usize = std::mem::size_of::<bool>();
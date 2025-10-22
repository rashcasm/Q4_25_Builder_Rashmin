use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{errors::AmmError, state::Config};

// #[derive(Accounts)]
// pub struct Swap<'info> {
// TODO: Write the accounts struct
// }

// impl<'info> Swap<'info> {
//     pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
// TODO
//}

//     pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
// TODO
//}

//     pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {}
// TODO
//}

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;

use crate::{errors::AmmError, state::Config};

// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     //TODO
// }

// impl<'info> Withdraw<'info> {
//     pub fn withdraw(
//         &mut self,
//         amount: u64, // Amount of LP tokens that the user wants to "burn"
//         min_x: u64,  // Minimum amount of token X that the user wants to receive
//         min_y: u64,  // Minimum amount of token Y that the user wants to receive
//     ) -> Result<()> {
//         // TODO
//     }

//     pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
//         //TODO
//     }

//     pub fn burn_lp_tokens(&self, amount: u64) -> Result<()> {
//         //TODO
//     }
// }

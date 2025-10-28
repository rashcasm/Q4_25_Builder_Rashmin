use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{StakeConfig, UserAccount};

// #[derive(Accounts)]
// pub struct Claim<'info> {
// //TODO
// }

// impl<'info> Claim<'info> {
//     pub fn claim(&mut self) -> Result<()> {
//     //TODO
//     }
// }

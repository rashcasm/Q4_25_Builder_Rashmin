use anchor_lang::prelude::*;
use mpl_core::{
    instructions::AddPluginV1CpiBuilder,
    types::{FreezeDelegate, Plugin, PluginAuthority},
    ID as CORE_PROGRAM_ID,
};

use crate::{
    errors::StakeError,
    state::{StakeAccount, StakeConfig, UserAccount},
};

// #[derive(Accounts)]
// pub struct Stake<'info> {
//TODO
// }

// impl<'info> Stake<'info> {
//     pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
//TODO
//     }
// }

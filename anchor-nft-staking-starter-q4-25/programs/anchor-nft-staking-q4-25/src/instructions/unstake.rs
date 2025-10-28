use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{RemovePluginV1CpiBuilder, UpdatePluginV1CpiBuilder},
    types::{FreezeDelegate, Plugin, PluginType},
    ID as CORE_PROGRAM_ID,
};

use crate::{
    errors::StakeError,
    state::{StakeAccount, StakeConfig, UserAccount},
};

// #[derive(Accounts)]
// pub struct Unstake<'info> {
// //TODO
// }

// impl<'info> Unstake<'info> {
//     pub fn unstake(&mut self) -> Result<()> {
// //TODO
//     }
// }

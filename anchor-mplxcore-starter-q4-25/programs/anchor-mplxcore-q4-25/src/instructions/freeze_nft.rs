use anchor_lang::prelude::*;
use mpl_core::{
    instructions::UpdatePluginV1CpiBuilder,
    types::{FreezeDelegate, Plugin},
    ID as CORE_PROGRAM_ID,
};

use crate::{error::MPLXCoreError, state::CollectionAuthority};

// #[derive(Accounts)]
// pub struct FreezeNft<'info> {
//    // TODO
// }

// impl<'info> FreezeNft<'info> {
//     pub fn freeze_nft(&mut self) -> Result<()> {
//         // TODO
//         Ok(())
//     }
// }

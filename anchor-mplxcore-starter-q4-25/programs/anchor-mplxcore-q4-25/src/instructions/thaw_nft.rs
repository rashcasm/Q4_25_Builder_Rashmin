use anchor_lang::prelude::*;
use mpl_core::{
    instructions::UpdatePluginV1CpiBuilder,
    types::{FreezeDelegate, Plugin},
    ID as CORE_PROGRAM_ID,
};

use crate::{error::MPLXCoreError, state::CollectionAuthority};

// #[derive(Accounts)]
// pub struct ThawNft<'info> {
//    // TODO
// }

// impl<'info> ThawNft<'info> {
//     pub fn thaw_nft(&mut self) -> Result<()> {
//         // TODO

//         Ok(())
//     }
// }

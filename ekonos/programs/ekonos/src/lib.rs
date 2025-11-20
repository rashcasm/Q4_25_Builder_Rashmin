#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use crate::instructions::*;
pub mod errors;
pub mod instructions;
pub mod state;
use crate::state::ShareDistribution;

declare_id!("23XFNuyCuMKxHYWjoqdU4tNxgwJv7Mh184X6t6C2vMR6");

#[program]
pub mod ekonos {
    use super::*;
    pub fn initialize_partnership(
        ctx: Context<InitializePartnership>,
        partnership_id: u64,
        total_shares: u64,
    ) -> Result<()> {
        initialize_partnership::handle(ctx, partnership_id, total_shares)
    }
    pub fn deposit_nft(ctx: Context<DepositNft>) -> Result<()> {
        deposit_nft::handle(ctx)
    }
    pub fn mint_shares(
        ctx: Context<MintShares>,
        distribution: Vec<ShareDistribution>,
    ) -> Result<()> {
        mint_shares::handle(ctx, distribution)
    }
}


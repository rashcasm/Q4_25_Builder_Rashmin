use anchor_lang::prelude::*;
use crate::{state::*};

#[derive(Accounts)]
#[instruction(partnership_id: u64)]
pub struct InitializePartnership<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Partnership>(),
        seeds = [b"partnership", authority.key().as_ref(), &partnership_id.to_le_bytes()],
        bump
    )]
    pub partnership: Account<'info, Partnership>,

    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<InitializePartnership>,
    partnership_id: u64,
    total_shares: u64,
) -> Result<()> {
    let partnership = &mut ctx.accounts.partnership;

    partnership.authority = ctx.accounts.authority.key();
    partnership.partnership_id = partnership_id;
    partnership.total_shares = total_shares;
    partnership.is_nft_deposited = false;
    partnership.is_shares_minted = false;
    partnership.bumps.partnership = ctx.bumps.partnership;

    Ok(())
}

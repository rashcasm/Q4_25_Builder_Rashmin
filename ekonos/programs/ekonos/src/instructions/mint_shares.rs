use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token::{self, Mint, Token, MintTo}};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{Partnership, ShareDistribution};
use crate::errors::PartnershipError;

/// Mint fractional shares for a partnership and distribute them to provided wallets.
///
/// Remaining accounts: for each `ShareDistribution` entry you MUST pass two AccountInfos in order:
/// 1) the destination wallet `UncheckedAccount` (owner of the ATA)
/// 2) the destination associated token account `UncheckedAccount` (may be uninitialized)
///
#[derive(Accounts)]
#[instruction(distribution: Vec<ShareDistribution>)]
pub struct MintShares<'info> {
    /// The authority that created the partnership
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The partnership account (must already exist and be owned by `authority`)
    #[account(mut, has_one = authority)]
    pub partnership: Account<'info, Partnership>,

    /// PDA mint that will represent fractional shares for this partnership.
    /// Created with `mint::` init helper so it's a real SPL Mint account.
    #[account(
        init,
        payer = authority,
        seeds = [b"share_mint", partnership.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = partnership,
        mint::freeze_authority = partnership,
    )]
    pub share_mint: Account<'info, Mint>,

    /// Programs / sysvars used for CPI
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// Authority's ATA to receive all minted shares (single-recipient flow)
    /// CHECK: This is the associated token account for the `authority` and `share_mint`.
    /// We verify its address and create it via CPI if it doesn't exist.
    #[account(mut)]
    pub authority_ata: UncheckedAccount<'info>,
}

// pub fn handle<'info>(ctx: Context<'info, MintShares<'info>>, distribution: Vec<ShareDistribution>) -> Result<()> {
pub fn handle(ctx: Context<MintShares>, distribution: Vec<ShareDistribution>) -> Result<()> {
    let partnership = &mut ctx.accounts.partnership;

    // Preconditions
    if !partnership.is_nft_deposited {
        return err!(PartnershipError::NftNotDeposited);
    }
    if partnership.is_shares_minted {
        return err!(PartnershipError::SharesAlreadyMinted);
    }

    // Validate distribution sums to total_shares
    let mut total: u64 = 0;
    for d in &distribution {
        total = total.checked_add(d.amount).ok_or(error!(PartnershipError::InvalidShareDistribution))?;
    }
    if total != partnership.total_shares {
        return err!(PartnershipError::InvalidShareDistribution);
    }

    // Store mint and bump
    partnership.share_mint = ctx.accounts.share_mint.key();
    partnership.is_shares_minted = true;
    partnership.bumps.share_mint = ctx.bumps.share_mint;

    // Single-recipient flow: mint the total shares to the authority's ATA provided in `authority_ata`.
    let (expected_ata, _bump) = Pubkey::find_program_address(
        &[
            ctx.accounts.authority.key().as_ref(),
            token::ID.as_ref(),
            ctx.accounts.share_mint.key().as_ref(),
        ],
        &associated_token::ID,
    );
    if ctx.accounts.authority_ata.key() != expected_ata {
        return err!(PartnershipError::InvalidTokenAccount);
    }

    let ata_info = ctx.accounts.authority_ata.to_account_info();
    if ata_info.data_is_empty() {
        let cpi_program = ctx.accounts.associated_token_program.to_account_info();
        let cpi_accounts = associated_token::Create {
            payer: ctx.accounts.authority.to_account_info(),
            associated_token: ata_info.clone(),
            authority: ctx.accounts.authority.to_account_info(),
            mint: ctx.accounts.share_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        associated_token::create(cpi_ctx)?;
    }

    // Mint total shares to authority ATA using partnership PDA as mint authority
    let seeds: &[&[u8]] = &[
        b"partnership",
        partnership.authority.as_ref(),
        &partnership.partnership_id.to_le_bytes(),
        &[partnership.bumps.partnership],
    ];
    let signer = &[seeds];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.share_mint.to_account_info(),
        to: ata_info.clone(),
        authority: partnership.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::mint_to(cpi_ctx, partnership.total_shares)?;


    Ok(())
}

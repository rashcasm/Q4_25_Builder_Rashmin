use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::{self, AssociatedToken};
use crate::state::Partnership;
use crate::errors::PartnershipError;
use anchor_lang::solana_program::program_pack::Pack;

#[derive(Accounts)]
pub struct DepositNft<'info> {
    /// The creator / authority that owns the Partnership
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The partnership account (must be already created via initialize_partnership)
    /// We require has_one = authority so only the partnership authority may call this.
    #[account(mut, has_one = authority)]
    pub partnership: Account<'info, Partnership>,

    /// The NFT mint that is being deposited
    /// CHECK: read-only mint account for the NFT (we don't deserialize as Mint here)
    pub nft_mint: UncheckedAccount<'info>,

    /// Source token account holding the NFT (must be owned by the authority)
    #[account(
        mut,
        constraint = nft_from.mint == nft_mint.key() @ PartnershipError::InvalidTokenAccount,
        constraint = nft_from.amount == 1 @ PartnershipError::InvalidTokenAccount,
        constraint = nft_from.owner == authority.key() @ PartnershipError::Unauthorized,
    )]
    pub nft_from: Account<'info, TokenAccount>,

    /// Vault ATA that will hold the NFT; owner = partnership PDA.
    /// CHECK: This is the associated token account for the `partnership` PDA and the `nft_mint`.
    /// We accept it as `UncheckedAccount` so we can create it via CPI if not initialized.
    /// After creation/verification we will treat it as a TokenAccount.
    #[account(mut)]
    pub vault_ata: UncheckedAccount<'info>,

    /// Programs / sysvars for CPI
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(ctx: Context<DepositNft>) -> Result<()> {
    let partnership = &mut ctx.accounts.partnership;

    // ensure NFT isn't already deposited for this partnership (MVP: only one NFT)
    if partnership.is_nft_deposited {
        return err!(PartnershipError::NftAlreadyDeposited);
    }

    // Confirm nft_from actually holds 1 token (checked by constraint), but double-check
    if ctx.accounts.nft_from.amount < 1 {
        return err!(PartnershipError::InvalidTokenAccount);
    }

    // Calculate expected ATA address for (nft_mint, partnership_pda)
    let partnership_key = partnership.key();
    // associated token PDA = PDA([owner, token_program_id, mint], associated_token_program_id)
    let (expected_vault, _bump) = Pubkey::find_program_address(
        &[
            partnership_key.as_ref(),
            token::ID.as_ref(),
            ctx.accounts.nft_mint.key().as_ref(),
        ],
        &associated_token::ID,
    );

    // If provided vault_ata address doesn't match expected derived ATA -> error
    if ctx.accounts.vault_ata.key() != expected_vault {
        return Err(error!(PartnershipError::InvalidTokenAccount));
    }

    // If vault ATA is not initialized (data len == 0), create it via CPI
    // Note: AccountInfo::data_is_empty is available on UncheckedAccount::to_account_info()
    let vault_info = ctx.accounts.vault_ata.to_account_info();
    if vault_info.data_is_empty() {
        // Build CPI context for creating associated token account
        let cpi_program = ctx.accounts.associated_token_program.to_account_info();
        let cpi_accounts = anchor_spl::associated_token::Create {
            payer: ctx.accounts.authority.to_account_info(),
            associated_token: vault_info.clone(),
            authority: partnership.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        associated_token::create(cpi_ctx)?;
    } else {
        // If ATA exists, verify it's a token account with correct mint and owner
        // If ATA exists, verify it's a token account with correct mint and owner
        // Deserialize using `spl_token::state::Account` which provides `unpack`.
        let data = vault_info.data.borrow();
        let vault_state = spl_token::state::Account::unpack(&data[..])
            .map_err(|_| error!(PartnershipError::InvalidTokenAccount))?;
        if vault_state.owner != partnership_key || vault_state.mint != ctx.accounts.nft_mint.key() {
            return Err(error!(PartnershipError::InvalidTokenAccount));
        }
    }

    // Transfer 1 NFT from user's nft_from -> vault_ata
    let cpi_accounts = Transfer {
        from: ctx.accounts.nft_from.to_account_info(),
        to: ctx.accounts.vault_ata.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    // Update partnership state
    partnership.nft_mint = ctx.accounts.nft_mint.key();
    partnership.vault_ata = ctx.accounts.vault_ata.key();
    partnership.is_nft_deposited = true;

    Ok(())
}

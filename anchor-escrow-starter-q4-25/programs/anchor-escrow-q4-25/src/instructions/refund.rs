use anchor_lang::prelude::*;

use crate::Escrow;
use anchor_spl::{
    // CPI helper for dealing with associated token accounts (ATAs).
    associated_token::AssociatedToken,
    // Token interface helpers — we call `transfer_checked` and `close_account`
    // as CPIs against the token program via the Token Interface wrapper.
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    // The on-chain `Escrow` account that stores escrow metadata.
    //
    // - `mut`: we will write to it (Anchor will zero it on close).
    // - `close = maker`: when this account is closed, lamports are sent to `maker`.
    // - `has_one = mint_a` / `has_one = maker`: Anchor enforces the stored
    //    escrow fields for safety (prevents mismatched accounts).
    // - `seeds` / `bump`: this account is a PDA derived from the maker and seed
    //    so we can sign CPI calls on its behalf using the PDA seeds.
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow.seed.to_le_bytes()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    // The token vault ATA owned by the PDA `escrow` that holds the maker's
    // `mint_a` tokens while the escrow is active.
    //
    // - `mut`: we will transfer tokens out of this account and then close it.
    // - `associated_token::authority = escrow`: the owner of the ATA is the
    //    escrow PDA (not the maker). This allows the program-derived account
    //    to be the token account authority.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        // Recreate the PDA signer seeds so the program can sign CPIs on behalf
        // of the `escrow` PDA. This is required because the vault's authority
        // is the PDA and CPI calls require the authority to sign.
        //
        // The outer type is &[&[&[u8]]] to satisfy Anchor's `new_with_signer`
        // API (one signer, with the PDA seed slice).
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bump],
        ]];

        // Build the accounts required for `TransferChecked` CPI. We use
        // `transfer_checked` (rather than `transfer`) to ensure the mint
        // decimals are validated by the token program during transfer.
        let transfer_accounts = TransferChecked {
            // source token account (vault owned by PDA)
            from: self.vault.to_account_info(),
            // the mint of the tokens being transferred
            mint: self.mint_a.to_account_info(),
            // destination ATA for the maker
            to: self.maker_ata_a.to_account_info(),
            // authority that signs the transfer — the escrow PDA
            authority: self.escrow.to_account_info(),
        };

        // Create a CPI context that includes the signer seeds so Anchor will
        // perform a program-derived address (PDA) signature for the escrow
        // PDA when invoking the token program.
        let tranfer_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        // Transfer all tokens from the vault back to the maker's ATA. We pass
        // `self.vault.amount` to move every token in the vault, and use the
        // mint's decimal to satisfy `transfer_checked`'s requirements.
        transfer_checked(tranfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        // After transferring tokens out, close the vault token account. The
        // rent-exempt lamports held by the token account will be returned to
        // the maker (destination) because the `close` parameter on the
        // `escrow` account was set to `maker` in the `Accounts` struct.
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        // Close the vault via CPI. This will reclaim the account lamports and
        // make the vault's token account data uninitialized.
        close_account(close_cpi_ctx)
    }
}

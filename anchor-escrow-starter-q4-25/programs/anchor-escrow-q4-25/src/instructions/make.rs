use anchor_lang::prelude::*;


// Initializes the Escrow record and stores all the terms.
// Creates the Vault (an ATA for mint_a owned by the escrow).
// Moves the maker's Token A into that vault with a CPI to the SPL-Token program.

// The accounts needed in this context are:

// maker: the user that decides the terms and deposits the mint_a into the Escrow
// escrow: the account holding the exchange terms (maker, mints, amounts)
// mint_a: the token that the maker is depositing
// mint_b: the token that the maker wants in exchange
// maker_ata_a: the token account associated with the maker and mint_a used to deposit tokens in the vault
// vault: the token account associated with the escrow and mint_a where deposited tokens are parked
// associated_token_program: the associated token program used to create the associated token accounts
// token_program: the token program used to CPI the transfer
// system_program: the system program used to create the Escrow

use crate::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::errors::EscrowError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    // token account 
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    // token account
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    // token account
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Escrow::DISCRIMINATOR.len() + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    // token account
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow,
        });
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let tranfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), tranfer_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;

        Ok(())
    }

    pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        // Validate the amount
        require_gt!(receive, 0, EscrowError::InvalidAmount);
        require_gt!(amount, 0, EscrowError::InvalidAmount);
    
        // Save the Escrow Data
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
    
        // Deposit Tokens
        ctx.accounts.deposit(amount)?;
    
        Ok(())
    }
}

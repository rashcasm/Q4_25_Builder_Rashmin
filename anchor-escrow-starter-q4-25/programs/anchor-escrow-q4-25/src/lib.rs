use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("5UFZzEt5vU9fxtUAgsD11z63ApZEHJ5bH7Z4QpFwZ2CQ");

#[program]
pub mod anchor_escrow_q4_25 {
    use super::*;
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}

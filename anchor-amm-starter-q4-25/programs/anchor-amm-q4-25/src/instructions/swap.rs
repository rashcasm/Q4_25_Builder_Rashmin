use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{errors::AmmError, state::Config};

#[derive(Accounts)]
pub struct Swap<'info> {
 #[account(mut)]
 pub user:Signer<'info>,
 pub mint_x:Account<'info,Mint>,
pub mint_y:Account<'info,Mint>,

 #[account(
    mut,
    seeds=[b"config",config.seed.to_le_bytes().as_ref()],
    bump=config.config_bump,
 )]
 pub config:Account<'info,Config>,

     #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,

  #[account(
    mut,
    associated_token::mint=mint_x,
    associated_token::authority=config
  )]
 pub vault_x:Account<'info,TokenAccount>,

  #[account(
    mut,
    associated_token::mint=mint_y,
    associated_token::authority=config
  )]
 pub vault_y:Account<'info,TokenAccount>,

  #[account(
    mut,
    associated_token::mint=mint_x,
    associated_token::authority=user
  )]
  pub user_x:Account<'info,TokenAccount>,

  #[account(
    mut,
    associated_token::mint=mint_y,
    associated_token::authority=user
  )]
  pub user_y:Account<'info,TokenAccount>,


 pub associated_token_program: Program<'info, AssociatedToken>,
 pub token_program:Program<'info,Token>,
 pub system_program:Program<'info,System>,
}

impl<'info> Swap<'info> {

    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {

     require!(self.config.locked==false,AmmError::PoolLocked);
     require!(self.mint_lp.supply>0,AmmError::NoLiquidityInPool);
     require!(amount != 0, AmmError::InvalidAmount);

     let mut c: ConstantProduct=ConstantProduct::init(self.vault_x.amount,self.vault_y.amount,self.mint_lp.supply,self.config.fee,Some(6)).unwrap();


    let swap_res=match is_x {
      true => c.swap(LiquidityPair::X, amount, min).unwrap(),
      false => c.swap(LiquidityPair::Y, amount, min).unwrap()
    };
     

     self.deposit_tokens(is_x, swap_res.deposit)?;
     self.withdraw_tokens(is_x, swap_res.withdraw)?;

     Ok(())
}

    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {

        let (from,to)=match is_x {
            true=>(
             self.user_x.to_account_info(),
             self.vault_x.to_account_info()
            ),
            false => (
            self.user_y.to_account_info(),
            self.vault_y.to_account_info()
            )
        };

        let cpi_accounts=Transfer{
            from,
            to,
            authority:self.user.to_account_info()
        };

        let cpi_ctx=CpiContext::new(
            self.token_program.to_account_info(),
            cpi_accounts
        );

        transfer(cpi_ctx, amount)

}

    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from,to)=match is_x {
             true => (
                self.vault_y.to_account_info(),
                self.user_y.to_account_info()
             ),
             false=>(
                self.vault_x.to_account_info(),
                self.user_x.to_account_info()
             )
        };

        let cpi_accounts=Transfer{
            from,
            to,
            authority:self.config.to_account_info()
        };

        let signer_seeds: &[&[&[u8]]]=&[&[b"config",&self.config.seed.to_le_bytes(),&[self.config.config_bump]]];

        let cpi_ctx=CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);


        transfer(cpi_ctx, amount)
}

}
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8
}

// seed: Random number used during seed derivation so one maker
//       can open multiple escrows with the same token pair;
//       stored on-chain so we can always re-derive the PDA.
// maker: The wallet that created the escrow;
//        needed for refunds and to receive payment.
// mint_a & mint_b: The SPL mints addresses for the "give" and "get"
//                  sides of the swap.
// receive: How much of token B the maker wants.
//          (The vault's balance itself shows how much token A
//           was deposited, so we don't store that.)
// bump: Cached bump byte; deriving it on the fly costs compute,
//       so we save it once.
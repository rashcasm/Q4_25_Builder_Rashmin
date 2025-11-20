use anchor_lang::prelude::*;

#[account]
pub struct Partnership {
    /// The authority (creator) who initializes and manages this partnership
    pub authority: Pubkey,

    /// The NFT mint that has been locked
    pub nft_mint: Pubkey,

    /// The vault token account (PDA-owned) holding the NFT
    pub vault_ata: Pubkey,

    /// The SPL mint address for fractional shares
    pub share_mint: Pubkey,

    /// Total number of shares to mint (e.g., 10,000)
    pub total_shares: u64,

    /// An arbitrary ID to differentiate multiple partnerships from same authority
    pub partnership_id: u64,

    /// Flags for flow control
    pub is_nft_deposited: bool,
    pub is_shares_minted: bool,

    /// PDA bumps for seeds
    pub bumps: PartnershipBumps,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PartnershipBumps {
    pub partnership: u8,
    pub share_mint: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ShareDistribution {
    pub wallet: Pubkey,
    pub amount: u64,
}

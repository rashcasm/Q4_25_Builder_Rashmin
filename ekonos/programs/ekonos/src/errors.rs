use anchor_lang::prelude::*;

#[error_code]
pub enum PartnershipError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    #[msg("NFT has already been deposited")]
    NftAlreadyDeposited,
    #[msg("NFT not deposited yet")]
    NftNotDeposited,
    #[msg("Shares already minted")]
    SharesAlreadyMinted,
    #[msg("Invalid total shares or distribution")]
    InvalidShareDistribution,
    #[msg("Invalid token account provided")]
    InvalidTokenAccount,
}

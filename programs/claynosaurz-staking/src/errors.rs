use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    // General Errors
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic underflow")]
    Underflow,

    // NFT Related Errors
    #[msg("You passed an NFT with the wrong collection")]
    WrongCollection,
    #[msg("You passed an Invalid Metadata Account")]
    InvalidMetadata,
    #[msg("The Class PDA for this NFT already exists")]
    ClassPdaAlreadyExists,
    #[msg("The owner of this account is the wrong one")]
    WrongOwner,
    #[msg("The mint of this account is the wrong one")]
    WrongMint,
    #[msg("There are no NFTs in the account")]
    WrongAmount,
    #[msg("The remaining accounts schema is not the correct one")]
    InvalidRemainingAccountSchema,

    // Staking Related Errors
    #[msg("The user has never staked")]
    NeverStaked,
    #[msg("The user does not have enough points")]
    NotEnoughPoints,
    #[msg("The user does not have anything staked")]
    NotStaked,
    #[msg("The authority is not correct")]
    WrongAuthority,
}  
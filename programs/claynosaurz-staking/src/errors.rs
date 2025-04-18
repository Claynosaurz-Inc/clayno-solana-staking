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
    #[msg("You passed an unverified collection")]
    UnverifiedCollection,
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
    #[msg("The token record supplied is not valid")]
    InvalidTokenRecord,

    // Staking Related Errors
    #[msg("The user has never staked")]
    NeverStaked,
    #[msg("The user does not have enough points")]
    NotEnoughPoints,
    #[msg("The user is already at the maximum level")]
    AlreadyAtMaximumLevel,
    #[msg("The user does not have anything staked")]
    NotStaked,
    #[msg("The authority is not correct")]
    WrongAuthority,
    #[msg("The expiry time is not greater than the current time")]
    InvalidExpiryTime,
    #[msg("The asset is locked")]
    AssetLocked,

    // Class Related Errors
    #[msg("The multiplier needs to be greater than 1 (1x is the base multiplier already)")]
    InvalidMultiplier,
    #[msg("The lock time is not valid")]
    InvalidLockTime,
}  

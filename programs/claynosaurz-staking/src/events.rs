use anchor_lang::prelude::*;

#[event]
pub struct StakingAccountCreated {
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct StakingAccountUpdated {
    pub owner: Pubkey,
    pub points: u64,
    pub current_multiplier: u16,
    pub last_claimed: i64,
    pub timestamp: i64,
}
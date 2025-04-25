use anchor_lang::prelude::*;

use crate::state::EphemeralMultiplier;

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
    pub ephemeral_multiplier: Vec<EphemeralMultiplier>,
    pub last_claimed: i64,
    pub timestamp: i64,
}

#[event]
pub struct ClaynoUpdated {
    pub clayno_id: Pubkey,
    pub multiplier: u16,
    pub is_staked: bool,
    pub lock_time: i64,
    pub timestamp: i64,
}

#[event]
pub struct StakingAccountLevelUpdated {
    pub owner: Pubkey,
    pub points: u64,
    pub level: u8,
    pub timestamp: i64,
}
use anchor_lang::prelude::*;

use crate::events::{StakingAccountLevelUpdated, StakingAccountUpdated};
use crate::state::StakingData;
use crate::errors::StakingError;
use crate::constant::{STAKING_ACCOUNT_SEED, MAX_LEVEL};

/// Increases the level of the staking account by 1 if the user has enough points.
pub fn increase_level(ctx: Context<CrankingAction>) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    // Check if the user doesn't already have the max level
    require!(staking_account.current_level < MAX_LEVEL, StakingError::AlreadyAtMaximumLevel);
    
    // Check if the user has ever staked
    require_neq!(staking_account.last_claimed, 0, StakingError::NeverStaked);

    // Update current points
    let account_info = staking_account.to_account_info();
    staking_account.update_points(Clock::get()?.unix_timestamp, &account_info)?;

    // Check if the user has enough points
    require!(staking_account.has_enough_points()?, StakingError::NotEnoughPoints);

    // Update the level
    staking_account.current_level = staking_account.current_level.checked_add(1).ok_or(StakingError::Overflow)?;

    // Emit staking account update event
    emit!(StakingAccountLevelUpdated {
        owner: staking_account.owner,
        points: staking_account.points,
        level: staking_account.current_level,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

/// Claims points from the staking account and updates the last claimed timestamp.
pub fn claim(ctx: Context<CrankingAction>) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;
    
    // Check if the user has ever staked
    require_neq!(staking_account.last_claimed, 0, StakingError::NeverStaked);

    // Update the points
    let account_info = staking_account.to_account_info();
    staking_account.update_points(Clock::get()?.unix_timestamp, &account_info)?;

    // Emit staking account update event
    emit!(StakingAccountUpdated {
        owner: staking_account.owner,
        points: staking_account.points,
        current_multiplier: staking_account.current_multiplier,
        ephemeral_multiplier: staking_account.ephemeral_multiplier.clone(),
        last_claimed: staking_account.last_claimed,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CrankingAction<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [STAKING_ACCOUNT_SEED.as_bytes(), user.key().as_ref()],
        bump = staking_account.bump,
    )]
    pub staking_account: Account<'info, StakingData>,
}
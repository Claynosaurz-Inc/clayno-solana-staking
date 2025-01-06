use anchor_lang::prelude::*;

use crate::state::StakingData;
use crate::errors::StakingError;
use crate::constant::STAKING_ACCOUNT_SEED;

/// Increases the level of the staking account by 1 if the user has enough points.
pub fn increase_level(ctx: Context<CrankingAction>) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;
    
    // Check if the user has ever staked
    require_neq!(staking_account.last_claimed, 0, StakingError::NeverStaked);

    // Update current points
    staking_account.update_points(Clock::get()?.unix_timestamp)?;

    // Check if the user has enough points
    require!(staking_account.has_enough_points()?, StakingError::NotEnoughPoints);

    // Update the level
    staking_account.current_level = staking_account.current_level.checked_add(1).ok_or(StakingError::Overflow)?;

    Ok(())
}

/// Claims points from the staking account and updates the last claimed timestamp.
pub fn claim(ctx: Context<CrankingAction>) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;
    
    // Check if the user has ever staked
    require_neq!(staking_account.last_claimed, 0, StakingError::NeverStaked);

    // Update the points
    staking_account.update_points(Clock::get()?.unix_timestamp)?;

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
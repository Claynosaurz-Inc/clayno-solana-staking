use anchor_lang::prelude::*;

use crate::constant::{EXPERIENCE_FOR_SECOND, MAX_LEVEL, LEVEL_25_POINTS};
use crate::errors::StakingError;

/// Represents a temporary multiplier with an expiry time.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct EphemeralMultiplier {
    pub multiplier: u8,
    pub expiry_time: i64,
}

impl Space for EphemeralMultiplier {
    const INIT_SPACE: usize = 1 + 8; // Multiplier (u8) + Expiry time (i64)
}

impl EphemeralMultiplier {
    /// Checks if the multiplier is still active based on the current time.
    pub fn is_active(&self, current_time: i64) -> bool {
        current_time < self.expiry_time
    }

    /// Calculates partial points based on the time since last claimed.
    pub fn calculate_partial_points(&self, last_claimed: i64) -> Result<u64> {
        let partial_time = self.expiry_time
            .checked_sub(last_claimed)
            .ok_or(StakingError::Underflow)? as u64;
            
        Ok(EXPERIENCE_FOR_SECOND
            .checked_mul(partial_time)
            .ok_or(StakingError::Overflow)?
            .checked_mul(self.multiplier as u64)
            .ok_or(StakingError::Overflow)?)
    }
}

/// Represents the staking data for a user.
#[account]
#[derive(Debug)]
pub struct StakingData {
    pub owner: Pubkey,
    pub current_level: u8,
    pub points: u64,
    pub current_multiplier: u16,
    pub ephemeral_multiplier: Vec<EphemeralMultiplier>,
    pub last_claimed: i64,
    pub bump: u8,
}

impl Space for StakingData {
    const INIT_SPACE: usize = 8 + 32 + 1 + 8 + 2 + 4 + 8 + 1;
}

impl StakingData {
    /// Updates the points based on the current time and active multipliers.
    pub fn update_points(&mut self, current_time: i64) -> Result<()> {
        let mut total_points = 0u64;
        let mut active_multipliers = Vec::with_capacity(self.ephemeral_multiplier.len());
        let mut total_multiplier = self.current_multiplier as u64;
        
        // Process all multipliers in a single pass
        for &ephemeral_multiplier in self.ephemeral_multiplier.iter() {
            if ephemeral_multiplier.is_active(current_time) {
                // Active multiplier: add to total and keep
                total_multiplier = total_multiplier
                    .checked_add(ephemeral_multiplier.multiplier as u64)
                    .ok_or(StakingError::Overflow)?;
                active_multipliers.push(ephemeral_multiplier);
            } else if ephemeral_multiplier.expiry_time > self.last_claimed {
                // Expired multiplier: partial points calculation
                let partial_points = ephemeral_multiplier.calculate_partial_points(self.last_claimed)?;
                total_points = total_points
                    .checked_add(partial_points)
                    .ok_or(StakingError::Overflow)?;
            }
        }

        // Calculate points for the active period with combined multiplier
        let active_time = current_time
            .checked_sub(self.last_claimed)
            .ok_or(StakingError::Underflow)? as u64;
            
        total_points = total_points
            .checked_add(
                EXPERIENCE_FOR_SECOND
                    .checked_mul(active_time)
                    .ok_or(StakingError::Overflow)?
                    .checked_mul(total_multiplier)
                    .ok_or(StakingError::Overflow)?
            )
            .ok_or(StakingError::Overflow)?;

        // Update state
        self.points = self.points.checked_add(total_points).ok_or(StakingError::Overflow)?;
        self.last_claimed = current_time;
        self.ephemeral_multiplier = active_multipliers;

        Ok(())
    }

    /// Checks if the user has enough points to level up.
    pub fn has_enough_points(&self) -> Result<bool> {
        let required_points = self.calculate_points_for_level(self.current_level + 1)?;
        
        Ok(self.points >= required_points)
    }

    /// Calculates the points required for a given level.
    fn calculate_points_for_level(&self, level: u8) -> Result<u64> {
        if level == MAX_LEVEL {
            return Ok(LEVEL_25_POINTS);
        }
        
        // First multiply level by 1_000 for precision, using u128 for calculations
        let normalized_level = (level as u128)
            .checked_mul(1_000)
            .ok_or(StakingError::Overflow)?
            .checked_div(MAX_LEVEL as u128)
            .ok_or(StakingError::Underflow)?;
            
        // Calculate (normalized_level)^4 with u128
        let power = normalized_level
            .checked_mul(normalized_level)
            .ok_or(StakingError::Overflow)?
            .checked_mul(normalized_level)
            .ok_or(StakingError::Overflow)?
            .checked_mul(normalized_level)
            .ok_or(StakingError::Overflow)?;
            
        // Multiply by LEVEL_25_POINTS and divide by 1_000^4 to normalize
        let points = power
            .checked_mul(LEVEL_25_POINTS as u128)
            .ok_or(StakingError::Overflow)?
            .checked_div(1_000_000_000_000)  // 1_000^4
            .ok_or(StakingError::Underflow)?;
            
        // Convert back to u64 for return
        Ok(points.try_into().map_err(|_| StakingError::Overflow)?)
    }
}
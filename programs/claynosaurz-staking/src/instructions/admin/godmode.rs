use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

use crate::state::{StakingData, EphemeralMultiplier};
use crate::errors::StakingError;
use crate::constant::{STAKING_ACCOUNT_SEED, ADMIN_ADDRESS};

/// Adds experience points to the staking account.
pub fn add_experience(ctx: Context<GodMode>, amount: u64) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    // Update current points
    staking_account.update_points(Clock::get()?.unix_timestamp)?;

    // Add experience
    staking_account.points = staking_account.points.checked_add(amount).ok_or(StakingError::Overflow)?;

    Ok(())
}

/// Adds multiplier to the staking account.
pub fn add_multiplier(ctx: Context<GodMode>, additional_multiplier: u16) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    staking_account.current_multiplier = staking_account.current_multiplier
        .checked_add(additional_multiplier)
        .ok_or(StakingError::Overflow)?;

    Ok(())
}

/// Adds an ephemeral multiplier to the staking account and updates the data length if necessary.
pub fn add_ephemeral_multiplier(ctx: Context<GodMode>, multiplier: u8, expiry_time: i64) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    // Update current points
    staking_account.update_points(Clock::get()?.unix_timestamp)?;

    // Add Ephemeral Multiplier
    staking_account.ephemeral_multiplier.push(EphemeralMultiplier {multiplier, expiry_time});

    // Calculate the data length
    let data_len = StakingData::INIT_SPACE
        .checked_add(staking_account.ephemeral_multiplier.len().
            checked_mul(EphemeralMultiplier::INIT_SPACE)
            .ok_or(StakingError::Overflow)?
        ).ok_or(StakingError::Overflow)?;

    // Verify if the data_len is greater than the current data_len, if so, resize the account
    if data_len > staking_account.to_account_info().data_len() {
        let new_minimum_balance = Rent::get()?.minimum_balance(data_len);

        let lamports_diff = new_minimum_balance.saturating_sub(staking_account.to_account_info().lamports());

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.admin.to_account_info(),
                    to: staking_account.to_account_info(),
                },
            ),
            lamports_diff,
        )?;

        staking_account.to_account_info().realloc(data_len, false)?;
    }

    Ok(())
}

/// Removes experience points from the staking account.
pub fn remove_experience(ctx: Context<GodMode>, amount: u64) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    // Update current points
    staking_account.update_points(Clock::get()?.unix_timestamp)?;

    // Remove experience
    staking_account.points = staking_account.points
        .checked_sub(amount)
        .ok_or(StakingError::Underflow)?;

    Ok(())
}

/// Removes an multiplier from the staking account.
pub fn remove_multiplier(ctx: Context<GodMode>, multiplier: u16) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    staking_account.current_multiplier = staking_account.current_multiplier
        .checked_sub(multiplier)
        .ok_or(StakingError::Underflow)?;

    Ok(())
}

/// Reclaims rent from unused space in the staking account.
pub fn reclaim_rent(ctx: Context<GodMode>) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;

    // Update current points
    staking_account.update_points(Clock::get()?.unix_timestamp)?;

    // Calculate the data length
    let data_len = StakingData::INIT_SPACE
        .checked_add(staking_account.ephemeral_multiplier.len().
            checked_mul(EphemeralMultiplier::INIT_SPACE)
            .ok_or(StakingError::Overflow)?
        ).ok_or(StakingError::Overflow)?;
    
    // Verify if the data_len is less than the current data_len, if so, resize the account
    if data_len < staking_account.to_account_info().data_len() {
        let new_minimum_balance = Rent::get()?.minimum_balance(data_len);

        let bindings = ctx.accounts.user.key();
        let seeds = [
            STAKING_ACCOUNT_SEED.as_bytes(), 
            bindings.as_ref(), 
            &[staking_account.bump]
        ];
        let signer = &[&seeds[..]];
        let lamports_diff = new_minimum_balance.saturating_sub(staking_account.to_account_info().lamports());

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from:staking_account.to_account_info(),
                    to: ctx.accounts.admin.to_account_info(),
                },
                signer,
            ),
            lamports_diff,
        )?;

        staking_account.to_account_info().realloc(data_len, true)?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct GodMode<'info> {
    #[account(address = ADMIN_ADDRESS.parse::<Pubkey>().unwrap())]
    pub admin: Signer<'info>,
    /// CHECK: this is fine because it's a "godmode" state
    pub user: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [STAKING_ACCOUNT_SEED.as_bytes(), user.key().as_ref()],
        bump = staking_account.bump,
    )]
    pub staking_account: Account<'info, StakingData>,
    pub system_program: Program<'info, System>,
}
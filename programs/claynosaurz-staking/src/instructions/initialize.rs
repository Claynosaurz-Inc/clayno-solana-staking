use anchor_lang::prelude::*;

use crate::state::StakingData;
use crate::constant::STAKING_ACCOUNT_SEED;
use crate::events::StakingAccountCreated;

/// Initializes a new staking account with default values.
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // Create a new StakingData instance with initial values
    let staking_data = StakingData {
        owner: *ctx.accounts.user.key,
        current_level: 0,
        points: 0,
        current_multiplier: 0,
        last_claimed: 0,
        ephemeral_multiplier: vec![],
        bump: ctx.bumps.staking_account,
    };

    // Set the staking account's inner data to the newly created StakingData
    ctx.accounts.staking_account.set_inner(staking_data);

    // Emit an event indicating the creation of a new staking account
    emit!(StakingAccountCreated {
        owner: ctx.accounts.staking_account.owner,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = StakingData::INIT_SPACE,
        seeds = [STAKING_ACCOUNT_SEED.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub staking_account: Account<'info, StakingData>,
    pub system_program: Program<'info, System>,
}
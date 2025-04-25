use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

mod state;
mod events;
mod constant;
mod errors;

declare_id!("EhZDXNbFJFbTxLWd1pPPV4TWBwyZD5dn1D6Eg1cJgDyG");

#[program]
pub mod clayno_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn stake(ctx: Context<StakingAction>, lock: u8) -> Result<()> {
        instructions::staking_action::stake(ctx, lock)
    }

    pub fn unstake(ctx: Context<StakingAction>) -> Result<()> {
        instructions::staking_action::unstake(ctx)
    }

    pub fn claim(ctx: Context<CrankingAction>) -> Result<()> {
        instructions::cranking_action::claim(ctx)
    }

    pub fn increase_level(ctx: Context<CrankingAction>) -> Result<()> {
        instructions::cranking_action::increase_level(ctx)
    }

    // Admin Instructions
    pub fn create_class(ctx: Context<CreateClass>, multiplier: u16) -> Result<()> {
        instructions::admin::create_class(ctx, multiplier)
    }

    pub fn modify_class(ctx: Context<ModifyClass>, multiplier: u16, lock: u8) -> Result<()> {
        instructions::admin::modify_class(ctx, multiplier, lock)
    }

    pub fn add_experience(ctx: Context<GodMode>, amount: u64) -> Result<()> {
        instructions::admin::add_experience(ctx, amount)
    }

    pub fn remove_experience(ctx: Context<GodMode>, amount: u64) -> Result<()> {
        instructions::admin::remove_experience(ctx, amount)
    }

    pub fn add_multiplier(ctx: Context<GodMode>, additional_multiplier: u16) -> Result<()> {
        instructions::admin::add_multiplier(ctx, additional_multiplier)
    }

    pub fn remove_multiplier(ctx: Context<GodMode>, multiplier: u16) -> Result<()> {
        instructions::admin::remove_multiplier(ctx, multiplier)
    }

    pub fn add_ephemeral_multiplier(ctx: Context<GodMode>, multiplier: u8, expiry_time: i64) -> Result<()> {
        instructions::admin::add_ephemeral_multiplier(ctx, multiplier, expiry_time)
    }

    pub fn remove_ephemeral_multiplier(ctx: Context<GodMode>) -> Result<()> {
        instructions::admin::remove_ephemeral_multiplier(ctx)
    }

    pub fn reclaim_rent(ctx: Context<GodMode>) -> Result<()> {
        instructions::admin::reclaim_rent(ctx)
    }

}
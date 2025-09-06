use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::ID as SYSVAR_INSTRUCTIONS_ID;
use anchor_spl::token::{Token, TokenAccount, Mint};

use mpl_token_metadata::instructions::{DelegateStakingV1CpiBuilder, LockV1CpiBuilder, RevokeStakingV1CpiBuilder, UnlockV1CpiBuilder};
use mpl_token_metadata::accounts::Metadata;

use crate::state::{StakingData, Class};
use crate::errors::StakingError;
use crate::constant::{AUTHORITY_SEED, CLASS_PDA_SEED, CLAYNO_COLLECTION_ADDRESS, SAGA_COLLECTION_ADDRESS, STAKING_ACCOUNT_SEED, SHORT_LOCKUP, MEDIUM_LOCKUP, LONG_LOCKUP, MAX_LOCKUP, TEST_LOCKUP};
use crate::events::{StakingAccountUpdated, ClaynoUpdated};

/// Stakes an NFT by delegating it to the global authority PDA.
pub fn stake(ctx: Context<StakingAction>, lock: u8) -> Result<()> {
    // Update staking data
    let staking_account = &mut ctx.accounts.staking_account;

    msg!("Lock: {}", lock);

    // Update last claimed timestamp and points
    if staking_account.last_claimed != 0 {
        let account_info = staking_account.to_account_info();
        staking_account.update_points(Clock::get()?.unix_timestamp, &account_info)?;
    } else {
        staking_account.last_claimed = Clock::get()?.unix_timestamp;
    }

    // Handle lock period and class
    if lock != 0 {
        // If lock is specified, class must exist
        let mut class = Class::try_deserialize(&mut &ctx.accounts.class_pda.to_account_info().data.borrow_mut()[..])
            .map_err(|_| error!(StakingError::ClassNotFound))?;

        // Set the lock time based on the lock parameter
        class.lock_time = match lock {
            1 => Clock::get()?.unix_timestamp + SHORT_LOCKUP,
            2 => Clock::get()?.unix_timestamp + MEDIUM_LOCKUP,
            3 => Clock::get()?.unix_timestamp + LONG_LOCKUP,
            4 => Clock::get()?.unix_timestamp + MAX_LOCKUP,
            5 => Clock::get()?.unix_timestamp + TEST_LOCKUP,
            _ => return Err(error!(StakingError::InvalidLockTime)),
        };

        // Serialize the updated class back to the account
        class.try_serialize(&mut &mut ctx.accounts.class_pda.to_account_info().data.borrow_mut()[..])?;

        msg!("Class: {:?}", class.lock_time);

        // Update staking account multiplier
        staking_account.current_multiplier = staking_account.current_multiplier
            .checked_add(class.multiplier)
            .ok_or(StakingError::Overflow)?;
    } else {
        // If no lock specified, class is optional
        if let Ok(class) = Class::try_deserialize(&mut &ctx.accounts.class_pda.to_account_info().data.borrow_mut()[..]) {
            staking_account.current_multiplier = staking_account.current_multiplier
                .checked_add(class.multiplier)
                .ok_or(StakingError::Overflow)?;
        } else {
            staking_account.current_multiplier = staking_account.current_multiplier
                .checked_add(1)
                .ok_or(StakingError::Overflow)?;
        }
    }

    // Deserialize Metadata to verify collection
    let nft_metadata = Metadata::safe_deserialize(&mut ctx.accounts.nft_metadata.to_account_info().data.borrow_mut()).unwrap();
    if let Some(collection) = nft_metadata.collection {
        if collection.key.to_string() != CLAYNO_COLLECTION_ADDRESS && collection.key.to_string() != SAGA_COLLECTION_ADDRESS {
            return Err(error!(StakingError::WrongCollection));
        }
        if collection.verified != true {
            return Err(error!(StakingError::UnverifiedCollection));
        }
    } else {
        return Err(error!(StakingError::InvalidMetadata));
    };

    // Lock NFT to global authority PDA
    let seeds = &[AUTHORITY_SEED.as_bytes(), &[ctx.bumps.auth]];
    let delegate_seeds = &[&seeds[..]];

    DelegateStakingV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .delegate(&ctx.accounts.auth.to_account_info())
        .metadata(&ctx.accounts.nft_metadata.to_account_info())
        .master_edition(Some(ctx.accounts.nft_edition.as_ref()))
        .token_record(Some(ctx.accounts.nft_record.as_ref()))
        .mint(&ctx.accounts.nft.to_account_info())
        .token(&ctx.accounts.nft_account.to_account_info())
        .authority(&ctx.accounts.user)
        .payer(&ctx.accounts.user)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
        .authorization_rules_program(Some(ctx.accounts.auth_rules_program.as_ref()))
        .authorization_rules(Some(ctx.accounts.auth_rules.as_ref()))
        .amount(1)
        .invoke()?;

    LockV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .authority(&ctx.accounts.auth.to_account_info().clone())
        .token_owner(Some(&ctx.accounts.user))
        .token(&ctx.accounts.nft_account.to_account_info())
        .mint(&ctx.accounts.nft.to_account_info())
        .metadata(&ctx.accounts.nft_metadata)
        .edition(Some(ctx.accounts.nft_edition.as_ref()))
        .token_record(Some(ctx.accounts.nft_record.as_ref()))
        .payer(&ctx.accounts.user)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
        .authorization_rules_program(Some(ctx.accounts.auth_rules_program.as_ref()))
        .authorization_rules(Some(ctx.accounts.auth_rules.as_ref()))
        .invoke_signed(delegate_seeds)?;

    // Emit staking account update event
    emit!(StakingAccountUpdated {
        owner: staking_account.owner,
        points: staking_account.points,
        current_multiplier: staking_account.current_multiplier,
        ephemeral_multiplier: staking_account.ephemeral_multiplier.clone(),
        last_claimed: staking_account.last_claimed,
        timestamp: Clock::get()?.unix_timestamp,
    });

    // Emit clayno update event
    if let Ok(class) = Class::try_deserialize(&mut &ctx.accounts.class_pda.to_account_info().data.borrow_mut()[..]) {
        emit!(ClaynoUpdated {
            clayno_id: ctx.accounts.nft.key(),
            multiplier: class.multiplier,
            is_staked: true,
            lock_time: class.lock_time,
            timestamp: Clock::get()?.unix_timestamp,
        });
        msg!("Emitting ClaynoUpdated: {:?}", ClaynoUpdated {
            clayno_id: ctx.accounts.nft.key(),
            multiplier: class.multiplier,
            is_staked: true,
            lock_time: class.lock_time,
            timestamp: Clock::get()?.unix_timestamp,
        });
    } else {
        emit!(ClaynoUpdated {
            clayno_id: ctx.accounts.nft.key(),
            multiplier: 1,
            is_staked: true,
            lock_time: 0,
            timestamp: Clock::get()?.unix_timestamp,
        });
        msg!("Emitting ClaynoUpdated: {:?}", ClaynoUpdated {
            clayno_id: ctx.accounts.nft.key(),
            multiplier: 1,
            is_staked: true,
            lock_time: 0,
            timestamp: Clock::get()?.unix_timestamp,
        });
    };

    Ok(())
}

/// Unstakes an NFT by revoking the delegation and unlocking the NFT.
pub fn unstake(ctx: Context<StakingAction>) -> Result<()> {
    // Update staking data
    let staking_account = &mut ctx.accounts.staking_account;

    // Update points
    let account_info = staking_account.to_account_info();
    staking_account.update_points(Clock::get()?.unix_timestamp, &account_info)?;

    // Adjust multiplier based on class PDA ownership (default to 1 if no class PDA)
    if let Ok(class) = Class::try_deserialize(&mut &ctx.accounts.class_pda.to_account_info().data.borrow_mut()[..]) {
        msg!("Class: {:?}", class.lock_time);
        
        staking_account.current_multiplier = staking_account.current_multiplier
            .checked_sub(class.multiplier)
            .ok_or(StakingError::Overflow)?;
        
        let current_time = Clock::get()?.unix_timestamp;
        match class.lock_time {
            0 => {},
            _ => {
                if current_time < class.lock_time {
                    return Err(error!(StakingError::AssetLocked));
                }
            }
        }
    } else {
        staking_account.current_multiplier = staking_account.current_multiplier
            .checked_sub(1)
            .ok_or(StakingError::Overflow)?;
    }

    // Unlock NFT
    let seeds = &[AUTHORITY_SEED.as_bytes(), &[ctx.bumps.auth]];
    let delegate_seeds = &[&seeds[..]];

    UnlockV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .authority(&ctx.accounts.auth.to_account_info().clone())
        .token_owner(Some(&ctx.accounts.user))
        .token(&ctx.accounts.nft_account.to_account_info())
        .mint(&ctx.accounts.nft.to_account_info())
        .metadata(&ctx.accounts.nft_metadata)
        .edition(Some(ctx.accounts.nft_edition.as_ref()))
        .token_record(Some(ctx.accounts.nft_record.as_ref()))
        .payer(&ctx.accounts.user)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
        .authorization_rules_program(Some(ctx.accounts.auth_rules_program.as_ref()))
        .authorization_rules(Some(ctx.accounts.auth_rules.as_ref()))
        .invoke_signed(delegate_seeds)?;

    RevokeStakingV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .delegate(&ctx.accounts.auth.to_account_info())
        .metadata(&ctx.accounts.nft_metadata.to_account_info())
        .master_edition(Some(ctx.accounts.nft_edition.as_ref()))
        .token_record(Some(ctx.accounts.nft_record.as_ref()))
        .mint(&ctx.accounts.nft.to_account_info())
        .token(&ctx.accounts.nft_account.to_account_info())
        .authority(&ctx.accounts.user)
        .payer(&ctx.accounts.user)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
        .authorization_rules_program(Some(ctx.accounts.auth_rules_program.as_ref()))
        .authorization_rules(Some(ctx.accounts.auth_rules.as_ref()))
        .invoke()?;

    // Emit staking account update event
    emit!(StakingAccountUpdated {
        owner: staking_account.owner,
        points: staking_account.points,
        current_multiplier: staking_account.current_multiplier,
        ephemeral_multiplier: staking_account.ephemeral_multiplier.clone(),
        last_claimed: staking_account.last_claimed,
        timestamp: Clock::get()?.unix_timestamp,
    });

    // Emit clayno update event
    if let Ok(class) = Class::try_deserialize(&mut &ctx.accounts.class_pda.to_account_info().data.borrow_mut()[..]) {
        emit!(ClaynoUpdated {
            clayno_id: ctx.accounts.nft.key(),
            multiplier: class.multiplier,
            is_staked: false,
            lock_time: class.lock_time,
            timestamp: Clock::get()?.unix_timestamp,
        });
    } else {
        emit!(ClaynoUpdated {
            clayno_id: ctx.accounts.nft.key(),
            multiplier: 1,
            is_staked: false,
            lock_time: 0,
            timestamp: Clock::get()?.unix_timestamp,
        });
    };

    Ok(())
}

#[derive(Accounts)]
pub struct StakingAction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: this is fine
    #[account(seeds = [AUTHORITY_SEED.as_bytes()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [STAKING_ACCOUNT_SEED.as_bytes(), user.key().as_ref()],
        bump = staking_account.bump,
    )]
    pub staking_account: Account<'info, StakingData>,
    /// CHECK: this will be deserialized later
    #[account(mut, seeds = [CLASS_PDA_SEED.as_bytes(), nft.key().as_ref()], bump)]
    pub class_pda: UncheckedAccount<'info>,

    /// NFT Accounts
    pub nft: Box<Account<'info, Mint>>,
    #[account(
        mut, 
        token::mint = nft, 
        token::authority = user,
    )]
    pub nft_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: instruction will fail if wrong edition is supplied
    pub nft_edition: UncheckedAccount<'info>,
    /// CHECK: instruction will fail if wrong record is supplied
    #[account(mut)]
    pub nft_record: UncheckedAccount<'info>,
    /// CHECK: instruction will fail if wrong metadata is supplied
    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK: instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,    
    /// CHECK: instruction will fail if wrong sysvar ixns are supplied
    #[account(address = SYSVAR_INSTRUCTIONS_ID)]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// Programs
    pub token_program: Program<'info, Token>,
    /// CHECK: instruction will fail if wrong program is supplied
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: instruction will fail if wrong program is supplied
    pub auth_rules_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>
}


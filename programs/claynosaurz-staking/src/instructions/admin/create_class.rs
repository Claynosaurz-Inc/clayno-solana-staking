use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

use mpl_token_metadata::accounts::TokenRecord;
use mpl_token_metadata::types::TokenState;

use crate::errors::StakingError;
use crate::state::{Class, StakingData};
use crate::constant::{CLASS_PDA_SEED, AUTHORITY_SEED, ADMIN_ADDRESS}; 
use crate::events::StakingAccountUpdated;

/// Creates a new class PDA and initializes it with the necessary data.
pub fn create_class(ctx: Context<CreateClass>, multiplier: u16) -> Result<()> {
    // Check if multiplier is greater than 0
    require_gte!(multiplier, 1, StakingError::InvalidMultiplier);

    // Populate the Class PDA with the multiplier
    ctx.accounts.class_pda.set_inner(Class { multiplier });

    // Check if the asset is staked and update staking data if necessary
    let record = TokenRecord::safe_deserialize(&mut ctx.accounts.token_mint_record.to_account_info().data.borrow_mut()).unwrap();

    // Check that the token record is valid
    let token_record_address = TokenRecord::find_pda(&ctx.accounts.token_mint.key(), &ctx.accounts.token_account.key()).0;
    require_eq!(token_record_address, ctx.accounts.token_mint_record.key(), StakingError::InvalidTokenRecord);
    
    if record.state == TokenState::Locked {
        let (auth_pda, _) = Pubkey::find_program_address(&[AUTHORITY_SEED.as_bytes()], &crate::ID);
        require!(record.delegate == Some(auth_pda), StakingError::WrongAuthority);
        
        // Get owner, staking_data, and token_account from remaining_accounts
        let remaining_accounts = &ctx.remaining_accounts[..];
        if let [owner, staking_account] = remaining_accounts {
            // Deserialize staking_data
            let mut staking_account_data = StakingData::try_deserialize(&mut &staking_account.try_borrow_data()?[..])?;
            require_eq!(staking_account_data.owner, *owner.key, StakingError::WrongOwner);

            // Token Account Checks
            require_eq!(ctx.accounts.token_account.owner, *owner.key, StakingError::WrongOwner);
            require_eq!(ctx.accounts.token_account.mint, ctx.accounts.token_mint.key(), StakingError::WrongMint);
            require_eq!(ctx.accounts.token_account.amount, 1, StakingError::WrongAmount);

            // Update staking_data
            staking_account_data.update_points(Clock::get()?.unix_timestamp, &staking_account)?;
            staking_account_data.current_multiplier = staking_account_data.current_multiplier
                .checked_add(multiplier - 1u16)
                .ok_or(StakingError::Overflow)?;

            // Serialize staking_data back to the account
            staking_account_data.try_serialize(&mut &mut staking_account.try_borrow_mut_data()?[..])?;

            // Emit event
            emit!(StakingAccountUpdated {
                owner: *owner.key,
                points: staking_account_data.points,
                current_multiplier: staking_account_data.current_multiplier,
                ephemeral_multiplier: staking_account_data.ephemeral_multiplier.clone(),
                last_claimed: staking_account_data.last_claimed,
                timestamp: Clock::get()?.unix_timestamp,
            });
        } else {
            return Err(error!(StakingError::InvalidRemainingAccountSchema));
        }
    };

    Ok(())
}

#[derive(Accounts)]
pub struct CreateClass<'info> {
    #[account(mut, address = ADMIN_ADDRESS.parse::<Pubkey>().unwrap())]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Class::INIT_SPACE,
        seeds = [CLASS_PDA_SEED.as_bytes(), token_mint.key().as_ref()], 
        bump
    )]
    /// CHECK: this is safe since we're going to derive the address from the seeds
    pub class_pda: Account<'info, Class>,

    /// NFT Accounts
    pub token_mint: Box<Account<'info, Mint>>,
    pub token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: instruction will fail if wrong token_record is supplied
    pub token_mint_record: UncheckedAccount<'info>,

    /// Programs
    pub system_program: Program<'info, System>,
}
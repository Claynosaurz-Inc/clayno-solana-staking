use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer, assign, Assign};
use anchor_spl::token::{TokenAccount, Mint};

use mpl_token_metadata::accounts::TokenRecord;
use mpl_token_metadata::types::TokenState;

use crate::errors::StakingError;
use crate::state::StakingData;
use crate::constant::{CLASS_PDA_SEED, AUTHORITY_SEED, ADMIN_ADDRESS}; 
use crate::events::StakingAccountUpdated;

/// Creates a new class PDA and initializes it with the necessary data.
pub fn create_class(ctx: Context<CreateClass>) -> Result<()> {
    // Ensure the class PDA does not already exist
    require_neq!(ctx.accounts.class_pda.owner, &crate::ID, StakingError::ClassPdaAlreadyExists);

    // Calculate the minimum balance required for rent exemption
    let lamports = Rent::get()?.minimum_balance(128);

    // Transfer lamports to the class PDA
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.admin.to_account_info(),
                to: ctx.accounts.class_pda.to_account_info(),
            }
        ),
        lamports
    )?;

    // Prepare seeds for signing
    let bindings = ctx.accounts.token_mint.key();
    let seeds = &[
        CLASS_PDA_SEED.as_bytes(), 
        bindings.as_ref(),
        &[ctx.bumps.class_pda]
    ];
    let signer_seeds = &[&seeds[..]];

    // Assign the class PDA to the program ID
    assign(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Assign {
                account_to_assign: ctx.accounts.class_pda.to_account_info(),
            },
            signer_seeds
        ),
        &crate::ID,
    )?;

    // Check if the asset is staked and update staking data if necessary
    let (auth_pda, _) = Pubkey::find_program_address(&[AUTHORITY_SEED.as_bytes()], &crate::ID);
    let record = TokenRecord::safe_deserialize(&mut ctx.accounts.token_mint_record.to_account_info().data.borrow_mut()).unwrap();

    if record.state == TokenState::Locked {
        require!(record.delegate == Some(auth_pda), StakingError::WrongAuthority);
        
        // Get owner, staking_data, and token_account from remaining_accounts
        let remaining_accounts = &ctx.remaining_accounts[..];
        if let [owner, staking_account, token_account] = remaining_accounts {
            // Deserialize staking_data
            let mut staking_account_data = StakingData::try_deserialize(&mut &staking_account.try_borrow_data()?[..])?;
            require_eq!(staking_account_data.owner, *owner.key, StakingError::WrongOwner);

            // Deserialize token_account
            let token_account_data = TokenAccount::try_deserialize(&mut &token_account.try_borrow_data()?[..])?;
            require_eq!(token_account_data.owner, *owner.key, StakingError::WrongOwner);
            require_eq!(token_account_data.mint, ctx.accounts.token_mint.key(), StakingError::WrongMint);
            require_eq!(token_account_data.amount, 1, StakingError::WrongAmount);

            // Update staking_data
            staking_account_data.update_points(Clock::get()?.unix_timestamp)?;
            staking_account_data.current_multiplier = staking_account_data.current_multiplier
                .checked_add(1)
                .ok_or(StakingError::Overflow)?;

            // Serialize staking_data back to the account
            staking_account_data.try_serialize(&mut &mut staking_account.try_borrow_mut_data()?[..])?;

            // Emit event
            emit!(StakingAccountUpdated {
                owner: *owner.key,
                points: staking_account_data.points,
                current_multiplier: staking_account_data.current_multiplier,
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
    #[account(seeds = [CLASS_PDA_SEED.as_bytes(), token_mint.key().as_ref()], bump)]
    /// CHECK: this is safe since we're going to derive the address from the seeds
    pub class_pda: UncheckedAccount<'info>,

    /// NFT Accounts
    pub token_mint: Box<Account<'info, Mint>>,
    /// CHECK: instruction will fail if wrong token_record is supplied
    pub token_mint_record: UncheckedAccount<'info>,

    /// Programs
    pub system_program: Program<'info, System>,
}
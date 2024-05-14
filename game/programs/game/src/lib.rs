use anchor_lang::prelude::*;

pub mod error;
pub mod utils;

use error::*;
use utils::*;

declare_id!("9uuTtDj7eDyYU1koiiaXsQcwFBSV2M9VquSzJMfbSvT2");

#[program]
pub mod game {
    use super::*;

        
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.sol_vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            ctx.accounts.rent.minimum_balance(0),
        )?;
        
        Ok(())
    }

    pub fn desposit(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {

        require!(
            ctx.accounts.admin.lamports() > sol_amount,
            GameError::InsufficientSolBalance
        );
        
        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.sol_vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            sol_amount
        )?;
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, sol_amount: u64, vault_bump: u8) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == BE_WALLET.parse::<Pubkey>().unwrap(),
            GameError::InsufficientSolBalance
        );


        require!(
            ctx.accounts.sol_vault.lamports() > sol_amount,
            GameError::InsufficientSolBalance
        );

        sol_transfer_with_signer(
            ctx.accounts.sol_vault.to_account_info().clone(),
            ctx.accounts.receiver.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            &[&[VAULT_SEED.as_ref(), &[vault_bump]]],
            sol_amount,
        )?;
        
        Ok(())
    }

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    // The admin account is the first initailize account- the first payer's pubkey
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,
    
    // These are system and rent
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // The admin account is the first initailize account- the first payer's pubkey
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,
    
    // These are system and rent
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // The admin account is the first initailize account- the first payer's pubkey
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub receiver: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,
    
    // These are system and rent
    pub system_program: Program<'info, System>,
}

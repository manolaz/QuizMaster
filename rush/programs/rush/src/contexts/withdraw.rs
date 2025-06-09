use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::{Config, GameVault};
use crate::constants::{CONFIG, GAME_VAULT_STATE, GAME_VAULT};
use crate::errors::RushError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Config to verify admin
    #[account(
        seeds = [CONFIG, admin.key().as_ref()], 
        bump,
        constraint = config.admin == admin.key() @ RushError::Unauthorized
    )]
    pub config: Account<'info, Config>,

    // Vault state to track funding
    #[account(
        mut,
        seeds = [GAME_VAULT_STATE, admin.key().as_ref()], 
        bump
    )]
    pub vault_state: Account<'info, GameVault>,

    // The vault PDA to receive funds
    #[account(
        mut,
        seeds = [GAME_VAULT, vault_state.key().as_ref()], 
        bump = vault_state.bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> Withdraw<'info> {
    pub fn withdraw_vault(&mut self, amount: u64) -> Result<()> {
        // Validate amount
        require!(amount > 0, RushError::InvalidAmount);

        // Check vault has sufficient balance
        let vault_balance = self.vault.lamports();
        require!(vault_balance >= amount, RushError::InsufficientVaultBalance);

        let vault_state_key = self.vault_state.key();
        let seeds = &[
            GAME_VAULT,
            vault_state_key.as_ref(),
            &[self.vault_state.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Transfer from vault to admin (vault signs as PDA)
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(), 
            to: self.admin.to_account_info(),  
        };
        
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds 
        );
        
        transfer(cpi_ctx, amount)?;
        
        self.vault_state.total_withdrawn += amount;

        msg!("Admin withdrew {} lamports from vault", amount);
        msg!("Remaining vault balance: {} lamports", self.vault.lamports() - amount);

        Ok(())
    }
}


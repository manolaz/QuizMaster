use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::{Config, GameVault};
use crate::constants::{CONFIG, GAME_VAULT_STATE, GAME_VAULT};
use crate::errors::RushError;

#[derive(Accounts)]
pub struct FundVault<'info> {
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

impl<'info> FundVault<'info> {
    pub fn fund_vault(&mut self, amount: u64) -> Result<()> {
        // Validate amount
        require!(amount > 0, RushError::InvalidAmount);

        // Transfer from admin to vault
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.admin.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer(cpi_ctx, amount)?;

        msg!("Vault funded with {} lamports", amount);
        msg!("Vault balance: {} lamports", self.vault.lamports());

        Ok(())
    }
}
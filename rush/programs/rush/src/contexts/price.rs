use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::{Session, SessionStatus, Config, GameVault};
use crate::constants::{SESSION, CONFIG, GAME_VAULT_STATE, GAME_VAULT};
use crate::errors::RushError;

#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct DistributePrizes<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Session - now on mainnet with winners populated
    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
        constraint = session.status == SessionStatus::Completed @ RushError::SessionNotCompleted,
        // Ensure winners are actually set
        constraint = session.winners[0] != Pubkey::default() @ RushError::WinnersNotSet,
        constraint = session.winners[1] != Pubkey::default() @ RushError::WinnersNotSet,
    )]
    pub session: Account<'info, Session>,

    #[account(seeds = [CONFIG, admin.key().as_ref()], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [GAME_VAULT_STATE, admin.key().as_ref()], bump)]
    pub vault_state: Account<'info, GameVault>,

    #[account(
        mut, 
        seeds = [GAME_VAULT, vault_state.key().as_ref()], 
        bump = vault_state.bump
    )]
    pub vault: SystemAccount<'info>,

    /// CHECK: This is the first place winner account
    #[account(mut, constraint = winner1.key() == session.winners[0] @ RushError::InvalidWinnerAccount)]
    pub winner1: UncheckedAccount<'info>,

    /// CHECK: This is the second place winner account
    #[account(mut, constraint = winner2.key() == session.winners[1] @ RushError::InvalidWinnerAccount)]
    pub winner2: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> DistributePrizes<'info> {
    pub fn distribute_prizes(&mut self, _session_id: [u8; 32]) -> Result<()> {

    require!(
    !self.session.prizes_distributed,
    RushError::PrizesAlreadyDistributed
    );
    self.session.prizes_distributed = true;


        // Get winners for logging
        let stored_winner1 = self.session.winners[0];
        let stored_winner2 = self.session.winners[1];

        // since game has ended, update the vault date. 
        self.vault_state.total_games_played += 1;
     

        let cpi_program = self.system_program.to_account_info();

        // Prepare signer seeds for the vault PDA
        let seeds = &[
            GAME_VAULT,
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Transfer first prize
        let cpi_accounts_first = Transfer {
            from: self.vault.to_account_info(),
            to: self.winner1.to_account_info(),
        };
        let cpi_ctx_first = CpiContext::new_with_signer(
            cpi_program.clone(), 
            cpi_accounts_first, 
            signer_seeds
        );
        transfer(cpi_ctx_first, self.config.first_prize)?;

        // Transfer second prize
        let cpi_accounts_second = Transfer {
            from: self.vault.to_account_info(),
            to: self.winner2.to_account_info(),
        };
        let cpi_ctx_second = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts_second, 
            signer_seeds
        );
        transfer(cpi_ctx_second, self.config.second_prize)?;

        // Update vault state
        self.vault_state.total_disbursed += self.config.first_prize + self.config.second_prize;

        msg!("Prizes distributed to stored winners:");
        msg!("1st: {} - {} lamports", stored_winner1, self.config.first_prize);
        msg!("2nd: {} - {} lamports", stored_winner2, self.config.second_prize);

        Ok(())
    }
}
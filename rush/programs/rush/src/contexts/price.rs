use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::{Config, GameVault, Session, SessionStatus, PlayerState};
use crate::constants::{SESSION, CONFIG, GAME_VAULT_STATE, GAME_VAULT, PLAYER_STATE};
use crate::errors::RushError;

#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct DistributePrizes<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Session - now on base layer after undelegation, with winners populated
    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
        constraint = session.status == SessionStatus::Completed @ RushError::SessionNotCompleted,
        constraint = session.winners[0] != Pubkey::default() @ RushError::WinnersNotSet,
        constraint = session.winners[1] != Pubkey::default() @ RushError::WinnersNotSet,
    )]
    pub session: Account<'info, Session>,

    #[account(
        seeds = [CONFIG, admin.key().as_ref()], 
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut, 
        seeds = [GAME_VAULT_STATE, admin.key().as_ref()], 
        bump
    )]
    pub vault_state: Account<'info, GameVault>,

    #[account(
        mut, 
        seeds = [GAME_VAULT, vault_state.key().as_ref()], 
        bump = vault_state.bump
    )]
    pub vault: SystemAccount<'info>,

    // Winner 1 PlayerState
    #[account(
        mut,
        seeds = [PLAYER_STATE, session.winners[0].as_ref()],
        bump,
        constraint = winner1_player_state.player == session.winners[0] @ RushError::InvalidWinnerAccount
    )]
    pub winner1_player_state: Account<'info, PlayerState>,

    // Winner 2 PlayerState
    #[account(
        mut,
        seeds = [PLAYER_STATE, session.winners[1].as_ref()],
        bump,
        constraint = winner2_player_state.player == session.winners[1] @ RushError::InvalidWinnerAccount
    )]
    pub winner2_player_state: Account<'info, PlayerState>,

    /// CHECK: Winner 1 wallet (validated through PlayerState constraint)
    #[account(
        mut,
        constraint = winner1_wallet.key() == winner1_player_state.player @ RushError::InvalidWinnerAccount
    )]
    pub winner1_wallet: UncheckedAccount<'info>,

    /// CHECK: Winner 2 wallet (validated through PlayerState constraint)
    #[account(
        mut,
        constraint = winner2_wallet.key() == winner2_player_state.player @ RushError::InvalidWinnerAccount
    )]
    pub winner2_wallet: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> DistributePrizes<'info> {
    pub fn distribute_prizes(&mut self, _session_id: [u8; 32]) -> Result<()> {
        // Check if prizes already distributed
        require!(
            !self.session.prizes_distributed,
            RushError::PrizesAlreadyDistributed
        );

        // Mark prizes as distributed
        self.session.prizes_distributed = true;

        // Update game count
        self.vault_state.total_games_played += 1;

        // Get prize amounts
        let first_prize = self.config.first_prize;
        let second_prize = self.config.second_prize;

        let cpi_program = self.system_program.to_account_info();

        // Prepare signer seeds for the vault PDA
        let vault_state_key = self.vault_state.key();
        let seeds = &[
            GAME_VAULT,
            vault_state_key.as_ref(),
            &[self.vault_state.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Transfer first prize to winner 1's wallet
        let cpi_accounts_first = Transfer {
            from: self.vault.to_account_info(),
            to: self.winner1_wallet.to_account_info(),
        };
        let cpi_ctx_first = CpiContext::new_with_signer(
            cpi_program.clone(), 
            cpi_accounts_first, 
            signer_seeds
        );
        transfer(cpi_ctx_first, first_prize)?;

        // Update winner 1 PlayerState total_earnings
        self.winner1_player_state.total_earnings += first_prize;

        // Transfer second prize to winner 2's wallet
        let cpi_accounts_second = Transfer {
            from: self.vault.to_account_info(),
            to: self.winner2_wallet.to_account_info(),
        };
        let cpi_ctx_second = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts_second, 
            signer_seeds
        );
        transfer(cpi_ctx_second, second_prize)?;

        // Update winner 2 PlayerState total_earnings
        self.winner2_player_state.total_earnings += second_prize;

        // Update vault state
        self.vault_state.total_disbursed += first_prize + second_prize;

        msg!("Prizes distributed successfully:");
        msg!("1st place: {} - {} lamports (total earnings: {})", 
             self.winner1_player_state.player, 
             first_prize,
             self.winner1_player_state.total_earnings);
        msg!("2nd place: {} - {} lamports (total earnings: {})", 
             self.winner2_player_state.player, 
             second_prize,
             self.winner2_player_state.total_earnings);
        msg!("Total games played: {}", self.vault_state.total_games_played);

        Ok(())
    }
}
use anchor_lang::prelude::*;
use crate::state::{PlayerSessionAnswer, PlayerState, Session, SessionStatus};
use crate::constants::{PLAYER_STATE, PLAYER_SESSION_ANSWER, SESSION};
use crate::errors::RushError;

// MagicBlock SDK
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;

// Context for joining a session in ER
// PSA account must already exist and be delegated to ER
#[commit]
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct JoinSession<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    // Player state must exist (profile created)
    #[account(
        mut,
        seeds = [PLAYER_STATE, player.key().as_ref()],
        bump,
        constraint = player_state.created_at != 0 @ RushError::ProfileNotCreated
    )]
    pub player_state: Account<'info, PlayerState>,

    // PSA must already exist and be delegated to ER
    #[account(
        mut,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, player.key().as_ref()],
        bump,
        constraint = player_session_answer.player == player.key() @ RushError::ProfileNotCreated,
        constraint = player_session_answer.session_id == session_id @ RushError::PlayerNotInSession,
    )]
    pub player_session_answer: Account<'info, PlayerSessionAnswer>,

    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
    )]
    pub session: Account<'info, Session>,
}

impl<'info> JoinSession<'info> {
    pub fn player_join_session(
        &mut self,
        _session_id: [u8; 32],
    ) -> Result<()> {
        // Check session is joinable
        require!(
            self.session.status == SessionStatus::Initialized,
            RushError::SessionAlreadyStarted
        );

        // Check session not full
        require!(
            self.session.current_players < 4,
            RushError::SessionFull
        );

        // Check player not already in session
        require!(
            !self.session.players.contains(&self.player.key()),
            RushError::PlayerInSession
        );

        // Update PSA to active state (account already exists)
        self.player_session_answer.is_active = true;
        self.player_session_answer.score = 0;
        self.player_session_answer.final_position = 0;
        self.player_session_answer.answers = [255; 10];

        // Add player to session (this is committed to ER)
        let player_index = self.session.current_players;
        self.session.players[player_index as usize] = self.player.key();
        self.session.current_players += 1;

        // Update player stats
        self.player_state.quizzes_joined += 1;

        // Check if session is ready (all 4 players joined)
        if self.session.current_players == 4 {
            self.session.status = SessionStatus::Ready;
            msg!("Session ready to start with 4 players!");
        }

        commit_accounts(
            &self.player,
            vec![&self.session.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        msg!("Player {} joined session", self.player.key());
        msg!("Current players: {}/4", self.session.current_players);

        Ok(())
    }
}
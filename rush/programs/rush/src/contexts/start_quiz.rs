use anchor_lang::prelude::*;
use crate::state::{Session, SessionStatus};
use crate::constants::{SESSION};
use crate::errors::RushError;

// Start Quiz - Runs in rollup (accounts already delegated)

#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct StartQuiz<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Session account - already delegated to rollup
    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
        constraint = session.creator == admin.key() @ RushError::Unauthorized
    )]
    pub session: Account<'info, Session>,
}

impl<'info> StartQuiz<'info> {
    pub fn start_quiz(
        &mut self,
        session_id: [u8; 32],
    ) -> Result<()> {
        // Verify session is ready to start
        require!(
            self.session.status == SessionStatus::Ready,
            RushError::SessionNotReady
        );
        
        require!(
            self.session.current_players == 4,
            RushError::InsufficientPlayers
        );

        let clock = Clock::get()?;

        // Update session status and timing in rollup (real-time!)
        self.session.status = SessionStatus::Live;
        self.session.started_at = clock.unix_timestamp as u64;
        self.session.question_start_time = clock.unix_timestamp;

        msg!("Quiz started in MagicBlock rollup!");
        msg!("Session ID: {:?}", session_id);
        msg!("Status changed to Live - game begins!");
        msg!("Questions already loaded from create_session");

        Ok(())
    }
}
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use crate::state::{Session, SessionData, SessionStatus, PlayerSessionAnswer, PlayerState};
use crate::constants::{SESSION, SESSIONDATA, PLAYER_SESSION_ANSWER, PLAYER_STATE};
use crate::errors::RushError;

// END GAME : End game + commit and undelegate in one instruction
#[commit]
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct EndGameAndUndelegate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Session account (delegated)
    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
        constraint = session.creator == admin.key() @ RushError::Unauthorized
    )]
    pub session: Account<'info, Session>,

    // Session data account - read only
    #[account(
        seeds = [SESSIONDATA, admin.key().as_ref(), &session_id],
        bump,
    )]
    pub session_data: Account<'info, SessionData>,

    // All 4 PlayerState accounts (delegated, need updates)
    #[account(
        mut,
        seeds = [PLAYER_STATE, session.players[0].as_ref()],
        bump,
    )]
    pub player1_state: Account<'info, PlayerState>,

    #[account(
        mut,
        seeds = [PLAYER_STATE, session.players[1].as_ref()],
        bump,
    )]
    pub player2_state: Account<'info, PlayerState>,

    #[account(
        mut,
        seeds = [PLAYER_STATE, session.players[2].as_ref()],
        bump,
    )]
    pub player3_state: Account<'info, PlayerState>,

    #[account(
        mut,
        seeds = [PLAYER_STATE, session.players[3].as_ref()],
        bump,
    )]
    pub player4_state: Account<'info, PlayerState>,

    // All 4 PlayerSessionAnswer accounts (delegated)
    #[account(
        mut,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[0].as_ref()],
        bump,
    )]
    pub player1_session_answer: Account<'info, PlayerSessionAnswer>,

    #[account(
        mut,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[1].as_ref()],
        bump,
    )]
    pub player2_session_answer: Account<'info, PlayerSessionAnswer>,

    #[account(
        mut,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[2].as_ref()],
        bump,
    )]
    pub player3_session_answer: Account<'info, PlayerSessionAnswer>,

    #[account(
        mut,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[3].as_ref()],
        bump,
    )]
    pub player4_session_answer: Account<'info, PlayerSessionAnswer>,
}

impl<'info> EndGameAndUndelegate<'info> {
    pub fn end_game_and_undelegate(&mut self, _session_id: [u8; 32]) -> Result<()> {
        // Verify game is live
        require!(
            self.session.status == SessionStatus::Live,
            RushError::SessionInactive
        );

        // Calculate winners from the final scores 
        let player_scores = [
            (self.session.players[0], self.player1_session_answer.score),
            (self.session.players[1], self.player2_session_answer.score),
            (self.session.players[2], self.player3_session_answer.score),
            (self.session.players[3], self.player4_session_answer.score),
        ];

        // Sort by highest score first 
        let mut sorted_scores = player_scores;
        sorted_scores.sort_by(|a, b| b.1.cmp(&a.1)); 

        // Set winners
        self.session.winners[0] = sorted_scores[0].0;
        self.session.winners[1] = sorted_scores[1].0;

        // Update final positions in PSA accounts
        for (i, (player, _score)) in sorted_scores.iter().enumerate() {
            let final_position = (i + 1) as u8; // 1st, 2nd, 3rd, 4th

            if *player == self.session.players[0] {
                self.player1_session_answer.final_position = final_position;
            } else if *player == self.session.players[1] {
                self.player2_session_answer.final_position = final_position;
            } else if *player == self.session.players[2] {
                self.player3_session_answer.final_position = final_position;
            } else if *player == self.session.players[3] {
                self.player4_session_answer.final_position = final_position;
            }
        }

        // Update PlayerState accounts with real-time stats
        // Player 1
        self.player1_state.quizzes_joined += 1;
        self.player1_state.total_points += self.player1_session_answer.score;
        if self.session.winners.contains(&self.session.players[0]) {
            self.player1_state.quizzes_won += 1;
        }

        // Player 2
        self.player2_state.quizzes_joined += 1;
        self.player2_state.total_points += self.player2_session_answer.score;
        if self.session.winners.contains(&self.session.players[1]) {
            self.player2_state.quizzes_won += 1;
        }

        // Player 3
        self.player3_state.quizzes_joined += 1;
        self.player3_state.total_points += self.player3_session_answer.score;
        if self.session.winners.contains(&self.session.players[2]) {
            self.player3_state.quizzes_won += 1;
        }

        // Player 4
        self.player4_state.quizzes_joined += 1;
        self.player4_state.total_points += self.player4_session_answer.score;
        if self.session.winners.contains(&self.session.players[3]) {
            self.player4_state.quizzes_won += 1;
        }

        // Update session status to completed
        self.session.status = SessionStatus::Completed;

        // Commit and undelegate ALL DELEGATED accounts back to mainnet
        commit_and_undelegate_accounts(
            &self.admin,
            vec![
                // Session
                &self.session.to_account_info(),
                // PlayerState accounts (now delegated)
                &self.player1_state.to_account_info(),
                &self.player2_state.to_account_info(),
                &self.player3_state.to_account_info(),
                &self.player4_state.to_account_info(),
                // PlayerSessionAnswer accounts
                &self.player1_session_answer.to_account_info(),
                &self.player2_session_answer.to_account_info(),
                &self.player3_session_answer.to_account_info(),
                &self.player4_session_answer.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;

        msg!("Game ended and accounts undelegated");
        msg!("Winners calculated and stored:");
        msg!("1st place: {} with {} points", sorted_scores[0].0, sorted_scores[0].1);
        msg!("2nd place: {} with {} points", sorted_scores[1].0, sorted_scores[1].1);
        msg!("session.winners = [{}, {}]", self.session.winners[0], self.session.winners[1]);

        Ok(())
    }
}
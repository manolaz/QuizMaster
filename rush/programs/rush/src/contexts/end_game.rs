use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use crate::state::{Session, SessionData, SessionStatus, PlayerSessionAnswer};
use crate::constants::{SESSION, SESSIONDATA, PLAYER_SESSION_ANSWER};
use crate::errors::RushError;

// END GAME : End game + commit and undelegate in one instruction
#[commit]
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct EndGameAndUndelegate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // ACCOUNTS TO UNDELEGATE
    // Session account
    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
        constraint = session.creator == admin.key() @ RushError::Unauthorized
    )]
    pub session: Account<'info, Session>,

    // Session data account
    #[account(
        mut,
        seeds = [SESSIONDATA, admin.key().as_ref(), &session_id],
        bump,
    )]
    pub session_data: Account<'info, SessionData>,

    // All 4 PlayerSessionAnswer accounts explicitly defined
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
        //calculate winners from the final scores 
          let player_scores = [
            (self.session.players[0], self.player1_session_answer.score),
            (self.session.players[1], self.player2_session_answer.score),
            (self.session.players[2], self.player3_session_answer.score),
            (self.session.players[3], self.player4_session_answer.score),
        ];

       //sort by highest first 

       let mut sorted_scores = player_scores;
       sorted_scores.sort_by(|a, b| b.1.cmp(&a.1)); 

       self.session.winners[0] = sorted_scores[0].0;
       self.session.winners[1] = sorted_scores[1].0;

         for (i, (player, _score)) in sorted_scores.iter().enumerate() {
            let final_position = (i + 1) as u8; // 1st, 2nd, 3rd, 4th

            // Find which PlayerSessionAnswer to update
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

        // Update session status to completed
        self.session.status = SessionStatus::Completed;

        // Commit and undelegate all accounts back to mainnet
        commit_and_undelegate_accounts(
            &self.admin,
            vec![
                &self.session.to_account_info(),
                &self.session_data.to_account_info(),
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
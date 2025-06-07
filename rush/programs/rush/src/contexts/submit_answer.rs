use anchor_lang::prelude::*;
use crate::state::{
    Session, SessionData, SessionStatus, Leaderboard, PlayerSessionAnswer
};
use crate::constants::{
    SESSION, SESSIONDATA, LEADERBOARD, PLAYER_SESSION_ANSWER
};
use crate::errors::RushError;

// players call to submit their answers
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct SubmitAnswer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    // Session - already in rollup
    #[account(
        mut,
        seeds = [SESSION, &session_id],
        bump,
    )]
    pub session: Account<'info, Session>,

    // Session data - already in rollup
    #[account(
        seeds = [SESSIONDATA, session.creator.as_ref(), &session_id],
        bump,
    )]
    pub session_data: Account<'info, SessionData>,

    // Player session answer - already in rollup
    #[account(
        mut,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, player.key().as_ref()],
        bump,
    )]
    pub player_session_answer: Account<'info, PlayerSessionAnswer>,

    // Global leaderboard - already in rollup
    #[account(
        mut,
        seeds = [LEADERBOARD, session.creator.as_ref()],
        bump,
    )]
    pub leaderboard: Account<'info, Leaderboard>,
}

impl<'info> SubmitAnswer<'info> {
    pub fn submit_answer(
        &mut self,
        _session_id: [u8; 32],
        question_index: u8,
        answer: u8,
    ) -> Result<()> {
        // Verify game is live
        require!(
            self.session.status == SessionStatus::Live,
            RushError::SessionInactive
        );
        // Verify valid inputs
        require!(question_index < 10, RushError::InvalidQuestionIndex);
        require!(answer < 4, RushError::InvalidAnswer);
        require!(
            self.player_session_answer.answers[question_index as usize] == 255,
            RushError::AlreadyAnswered
        );


        // Verify player is in this session
        require!(
            self.session.players.contains(&self.player.key()),
            RushError::PlayerNotInSession
        );

        // Record the answer
        self.player_session_answer.answers[question_index as usize] = answer;

        // Check if correct
        let correct_answer = self.session_data.correct_answers[question_index as usize];
        
        if answer == correct_answer {
            // Add points for correct answer
            self.player_session_answer.score += 100;
            
        //leaderboard Update
        self.leaderboard.update_player_score(
          self.player.key(), 
          self.player_session_answer.score
        )?;

        let player_rank = self.leaderboard.get_player_rank(&self.player.key());

            msg!("✅ Correct! Player: {}, New Score: {}, Rank: {}", 
                 self.player.key(), 
                 self.player_session_answer.score,
                 if player_rank > 0 { 
                     format!("#{}", player_rank) 
                 } else { 
                     "Not in top 10".to_string() 
                 }
            );
      
        } else {
            msg!("❌ Wrong answer. Player: {}, Score: {}", 
                 self.player.key(), self.player_session_answer.score);
        }

        msg!("Answer processed in rollup - real-time leaderboard updated!");
        
        Ok(())
    }
}

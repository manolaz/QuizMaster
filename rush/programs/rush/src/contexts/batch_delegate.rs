use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use crate::state::Session;
use crate::constants::{SESSION, PLAYER_SESSION_ANSWER};
use crate::errors::RushError;

// Batch delegate all PlayerSessionAnswer accounts
// Called by backend admin when session status = Ready (all 4 players joined)
#[delegate]
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct DelegateAllPlayerSessions<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    // Session to read the players array. This is possible cause the base layer can still read the state if the session account in the ER 
    #[account(
      seeds = [SESSION, &session_id],
      bump,
    )]
    pub session: Account<'info, Session>,
    
    // All 4 PlayerSessionAnswer accounts (created during join_session)

    /// CHECK: Player 1's PSA account
    #[account(
        mut, 
        del,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[0].as_ref()],
        bump,
    )]
    pub player1_session_answer: AccountInfo<'info>,
    
    /// CHECK: Player 2's PSA account  
    #[account(
        mut, 
        del,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[1].as_ref()],
        bump,
    )]
    pub player2_session_answer: AccountInfo<'info>,
    
    /// CHECK: Player 3's PSA account
    #[account(
        mut, 
        del,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[2].as_ref()],
        bump,
    )]
    pub player3_session_answer: AccountInfo<'info>,
    
    /// CHECK: Player 4's PSA account
    #[account(
        mut, 
        del,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, session.players[3].as_ref()],
        bump,
    )]
    pub player4_session_answer: AccountInfo<'info>,
}

impl<'info> DelegateAllPlayerSessions<'info> {
    pub fn delegate_all_player_sessions(&self, session_id: [u8; 32]) -> Result<()> {
        // Verify session is ready (all 4 players joined)
        require!(
            self.session.status == crate::state::SessionStatus::Ready,
            RushError::SessionNotReady
        );
        
        require!(
            self.session.current_players == 4,
            RushError::InsufficientPlayers
        );
        
        // Get the 4 players from session
        let players: [Pubkey; 4] = self.session.players;
        
        // Delegate Player 1's PSA
        let player1_seeds = [PLAYER_SESSION_ANSWER, &session_id, players[0].as_ref()];
        self.delegate_player1_session_answer(
            &self.admin,
            &player1_seeds,
            DelegateConfig::default(),
        )?;
        
        // Delegate Player 2's PSA
        let player2_seeds = [PLAYER_SESSION_ANSWER, &session_id, players[1].as_ref()];
        self.delegate_player2_session_answer(
            &self.admin,
            &player2_seeds,
            DelegateConfig::default(),
        )?;
        
        // Delegate Player 3's PSA
        let player3_seeds = [PLAYER_SESSION_ANSWER, &session_id, players[2].as_ref()];
        self.delegate_player3_session_answer(
            &self.admin,
            &player3_seeds,
            DelegateConfig::default(),
        )?;
        
        // Delegate Player 4's PSA
        let player4_seeds = [PLAYER_SESSION_ANSWER, &session_id, players[3].as_ref()];
        self.delegate_player4_session_answer(
            &self.admin,
            &player4_seeds,
            DelegateConfig::default(),
        )?;
        
        msg!("All 4 PlayerSessionAnswer accounts delegated to rollup");
        msg!("Players: {:?}", players);
        msg!("Ready for real-time score updates!");
        
        Ok(())
    }
}
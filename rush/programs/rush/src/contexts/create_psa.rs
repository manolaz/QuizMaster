use anchor_lang::prelude::*;
use crate::state::PlayerSessionAnswer;
use crate::constants::{ANCHOR_DISCRIMINATOR, PLAYER_SESSION_ANSWER};

// Context for creating PlayerSessionAnswer account on BASE LAYER
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]
pub struct CreatePlayerSessionAnswer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    // Create new player session answer for this specific game session
    #[account(
        init,
        payer = player,
        space = ANCHOR_DISCRIMINATOR + PlayerSessionAnswer::INIT_SPACE,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, player.key().as_ref()],
        bump,
    )]
    pub player_session_answer: Account<'info, PlayerSessionAnswer>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreatePlayerSessionAnswer<'info> {
    pub fn create_psa(
        &mut self,
        session_id: [u8; 32],
    ) -> Result<()> {
        // Initialize the player session answer account
        self.player_session_answer.set_inner(PlayerSessionAnswer {
            session_id,
            player: self.player.key(),
            score: 0,
            is_active: true,
            final_position: 0,
            answers: [255; 10], // 255 = not answered yet
        });

        msg!("PlayerSessionAnswer created for player: {}", self.player.key());
        msg!("Session ID: {:?}", session_id);

        Ok(())
    }
}
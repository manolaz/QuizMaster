use anchor_lang::prelude::*;
use crate::state::PlayerState;
use crate::constants::{ANCHOR_DISCRIMINATOR, PLAYER_STATE};
use crate::errors::RushError;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init,
        payer = player,
        space = ANCHOR_DISCRIMINATOR + PlayerState::INIT_SPACE,
        seeds = [PLAYER_STATE, player.key().as_ref()],
        bump,
    )]
    pub player_state: Account<'info, PlayerState>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateProfile<'info> {
    pub fn create_profile(
        &mut self,
        name: String,
    ) -> Result<()> {
        // Validate name
        require!(
            name.len() >= 3 && name.len() <= 50,
            RushError::InvalidNameLength
        );

        let clock = Clock::get()?;

        self.player_state.set_inner(PlayerState {
            player: self.player.key(),
            name,
            total_points: 0,
            quizzes_joined: 0,
            quizzes_won: 0,
            total_earnings: 0,
            created_at: clock.unix_timestamp as u64,
        });

        msg!("User profile created: {}", self.player_state.name);
        msg!("Player: {}", self.player.key());

        Ok(())
    }
}
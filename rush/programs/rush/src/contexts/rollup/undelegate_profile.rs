use anchor_lang::prelude::*;
use crate::constants::PLAYER_STATE;
use crate::state::PlayerState;

//magic block SDK
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

// undelegate single player profile
#[commit]
#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct UndelegateProfile<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    // Single PlayerState account
    #[account(
        mut,
        seeds = [PLAYER_STATE, player_pubkey.as_ref()],
        bump,
    )]
    pub player_state: Account<'info, PlayerState>,
}

impl<'info> UndelegateProfile<'info> {
    pub fn undelegate_player_profile(
      &mut self, 
      player_pubkey: Pubkey
    ) -> Result<()> {
    msg!("Undelegating PlayerState for player: {}", player_pubkey);
        
      commit_and_undelegate_accounts(
        &self.admin,
        vec![&self.player_state.to_account_info()],
        &self.magic_context,
        &self.magic_program,
        )?;
        
        msg!("✅ PlayerState account undelegated from ephemeral rollup");
        Ok(())
    }
}
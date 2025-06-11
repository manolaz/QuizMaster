use anchor_lang::prelude::*;
use crate::constants::PLAYER_STATE;

//magic block SDK
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;

// delegate the player profile
#[delegate]
#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct DelegateProfile<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: Player's State account to delegate
    #[account(
        mut,
        del,
        seeds = [PLAYER_STATE, player_pubkey.as_ref()],
        bump,
    )]
    pub player_state: AccountInfo<'info>,
}

impl<'info> DelegateProfile<'info> {
    pub fn delegate_user_profile(
        &self,
        player_pubkey: Pubkey,
    ) -> Result<()> {

        msg!("Delegating PlayerState for player: {}", player_pubkey);
  
        self.delegate_player_state(
            &self.admin,
            &[PLAYER_STATE, player_pubkey.as_ref()],
            DelegateConfig::default(),
        )?;
        
        msg!("✅ PlayerState account delegated to ephemeral rollup");
        Ok(())
    }
}
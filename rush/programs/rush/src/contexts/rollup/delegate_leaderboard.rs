use anchor_lang::prelude::*;
use crate::constants::{LEADERBOARD};

//magic block SDK 
use ephemeral_rollups_sdk::anchor::{delegate};
use ephemeral_rollups_sdk::cpi::DelegateConfig;

// delegate the leaderboard on first game 
#[delegate]
#[derive(Accounts)]
pub struct DelegateLeaderboard<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    ///CHECK : The Leaderboard account to be delegated to the ER
    #[account(
      mut,
      del,
      seeds = [LEADERBOARD, admin.key().as_ref()],
      bump,
    )]
    pub leaderboard: AccountInfo<'info>,
}

impl <'info> DelegateLeaderboard<'info> {
    pub fn delegate_global_leaderboard(
        &mut self,
    ) -> Result<()> {
      let admin = self.admin.key();
      // delegate the leaderboard account to the ER
      let leaderboard_seeds = [LEADERBOARD, admin.as_ref()];
      self.delegate_leaderboard(&self.admin, &leaderboard_seeds, DelegateConfig::default())?;
        Ok(())
    }
}
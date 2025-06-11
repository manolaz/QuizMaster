use anchor_lang::prelude::*;
use crate::constants::{LEADERBOARD};
use crate::state::Leaderboard;

//magic block SDK 
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

// undelegate the leaderboard
#[commit]
#[derive(Accounts)]
pub struct UndelegateLeaderboard<'info> {
  #[account(mut)]
  pub admin : Signer<'info>,

  //leaderboard account
  #[account(
    mut,
    seeds = [LEADERBOARD, admin.key().as_ref()],
    bump,
  )]
  pub leaderboard : Account<'info, Leaderboard>
}

impl <'info> UndelegateLeaderboard <'info>  {
  pub fn undelegate_leaderboard(&mut self) -> Result<()> {
    commit_and_undelegate_accounts(
      &self.admin, 
      vec![&self.leaderboard.to_account_info()], 
      &self.magic_context, 
      &self.magic_program
    )?;
    Ok(())
  } 
}
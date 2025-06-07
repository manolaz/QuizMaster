use anchor_lang::prelude::*;
use crate::state::{Config, GameVault, Leaderboard};
use crate::constants::{ANCHOR_DISCRIMINATOR, CONFIG, GAME_VAULT, GAME_VAULT_STATE,LEADERBOARD};
use crate::error;

//base layer
#[derive(Accounts)]
pub struct Initialize<'info> {
#[account(mut)]
pub admin: Signer<'info>,
//config pda 
#[account(
  init,
  payer = admin,
  space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
  seeds = [CONFIG, admin.key().as_ref()],
  bump,
)]
pub config : Account<'info, Config>,

//vault state 
#[account(
  init,
  payer = admin,
  space = ANCHOR_DISCRIMINATOR + GameVault::INIT_SPACE,
  seeds = [GAME_VAULT_STATE, admin.key().as_ref()],
  bump,
)]
pub vault_state : Account<'info, GameVault>,

#[account(
  seeds = [GAME_VAULT, vault_state.key().as_ref()],
  bump,
)]
 pub vault: SystemAccount<'info>,

//leaderboard 
#[account(
  init,
  payer = admin,
  space = ANCHOR_DISCRIMINATOR + Leaderboard::INIT_SPACE,
  seeds = [LEADERBOARD, admin.key().as_ref()],
  bump,
)]
pub leader_board : Account<'info, Leaderboard>,
pub system_program : Program<'info, System>
}

impl <'info> Initialize <'info> {
  pub fn init(
    &mut self,
    first_prize: u64,
    second_prize: u64,
    bump: InitializeBumps,
  ) -> Result<()> {

  // setup the config account
    self.config.set_inner(
      Config {
      admin: self.admin.key(), 
      first_prize,
      second_prize,
    }
    );

    //setup the vault state
    self.vault_state.set_inner(
      GameVault {
        total_disbursed: 0,
        bump: bump.vault
      }
    );

    //leaderboard
    self.leader_board.set_inner(
      Leaderboard {
        top_players: [Pubkey::default(); 10],
        top_scores: [0; 10],
        last_updated: 0,
      }
    );

    Ok(())
  }}
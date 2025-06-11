use anchor_lang::prelude::*;
use crate::constants::{SESSION};


//magic block SDK 
use ephemeral_rollups_sdk::anchor::{delegate};
use ephemeral_rollups_sdk::cpi::DelegateConfig;

// delegate the accounts we need to the ER so that we can work on them with the ER program as owner via a CPI call.  
#[delegate]
#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]

pub struct DelegateAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK : the session account we are delegating to the ER
     #[account(
        mut,
        del,
        seeds = [SESSION, &session_id],
        bump,
    )]
    pub session: AccountInfo<'info>,
}

impl <'info> DelegateAccounts<'info> {
    pub fn delegate_accounts(
        &mut self,
        session_id: [u8; 32],
    ) -> Result<()> {

      // delegate the session account to the ER
      let session_seeds = [SESSION, &session_id];
      self.delegate_session(
        &self.admin, 
        &session_seeds, 
        DelegateConfig::default())?;
    Ok(())
    }
}
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use crate::constants::{PLAYER_SESSION_ANSWER};
use crate::errors::RushError;

// Single PSA delegation
#[delegate]
#[derive(Accounts)]
#[instruction(session_id: [u8; 32], player_pubkey: Pubkey)]
pub struct DelegatePlayerSession<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// CHECK: Player's PSA account to delegate
    #[account(
        mut,
        del,
        seeds = [PLAYER_SESSION_ANSWER, &session_id, player_pubkey.as_ref()],
        bump,
    )]
    pub player_session_answer: AccountInfo<'info>,
}

impl<'info> DelegatePlayerSession<'info> {
    pub fn delegate_player_session(
        &self,
        session_id: [u8; 32],
        player_pubkey: Pubkey,
    ) -> Result<()> {
        require!(
            player_pubkey != Pubkey::default(),
            RushError::ProfileNotCreated
        );

        msg!("Delegating PSA for player: {}", player_pubkey);

       
        self.delegate_player_session_answer(
            &self.admin,
            &[PLAYER_SESSION_ANSWER, &session_id, player_pubkey.as_ref()],
            DelegateConfig::default(),
        )?;


        msg!("✅ PlayerSessionAnswer account delegated to ephemeral rollup");
        Ok(())
    }
}
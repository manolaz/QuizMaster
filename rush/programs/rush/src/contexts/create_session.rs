use anchor_lang::prelude::*;
use crate::state::{Session, SessionData, SessionStatus};
use crate::constants::{ANCHOR_DISCRIMINATOR, SESSION, SESSIONDATA};
use crate::error;

// This context ix should happen before the accounts are delegated.
//Base layer ix

#[derive(Accounts)]
#[instruction(session_id: [u8; 32])]

pub struct CreateSession<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
    #[account(
      init,
      payer = admin,
      space = ANCHOR_DISCRIMINATOR + SessionData::INIT_SPACE,
      seeds = [SESSIONDATA, admin.key().as_ref(), &session_id],
      bump,
    )]
    pub session_data: Account<'info, SessionData>,

  #[account(
    init,
    payer = admin,
    space = ANCHOR_DISCRIMINATOR + Session::INIT_SPACE, 
    seeds = [SESSION, &session_id],
    bump,
  )]
  pub session: Account<'info, Session>,
  pub system_program: Program<'info, System>,
}

impl <'info> CreateSession <'info> {
  pub fn create_session(
    &mut self,
    session_id: [u8; 32],
    question_ids: [u16; 10], //array of question IDs since we have a fixed number of questions from the backend
    correct_answers: [u8; 10], //array of question IDs since we have a fixed number of questions from the backend
  ) -> Result<()> {
    // set session data account 
    self.session_data.set_inner(
      SessionData{
      question_ids,
      correct_answers
      }
    );

    self.session.set_inner(
      Session { 
      session_id, 
      creator: self.admin.key(), 
      players: [Pubkey::default(); 4], 
      current_players: 0, 
      started_at: 0, 
      status: SessionStatus::Initialized, 
      question_data: self.session_data.key(), 
      winners:  [Pubkey::default(); 2],
      prizes_distributed: false
    }
    );

     msg!("Session created: {:?}", session_id);
      msg!("Creator: {}", self.admin.key());
      msg!("Questions stored on-chain");

    Ok(())
  }

}
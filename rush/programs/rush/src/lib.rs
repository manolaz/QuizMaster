use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

declare_id!("J7SqsE5DUZeNGKMYvpESEE3MyUymP8wdmXGyjydrJ2bi");

mod state;
mod constants;
mod errors;
mod contexts;

use contexts::*;

#[ephemeral]
#[program]
pub mod rush {
    use super::*;

    //initialize program
    pub fn initialize(
     ctx: Context<Initialize>,
    first_prize: u64,
    second_prize: u64,
    ) -> Result<()> {
    ctx.accounts.init(
    first_prize, 
    second_prize, 
    ctx.bumps
    )
    }

    //delegate leaderboard one time as it is global 
    pub fn delegate_leaderboard(
    ctx: Context<DelegateLeaderboard>
    ) -> Result<()> {
    ctx.accounts.delegate_global_leaderboard()
    }

    // undelegate  leaderboard
    pub fn undelegate_leaderboard(
    ctx: Context<UndelegateLeaderboard>
    ) -> Result<()> {
        ctx.accounts.undelegate_leaderboard()
    }

    // create user profile 
    pub fn create_profile(
     ctx: Context<CreateProfile>,
    name: String,
    ) -> Result<()> {
    ctx.accounts.create_profile(name)
    }

    //fund the vault from admin 
    pub fn fund_vault(
    ctx: Context<FundVault>,
    amount: u64
    ) -> Result<()> {
    ctx.accounts.fund_vault(amount)
    }

    //withdraw from vault
    pub fn withdraw( 
    ctx: Context<Withdraw>,
    amount: u64
    ) -> Result<()> {
    ctx.accounts.withdraw_vault(amount)
    }

    // create game session for users / frens to join 
    pub fn create_session(
     ctx: Context<CreateSession>,
    session_id: [u8; 32],
    question_ids: [u16; 10],
    correct_answers: [u8; 10],
    ) -> Result<()> {
   ctx.accounts.create_session(session_id, question_ids, correct_answers)
    }

    //delegate the session to the ER 
    pub fn delegate_session(
     ctx: Context<DelegateAccounts>,
    session_id: [u8; 32],
    ) -> Result<()> {
    ctx.accounts.delegate_accounts(session_id)
    }

    // join the game session
    pub fn join_game_session(
     ctx: Context<JoinSession>,
    session_id: [u8; 32],
    ) -> Result<()> {
    ctx.accounts.player_join_session(session_id)
    }

    //batch delegate all joined frens to the ER
    pub fn delegate_frens(
    ctx: Context<DelegateAllPlayerSessions>,
    session_id: [u8; 32]
    ) -> Result<()> {
    ctx.accounts.delegate_all_player_sessions(session_id)
    }


    // start quiz to start the fun 
    pub fn start_quiz(
     ctx: Context<StartQuiz>,
     session_id: [u8; 32],
    ) -> Result<()> {
    ctx.accounts.start_quiz(session_id)
    }

    //submit answer. Might be right or wrong :)
    pub fn submit_answer(
     ctx: Context<SubmitAnswer>,
    session_id: [u8; 32],
    question_index: u8,
    answer: u8,
    ) -> Result<()> {
    ctx.accounts.submit_answer(session_id, question_index, answer)
    }

    //end the game when it is done 
    pub fn end_game_session(
     ctx: Context<EndGameAndUndelegate>,
    session_id: [u8; 32]
    ) -> Result<()> {
    ctx.accounts.end_game_and_undelegate(session_id)
    }

    //disburse price to winners 
    pub fn payout(
     ctx: Context<DistributePrizes>,
    session_id: [u8; 32]
    ) -> Result<()> {
    ctx.accounts.distribute_prizes(session_id)
    }
}


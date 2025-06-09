//base layer calls
pub mod init;
pub mod profile;
pub mod create_session;
pub mod fund;

////
pub use init::*;
pub use profile::*;
pub use create_session::*;
pub use fund::*;


// delegation call i.e move to rollup
pub mod delegate_session;
pub use delegate_session::*;

//rollup calls i.e when we have accounts in ER
pub mod join_session; // x 4 as all 4 players call
pub mod batch_delegate;
pub mod start_quiz;
pub mod submit_answer;

pub use join_session::*;
pub use batch_delegate::*;
pub use start_quiz::*;
pub use submit_answer::*;


// return to mainnet calls
pub mod end_game;
pub use end_game::*;

//base calls
pub mod price;
pub use price::*;

//TODO : Since the leaderboard is a global leaderboard. we need to have a seperate context to delegate it once and then undelegate it later on. I think it should be called once and not together as the other ones we have. 

pub mod delegate_leaderboard;
pub use delegate_leaderboard::*;
pub mod withdraw;
pub use withdraw::*;
pub mod undelegate_leaderboard;
pub use undelegate_leaderboard::*;


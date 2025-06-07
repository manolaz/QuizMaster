use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
// Game program config account. 
pub struct Config {
    pub admin: Pubkey,
    pub first_prize: u64,
    pub second_prize: u64,
}

#[account]
#[derive(InitSpace)]
// player PDA, useful for the leaderboard
pub struct PlayerState {
    pub player: Pubkey,
    #[max_len(50)]
    pub name: String,
    pub total_points: u64,
    pub quizzes_joined: u64,
    pub quizzes_won: u64,
    pub total_earnings: u64,
    pub created_at: u64,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionStatus {
    Initialized,
    Ready,
    Live,
    Completed,
    Canceled,
}

impl Space for SessionStatus {
   const INIT_SPACE: usize = 1;
}

#[account]
#[derive(InitSpace)]
pub struct SessionData {
    pub question_ids : [u16; 10],
    pub correct_answers: [u8; 10], 
}

#[account]
#[derive(InitSpace)]
// Game Sessions account with id
pub struct Session {
    pub session_id: [u8; 32],
    pub creator: Pubkey,                   
    pub players: [Pubkey; 4],           
    pub current_players: u8,  // How many joined
    pub question_start_time: i64, // For timing
    pub created_at: u64,
    pub started_at: u64,
    pub ended_at: u64,
    pub status: SessionStatus,
    pub question_data: Pubkey, // pda referring to SessionData
    pub winners : [Pubkey; 2],
    pub prizes_distributed: bool,
}

#[account]
#[derive(InitSpace)]
// Player's performance in a specific session
// This PDA is created when player joins THIS specific game
pub struct PlayerSessionAnswer {
    pub session_id: [u8; 32],
    pub player: Pubkey,
    pub score: u64,
    pub is_active: bool,
    pub final_position: u8,
    pub answers: [u8; 10],
}

#[account]
#[derive(InitSpace)]
// Game vault for rewards disbursement
pub struct GameVault {
    pub total_disbursed: u64, 
    pub bump: u8,
    pub total_games_played: u64,
}


#[account]
#[derive(InitSpace)]
// Leaderboard account
pub struct Leaderboard {
pub top_players: [Pubkey; 10],
pub top_scores: [u64; 10], 
pub last_updated: i64,
}

impl Leaderboard {
    // Real-time leaderboard update method
    pub fn update_player_score(&mut self, player: Pubkey, new_score: u64) -> Result<()> {
        // Check if player is already in leaderboard
        let mut existing_position: Option<usize> = None;
        for i in 0..10 {
            if self.top_players[i] == player {
                existing_position = Some(i);
                break;
            }
        }

        match existing_position {
            Some(pos) => {
                // Player already in leaderboard - update their score
                self.top_scores[pos] = new_score;
            }
            None => {
                // Player not in leaderboard - try to add them
                self.add_new_player(player, new_score)?;
            }
        }

        // Sort leaderboard by score (descending)
        self.sort_leaderboard();
        self.last_updated = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    fn add_new_player(&mut self, player: Pubkey, score: u64) -> Result<()> {
        // Find first empty slot
        for i in 0..10 {
            if self.top_players[i] == Pubkey::default() {
                self.top_players[i] = player;
                self.top_scores[i] = score;
                return Ok(());
            }
        }

        // Leaderboard is full - check if this score beats the lowest
        if score > self.top_scores[9] {
            // Replace the last player
            self.top_players[9] = player;
            self.top_scores[9] = score;
        }
        
        Ok(())
    }

    fn sort_leaderboard(&mut self) {
        // Simple bubble sort for 10 elements (very fast)
        for i in 0..9 {
            for j in 0..(9-i) {
                if self.top_scores[j] < self.top_scores[j+1] {
                    // Swap scores
                    self.top_scores.swap(j, j+1);
                    // Swap corresponding players
                    self.top_players.swap(j, j+1);
                }
            }
        }
    }

    // Get player's current rank (1-indexed, 0 if not in top 10)
    pub fn get_player_rank(&self, player: &Pubkey) -> u8 {
        for (i, &leaderboard_player) in self.top_players.iter().enumerate() {
            if leaderboard_player == *player {
                return (i + 1) as u8;
            }
        }
        0 // Not in top 10
    }
}



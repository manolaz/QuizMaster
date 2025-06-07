pub const ANCHOR_DISCRIMINATOR: usize = 8;

// Seeds for the program's PDAs (Program Derived Addresses)

/// Seed for the Config account PDA
pub const CONFIG: &[u8] = b"config";

/// Seed for the PlayerState account PDA
pub const PLAYER_STATE: &[u8] = b"player_state";

pub const PLAYER_SESSION_ANSWER: &[u8] = b"player_session_answer";

/// Seed for the Session account PDA
pub const SESSION: &[u8] = b"session";

pub const SESSIONDATA: &[u8] = b"question";

/// Seed for the Leaderboard account PDA
pub const LEADERBOARD: &[u8] = b"leaderboard";

/// Seed for the GameVault account PDA
pub const GAME_VAULT: &[u8] = b"game_vault";

pub const GAME_VAULT_STATE: &[u8] = b"game_vault_state";

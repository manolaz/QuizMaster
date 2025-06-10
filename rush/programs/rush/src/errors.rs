use anchor_lang::*;

#[error_code]

pub enum RushError {
   #[msg("Configuration not initialized")]
    ConfigNotInitialized,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Session not found")]
    SessionNotFound,

    #[msg("Session is full")]
    SessionFull,

    #[msg("Session still active")]
    SessionNotCompleted,
    
    #[msg("Session not ready - need exactly 4 players")]
    SessionNotReady,
    

    #[msg("Session is inactive")]
    SessionInactive,

     #[msg("Name must be 3-20 characters")]
    InvalidNameLength,
    
    #[msg("Name can only contain letters, numbers, and underscores")]
    InvalidNameCharacters,
    
    #[msg("Must create profile before joining games")]
    ProfileNotCreated,

     #[msg("Profile Already Created")]
    ProfileAlreadyCreated,

    #[msg("Session already ended")]
    SessionAlreadyEnded,
    #[msg("Session already started")]
    SessionAlreadyStarted,

     #[msg("Price already distributed")]
    PrizesAlreadyDistributed,

    #[msg("No more questions in session")]
    NoMoreQuestions,

    #[msg("Player not in session")]
    PlayerNotInSession,
     #[msg("Player already in session")]
    PlayerInSession,

    #[msg("Invalid correct count")]
    InvalidCorrectCount,

    #[msg("Answers already submitted")]
    AnswersAlreadySubmitted,

    #[msg("Vault not initialized")]
    VaultNotInitialized,

    #[msg("Insufficient vault balance")]
    InsufficientVaultBalance,

    #[msg("Leaderboard not initialized")]
    LeaderboardNotInitialized,

    #[msg("Invalid vault address")]
    InvalidVaultAddress,

     #[msg("Insufficient Players")]
    InsufficientPlayers,

     #[msg("Invalid Question Index")]
    InvalidQuestionIndex,

    #[msg("Wrong Answer!!!!")]
    InvalidAnswer,

     #[msg("Already Answered")]
    AlreadyAnswered,

    #[msg("Winner not choosen")]
    WinnersNotSet,

    #[msg("Invalid Winner")]
    InvalidWinnerAccount,

     #[msg("Invalid Amount")]
    InvalidAmount,



}
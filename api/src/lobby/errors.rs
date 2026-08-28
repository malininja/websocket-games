use serde::Serialize;
use thiserror::Error;

#[derive(Error, Serialize, Clone, Debug)]
pub enum LobbyError {
    #[error("Game has already started")]
    AlreadyStarted,

    #[error("Player is in active game")]
    PlayerInActiveGame,

    #[error("Invalid action")]
    InvalidAction,

    #[error("Game doesn't exist")]
    GameDoesntExist,

    #[error("Invalid message")]
    InvalidMessage,

    #[error("Invalid user")]
    InvalidUser,

    #[error("Unauthorized")]
    Unauthorized,
}

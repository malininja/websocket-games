use serde::Serialize;
use thiserror::Error;

#[derive(Error, Serialize, Clone, Debug)]
pub enum TicTacToeError {
    #[error("Game is finished")]
    GameFinished,

    #[error("Field is occupied")]
    FieldOccupied,

    #[error("Field is out of bounds")]
    FieldOutOfBounds,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid move")]
    InvalidMove,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TicTacToeError {
    #[error("Game is finished")]
    GameFinished,

    #[error("Field is occupied")]
    FieldOccupied,

    #[error("Field is out of bounds")]
    FieldOutOfBounds,
}

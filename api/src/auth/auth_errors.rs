use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token creation error")]
    TokenCreationError,
}

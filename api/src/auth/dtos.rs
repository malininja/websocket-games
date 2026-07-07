use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginRequestDto {
    pub username: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponseDto {
    pub token: String,
}

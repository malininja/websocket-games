use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

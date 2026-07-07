use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::auth::{auth_errors::AuthError, claims::Claims};

pub async fn login(username: String, jwt_secret: String) -> Result<String, AuthError> {
    let claims = Claims {
        sub: username,
        exp: Utc::now().timestamp() as usize + 3600,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreationError)?;

    Ok(token)
}

pub async fn validate_token(token: String, secret: String) -> Option<Claims> {
    match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        Ok(token) => Some(token.claims),
        Err(_) => None,
    }
}

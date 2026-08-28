use axum::http::HeaderMap;
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

pub fn get_cookie_jwt(headers: HeaderMap, jwt_name: String) -> Option<String> {
    if let Some(cookie_header) = headers.get("cookie") {
        match cookie_header.to_str() {
            Ok(header_string) => {
                if let Some(jwt_token) = header_string
                    .split(";")
                    .find(|h| h.trim().starts_with(&format!("{}=", jwt_name)))
                {
                    let parts: Vec<&str> = jwt_token.trim().split("=").collect();

                    if parts.len() > 1 {
                        return Some(parts[1].to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading cookie header: {}", e);
            }
        }
    }

    None
}

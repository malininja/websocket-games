use axum::{
    Json,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};

use crate::{
    WebsiteState,
    auth::{
        auth_service,
        dtos::{LoginRequestDto, LoginResponseDto},
    },
};

pub async fn login(
    State(state): State<WebsiteState>,
    Json(body): Json<LoginRequestDto>,
) -> Result<impl IntoResponse, StatusCode> {
    match auth_service::login(body.username, state.jwt_secret).await {
        Ok(token) => {
            let cookie = format!("{}={}; Path=/; HttpOnly;", state.jwt_token_name, token);

            Ok((
                [(header::SET_COOKIE, cookie)],
                Json(LoginResponseDto { token }),
            ))
        }
        Err(e) => {
            eprintln!("auth_handler; login error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

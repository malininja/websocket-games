use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Router;

use crate::{config::Config, structs::WebsiteState};

pub mod auth;
pub mod chat;
pub mod config;
pub mod lobby;
pub mod structs;
pub mod tic_tac_toe;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let config = Config::from_env();

    let state = WebsiteState {
        jwt_secret: std::env::var(config.jwt_secret).unwrap_or("supertestsecret".to_string()),
        jwt_token_name: "websocket_games_jwt".to_string(),
        games: Arc::new(Mutex::new(Vec::new())),
    };

    // chat::main().await;

    let app = Router::new()
        .nest("/lobby", lobby::router::router(state.clone()))
        .nest("/tic-tac-toe", tic_tac_toe::router::router(state.clone()))
        .nest("/login", auth::auth_router::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.api_url).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

use axum::{Router, routing::get};
use serde::Serialize;
use tokio::sync::broadcast;

use crate::{
    Game, WebsiteState,
    lobby::{errors::LobbyError, handler::lobby_ws_handler},
};

#[derive(Clone)]
pub struct AppState {
    pub website_state: WebsiteState,
    pub tx: broadcast::Sender<ServerMessage>,
}

#[derive(Serialize, Clone)]
pub struct ServerMessage {
    pub games: Vec<Game>,
    pub requested_by: String,
    pub username: Option<String>,
    pub error: Option<LobbyError>,
}

pub fn router(state: WebsiteState) -> Router<WebsiteState> {
    let (tx, _) = broadcast::channel::<ServerMessage>(100);

    let app_state = AppState {
        website_state: state,
        tx,
    };

    Router::new()
        .route("/", get(lobby_ws_handler))
        .with_state(app_state)
}

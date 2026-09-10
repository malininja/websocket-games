use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};

use crate::{
    WebsiteState,
    tic_tac_toe::{errors::TicTacToeError, game::Letters, handler::tic_tac_toe_ws_handler},
};

#[derive(Debug, Deserialize)]
pub struct ClientMessage {
    pub position: (i32, i32),
}

#[derive(Debug, Serialize, Clone)]
pub struct ServerMessage {
    pub socket_username: Option<String>,
    #[serde(skip)]
    pub played_by: Option<String>,
    pub board: Option<Vec<Vec<Option<Letters>>>>,
    pub error: Option<TicTacToeError>,
    pub winner: Option<String>,
}

pub fn router(state: WebsiteState) -> Router<WebsiteState> {
    Router::new()
        .route("/", get(tic_tac_toe_ws_handler))
        .with_state(state)
}

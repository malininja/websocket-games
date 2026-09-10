use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::tic_tac_toe::{self, router::ServerMessage};

#[derive(Clone)]
pub struct WebsiteState {
    pub jwt_secret: String,
    pub jwt_token_name: String,
    pub games: Arc<Mutex<Vec<Game>>>,
}

#[derive(Clone)]
pub struct Game {
    pub id: Uuid,
    pub players: (Option<String>, Option<String>),
    pub board: Option<tic_tac_toe::game::Game>,
    pub tx: Option<broadcast::Sender<ServerMessage>>,
}

impl Game {
    pub fn to_dto(&self) -> GameDto {
        GameDto {
            id: self.id,
            players: self.players.clone(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct GameDto {
    pub id: Uuid,
    pub players: (Option<String>, Option<String>),
}

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, Utf8Bytes, WebSocket},
    },
    http::HeaderMap,
    response::Response,
};
use serde::Deserialize;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{
    WebsiteState,
    auth::auth_service::{get_cookie_jwt, validate_token},
    lobby::{
        errors::LobbyError,
        router::{AppState, ServerMessage},
    },
    structs::Game,
};

#[derive(Deserialize)]
struct ClientMessage {
    game_id: Option<Uuid>,
    action: GameAction,
}

#[derive(Deserialize, PartialEq)]
enum GameAction {
    Join,
    Remove,
}

pub async fn lobby_ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Response {
    let jwt_option = get_cookie_jwt(headers, state.website_state.jwt_token_name.clone());

    if let Some(token) = jwt_option
        && let Some(claims) = validate_token(token, state.website_state.jwt_secret.clone()).await
    {
        return ws.on_upgrade(|socket| handle_socket(socket, state, claims.sub));
    }

    eprintln!("Unauthorized");
    ws.on_upgrade(async |mut socket| {
        let message = ServerMessage {
            games: vec![],
            username: None,
            requested_by: "".to_string(),
            error: Some(LobbyError::Unauthorized),
        };

        let json = serde_json::to_string(&message).unwrap();
        let _ = socket.send(Message::Text(json.into())).await;
        let _ = socket.send(Message::Close(None)).await;

        while socket.recv().await.is_some() {}
    })
}

async fn handle_socket(mut socket: WebSocket, state: AppState, username: String) {
    println!("connection opened");

    let games = { state.website_state.games.lock().await.to_vec() };
    let message = ServerMessage {
        games: games.iter().map(|g| g.to_dto()).collect(),
        requested_by: username.clone(),
        username: Some(username.clone()),
        error: None,
    };
    let json = serde_json::to_string(&message).unwrap();
    let _ = socket.send(Message::Text(json.into())).await;

    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            message = socket.recv() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        let message = handle_client_text_message(text, state.website_state.clone(), username.clone()).await;
                        let _ = state.tx.send(message);
                    },
                    _ => break
                }
            }
            message = rx.recv() => {
                match message {
                    Ok(mut m) => {
                        if m.error.is_none() || m.requested_by == username.clone() {
                            m.username = Some(username.clone());
                            let json = serde_json::to_string(&m).unwrap();
                            println!("{}", json);
                            if socket.send(Message::Text(json.into())).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            else => break
        }
    }
}

async fn handle_client_text_message(
    text: Utf8Bytes,
    website_state: WebsiteState,
    username: String,
) -> ServerMessage {
    let mut games = website_state.games.lock().await;

    let error: Option<LobbyError> = match serde_json::from_str::<ClientMessage>(&text) {
        Ok(message) => handle_message(message, &mut games, username.clone()),
        Err(e) => {
            eprintln!("Invalid message: {}", e);
            Some(LobbyError::InvalidMessage)
        }
    };

    ServerMessage {
        games: games.iter().map(|g| g.to_dto()).collect(),
        requested_by: username,
        username: None,
        error,
    }
}

fn handle_message(
    message: ClientMessage,
    games: &mut Vec<Game>,
    username: String,
) -> Option<LobbyError> {
    match message.game_id {
        Some(game_id) => {
            let index_option = games.iter().position(|g| g.id == game_id);

            if let Some(index) = index_option {
                match message.action {
                    GameAction::Join => handle_join(games, index, username),
                    GameAction::Remove => handle_remove(games, index, &username),
                }
            } else {
                Some(LobbyError::GameDoesntExist)
            }
        }
        None => {
            if message.action == GameAction::Join {
                handle_create(games, username)
            } else {
                Some(LobbyError::InvalidAction)
            }
        }
    }
}

fn handle_join(games: &mut [Game], game_index: usize, username: String) -> Option<LobbyError> {
    if is_player_in_game(games, &username) {
        return Some(LobbyError::PlayerInActiveGame);
    }

    if games[game_index].players.1.is_some() {
        return Some(LobbyError::AlreadyStarted);
    }

    games[game_index].players.1 = Some(username);

    None
}

fn handle_create(games: &mut Vec<Game>, username: String) -> Option<LobbyError> {
    if is_player_in_game(games, &username) {
        return Some(LobbyError::PlayerInActiveGame);
    }

    games.push(Game {
        id: Uuid::new_v4(),
        players: (Some(username), None),
        board: None,
        tx: None,
    });

    None
}

fn handle_remove(games: &mut Vec<Game>, game_index: usize, username: &str) -> Option<LobbyError> {
    if games[game_index].players.0.as_deref() != Some(username) {
        return Some(LobbyError::InvalidUser);
    }

    if games[game_index].players.1.is_some() {
        return Some(LobbyError::AlreadyStarted);
    }

    let _ = games.remove(game_index);

    None
}

fn is_player_in_game(games: &[Game], username: &str) -> bool {
    games.iter().any(|g| {
        g.players.0.as_deref() == Some(username) || g.players.1.as_deref() == Some(username)
    })
}

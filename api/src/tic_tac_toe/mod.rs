pub mod errors;
pub mod game;

use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::HeaderMap,
    response::Response,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::{
    WebsiteState,
    auth::auth_service::{get_cookie_jwt, validate_token},
    tic_tac_toe::{errors::TicTacToeError, game::Game},
};

#[derive(Clone)]
struct AppState {
    website_state: WebsiteState,
    tx: broadcast::Sender<ServerMessage>,
    game: Arc<Mutex<Game>>,
    x_id: Arc<Mutex<Option<String>>>,
    o_id: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Deserialize)]
struct ClientMessage {
    position: (i32, i32),
}

#[derive(Debug, Serialize, Clone)]
struct ServerMessage {
    username: Option<String>,
    board: Option<Vec<Vec<Option<char>>>>,
    error: Option<TicTacToeError>,
    winner: Option<String>,
}

pub fn router(website_state: WebsiteState) -> Router<WebsiteState> {
    let (tx, _) = broadcast::channel::<ServerMessage>(100);
    let state = AppState {
        website_state,
        tx,
        game: Arc::new(Mutex::new(Game::new())),
        x_id: Arc::new(Mutex::new(None)),
        o_id: Arc::new(Mutex::new(None)),
    };

    Router::new().route("/", get(ws_handler)).with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Response {
    println!("jwt token: {}", state.website_state.jwt_secret);
    println!("headers: {:?}", headers);

    let jwt_option: Option<String> =
        get_cookie_jwt(headers, state.website_state.jwt_token_name.clone());

    if let Some(token) = jwt_option
        && let Some(claims) = validate_token(token, state.website_state.jwt_secret.clone()).await
    {
        return ws.on_upgrade(|socket| handle_socket(socket, state, claims.sub));
    }

    eprintln!("Unauthorized");
    ws.on_upgrade(async |mut socket| {
        let message = ServerMessage {
            username: None,
            board: None,
            error: Some(TicTacToeError::Unauthorized),
            winner: None,
        };

        let json = serde_json::to_string(&message).unwrap();
        let _ = socket.send(Message::Text(json.into())).await;
        let _ = socket.send(Message::Close(None)).await;

        while socket.recv().await.is_some() {}
    })
}

async fn handle_socket(mut socket: WebSocket, state: AppState, username: String) {
    println!("conection opened: {}", &username);

    let mut player_letter: char = 'X';

    {
        let mut x_id = state.x_id.lock().unwrap();
        if x_id.is_none() {
            *x_id = Some(username.clone());
        } else {
            let mut o_id = state.o_id.lock().unwrap();
            *o_id = Some(username.clone());
            player_letter = 'O';
        }
    }

    let mut rx = state.tx.subscribe();

    // send current state to the player when they join
    let _ = state.tx.send(ServerMessage {
        username: None,
        board: Some(state.game.lock().unwrap().board.to_vec()),
        error: None,
        winner: None,
    });

    loop {
        tokio::select! {
          message = socket.recv() => {
            match message {
                Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(msg) => {
                        let mut game = state.game.lock().unwrap();

                        if game.next_letter == player_letter {
                            let winner_result = game.make_move(msg.position.0 as usize, msg.position.1 as usize);

                            match winner_result {
                                Ok(winner_option) => {
                                    match winner_option {
                                        Some(_) => {
                                            let _ = state.tx.send(ServerMessage {
                                                username: None,
                                                board: Some(game.board.to_vec()),
                                                error: None,
                                                winner: Some(username.clone()),
                                            });
                                        },
                                        None => {
                                            let _ = state.tx.send(ServerMessage {
                                                username: None,
                                                board: Some(game.board.to_vec()),
                                                error: None,
                                                winner: None,
                                            });
                                        }
                                    }
                                },
                                Err(error) => {
                                    let _ = state.tx.send(ServerMessage {
                                        username: None,
                                        board: Some(game.board.to_vec()),
                                        error: Some(error),
                                        winner: None,
                                    });
                                }
                            }
                        } else {
                            let _ = state.tx.send(ServerMessage {
                                username: None,
                                board: None,
                                error: Some(TicTacToeError::InvalidMove),
                                winner: None,
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Invalid message: {}", e);
                    }
                },
                _ => break
            }
          }
          message = rx.recv() => {
            match message {
              Ok (mut m) => {
                m.username = Some(username.clone());
                let json = serde_json::to_string(&m).unwrap();
                if socket.send(Message::Text(json.into())).await.is_err() {
                  break;
                }
              }
              Err(broadcast::error::RecvError::Lagged(_)) => continue,
              Err(broadcast::error::RecvError::Closed) => break,
            }
          }
          else => break
        }
    }

    eprintln!("conection closed: {}", username);
}

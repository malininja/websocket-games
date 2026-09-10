use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::HeaderMap,
    response::Response,
};
use tokio::sync::broadcast;

use crate::{
    WebsiteState,
    auth::auth_service::{get_cookie_jwt, validate_token},
    tic_tac_toe::{
        self,
        errors::TicTacToeError,
        game::{Letters, Winner},
        router::{ClientMessage, ServerMessage},
    },
};

pub async fn tic_tac_toe_ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<WebsiteState>,
) -> Response {
    let jwt_option: Option<String> = get_cookie_jwt(headers, state.jwt_token_name.clone());

    if let Some(token) = jwt_option
        && let Some(claims) = validate_token(token, state.jwt_secret.clone()).await
    {
        return match init_game(&state, claims.sub.clone()).await {
            Ok(_) => ws.on_upgrade(|socket| handle_socket(socket, state, claims.sub)),
            Err(error) => {
                return ws.on_upgrade(async |mut socket| {
                    let message = ServerMessage {
                        socket_username: None,
                        played_by: None,
                        board: None,
                        error: Some(error),
                        winner: None,
                    };

                    let _ = send_msg_over_socket(&mut socket, message).await;
                    let _ = socket.send(Message::Close(None)).await;
                    while socket.recv().await.is_some() {}
                });
            }
        };
    }

    eprintln!("Unauthorized");
    ws.on_upgrade(async |mut socket| {
        let message = ServerMessage {
            socket_username: None,
            played_by: None,
            board: None,
            error: Some(TicTacToeError::Unauthorized),
            winner: None,
        };

        let _ = send_msg_over_socket(&mut socket, message).await;
        let _ = socket.send(Message::Close(None)).await;
        while socket.recv().await.is_some() {}
    })
}

async fn init_game(state: &WebsiteState, username: String) -> Result<(), TicTacToeError> {
    let mut games = state.games.lock().await;
    let game_option = games
        .iter_mut()
        .find(|g| g.players.0 == Some(username.clone()) || g.players.1 == Some(username.clone()));

    if let Some(game) = game_option {
        if game.board.is_none() {
            game.board = Some(tic_tac_toe::game::Game::new());
        }

        if game.tx.is_none() {
            let (tx, _) = broadcast::channel::<ServerMessage>(100);
            game.tx = Some(tx);
        }

        return Ok(());
    }

    Err(TicTacToeError::UserNotInGame)
}

async fn send_msg_over_socket(socket: &mut WebSocket, message: ServerMessage) {
    let json = serde_json::to_string(&message).unwrap();
    let _ = socket.send(Message::Text(json.into())).await;
}

async fn handle_socket(mut socket: WebSocket, state: WebsiteState, username: String) {
    println!("conection opened: {}", &username);

    let mut rx = {
        let mut games = state.games.lock().await;
        let game_option = games.iter_mut().find(|g| {
            g.players.0 == Some(username.clone()) || g.players.1 == Some(username.clone())
        });

        if let Some(game) = game_option {
            let rx = game.tx.as_mut().unwrap().subscribe();

            let msg = ServerMessage {
                socket_username: None,
                played_by: None,
                board: Some(game.board.as_ref().unwrap().cells.to_vec()),
                error: None,
                winner: None,
            };

            send_msg_over_socket(&mut socket, msg).await;

            rx
        } else {
            let msg = ServerMessage {
                socket_username: None,
                played_by: None,
                board: None,
                error: Some(TicTacToeError::GameDoesntExist),
                winner: None,
            };

            send_msg_over_socket(&mut socket, msg).await;

            return;
        }
    };

    loop {
        tokio::select! {
          message = socket.recv() => {
            match message {
                Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(msg) => {
                        println!("message: {}", username.clone());
                        let result = handle_client_message(msg, &state, username.clone()).await;

                        if result.is_err() {
                            break;
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
                if m.error.is_none() || m.played_by == Some(username.clone()) {
                    m.socket_username = Some(username.clone());
                    let json = serde_json::to_string(&m).unwrap();
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

    eprintln!("conection closed: {}", username);
}

async fn handle_client_message(
    message: ClientMessage,
    state: &WebsiteState,
    username: String,
) -> Result<(), ()> {
    let mut games = state.games.lock().await;
    let game_option = games
        .iter_mut()
        .find(|g| g.players.0 == Some(username.clone()) || g.players.1 == Some(username.clone()));

    if let Some(game) = game_option {
        let player_letter = if game.players.0 == Some(username.clone()) {
            Letters::X
        } else {
            Letters::O
        };

        let board = game.board.as_mut().unwrap();

        if board.next_letter == player_letter {
            let winner_result =
                board.make_move(message.position.0 as usize, message.position.1 as usize);

            match winner_result {
                Ok(winner) => {
                    let winner_response = match winner {
                        Winner::Option(_) => {
                            if winner == Winner::Option(None) {
                                Some("".to_string())
                            } else {
                                Some(username.clone())
                            }
                        }
                        Winner::None => None,
                    };

                    let _ = game.tx.as_ref().unwrap().send(ServerMessage {
                        socket_username: None,
                        played_by: Some(username),
                        board: Some(board.cells.to_vec()),
                        error: None,
                        winner: winner_response,
                    });

                    if winner != Winner::None {
                        let game_id = game.id.clone();
                        let index_option = games.iter_mut().position(|g| g.id == game_id);
                        games.remove(index_option.unwrap());
                    }
                }
                Err(error) => {
                    let _ = game.tx.as_ref().unwrap().send(ServerMessage {
                        socket_username: None,
                        played_by: Some(username),
                        board: Some(board.cells.to_vec()),
                        error: Some(error),
                        winner: None,
                    });
                }
            }
        } else {
            let _ = game.tx.as_ref().unwrap().send(ServerMessage {
                socket_username: None,
                played_by: Some(username),
                board: None,
                error: Some(TicTacToeError::InvalidMove),
                winner: None,
            });
        }

        Ok(())
    } else {
        Err(())
    }
}

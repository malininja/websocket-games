use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Clone)]
struct AppState {
    rooms: Arc<Mutex<HashMap<String, broadcast::Sender<ServerMsg>>>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMsg {
    Join { room: String },
    Chat { text: String },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMsg {
    Chat { text: String },
    System { text: String },
    Error { text: String },
}

pub async fn main() {
    let state = AppState {
        rooms: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let room = match socket.recv().await {
        Some(Ok(Message::Text(t))) => match serde_json::from_str::<ClientMsg>(&t) {
            Ok(ClientMsg::Join { room }) => room,
            _ => {
                let msg = "Websocket first call was not 'Join'";
                eprintln!("{}", msg);

                let server_msg = ServerMsg::Error {
                    text: String::from(msg),
                };
                let json = serde_json::to_string(&server_msg).unwrap();
                let _ = socket.send(json.into()).await;

                return;
            }
        },
        _ => return,
    };

    let tx = {
        let mut rooms = state.rooms.lock().unwrap();
        rooms
            .entry(room.clone())
            .or_insert_with(|| broadcast::channel::<ServerMsg>(100).0)
            .clone()
    };

    let mut rx = tx.subscribe();

    let _ = tx.send(ServerMsg::System {
        text: String::from("connect"),
    });

    loop {
        tokio::select! {
            Some(Ok(Message::Text(text))) = socket.recv() => {
                match serde_json::from_str::<ClientMsg>(&text) {
                    Ok(ClientMsg::Chat { text }) => {
                        let _ = tx.send(ServerMsg::Chat { text });
                    }
                    Ok(ClientMsg::Join { .. }) => {
                        eprintln!("Can't join twice. Current room: {}", room);
                    }
                    Err(e) => {
                        eprintln!("Invalid message: {}", e);

                    }
                }
            }
            message = rx.recv() => {
                match message {
                    Ok(m) => {
                        let json = serde_json::to_string(&m).unwrap();
                        if socket.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            else => break,
        }
    }

    let _ = tx.send(ServerMsg::System {
        text: String::from("disconnect"),
    });

    drop(rx);
    if tx.receiver_count() == 0 {
        state.rooms.lock().unwrap().remove(&room);
        println!("Room removed: {}", room);
    }
}

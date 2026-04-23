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
    tx: broadcast::Sender<ServerMsg>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMsg {
    Chat { text: String },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMsg {
    Chat { text: String },
    System { text: String },
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<ServerMsg>(100);
    let state = AppState { tx };

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
    let mut rx = state.tx.subscribe();

    let _ = state.tx.send(ServerMsg::System {
        text: String::from("connect"),
    });

    loop {
        tokio::select! {
            Some(Ok(Message::Text(text))) = socket.recv() => {
                match serde_json::from_str::<ClientMsg>(&text) {
                    Ok(ClientMsg::Chat { text }) => {
                        let _ = state.tx.send(ServerMsg::Chat { text });
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

    let _ = state.tx.send(ServerMsg::System {
        text: String::from("disconnect"),
    });
}

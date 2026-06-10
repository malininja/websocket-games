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
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<ServerMessage>,
    board: Arc<Mutex<Vec<Vec<Option<char>>>>>,
    x_id: Arc<Mutex<Option<Uuid>>>,
    o_id: Arc<Mutex<Option<Uuid>>>,
}

#[derive(Debug, Deserialize)]
struct ClientMessage {
    position: (i32, i32),
}

#[derive(Debug, Serialize, Clone)]
struct ServerMessage {
    board: Vec<Vec<Option<char>>>,
}

pub async fn main() {
    let (tx, _) = broadcast::channel::<ServerMessage>(100);
    let state = AppState {
        tx,
        board: Arc::new(Mutex::new(vec![
            vec![None, None, None],
            vec![None, None, None],
            vec![None, None, None],
        ])),
        x_id: Arc::new(Mutex::new(None)),
        o_id: Arc::new(Mutex::new(None)),
    };

    let app = Router::new()
        .route("/tic-tac-toe", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Response {
    println!("headers: {:?}", headers);
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let id = Uuid::new_v4();
    println!("conection opened: {}", id);

    {
        let mut x_id = state.x_id.lock().unwrap();
        if x_id.is_none() {
            *x_id = Some(id);
        } else {
            let mut o_id = state.o_id.lock().unwrap();
            *o_id = Some(id);
            std::mem::drop(o_id);
        }
    }

    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
          message = socket.recv() => {
            match message {
                Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(msg) => {
                        let x_id = *state.x_id.lock().unwrap();

                        let letter = if Some(id) == x_id { 'X' } else { 'O' };

                        let mut board = state.board.lock().unwrap();
                        board[msg.position.0 as usize][msg.position.1 as usize] = Some(letter);
                        let _ = state.tx.send(ServerMessage {
                            board: board.to_vec(),
                        });
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
              Ok (m) => {
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

    eprintln!("conection closed: {}", id);
}

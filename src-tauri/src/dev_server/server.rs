use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use pzql_ipc::WsCommandEntry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::net::TcpListener;

const PORT: u16 = 1421;

#[derive(Deserialize)]
struct WsRequest {
    id: u64,
    cmd: String,
    args: Value,
}

#[derive(Serialize)]
struct WsResponse {
    id: u64,
    result: Option<Value>,
    error: Option<Value>,
}

pub async fn run() {
    let app = Router::new().route("/ws", get(ws_handler));
    let listener = TcpListener::bind(("127.0.0.1", PORT)).await.unwrap();
    println!("dev-server listening on ws://127.0.0.1:{PORT}");
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let handlers: HashMap<&str, _> = inventory::iter::<WsCommandEntry>()
        .map(|e| (e.name, e.handler))
        .collect();

    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        let req: WsRequest = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("bad ws message: {e}");
                continue;
            }
        };

        let response = match handlers.get(req.cmd.as_str()) {
            Some(handler) => match handler(req.args).await {
                Ok(result) => WsResponse {
                    id: req.id,
                    result: Some(result),
                    error: None,
                },
                Err(error) => WsResponse {
                    id: req.id,
                    result: None,
                    error: Some(error),
                },
            },
            None => WsResponse {
                id: req.id,
                result: None,
                error: Some(serde_json::json!({
                    "message": format!("unknown command: {}", req.cmd)
                })),
            },
        };

        let _ = sender
            .send(Message::Text(
                serde_json::to_string(&response).unwrap().into(),
            ))
            .await;
    }
}

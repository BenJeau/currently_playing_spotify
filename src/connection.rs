use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::{headers::UserAgent, typed_header::TypedHeader};
use std::time::Instant;
use tokio::{select, sync::watch::Receiver, time::interval};
use tracing::{info, warn};

use crate::config;

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    State(rx): State<Receiver<String>>,
) -> impl IntoResponse {
    info!("Establishing connection: {user_agent:?}");

    ws.on_upgrade(|socket| async {
        let _ = handle_socket(socket, user_agent, rx).await;
    })
}

async fn handle_socket(
    mut ws_stream: WebSocket,
    user_agent: Option<TypedHeader<UserAgent>>,
    mut rx: Receiver<String>,
) -> Result<(), axum::Error> {
    info!("Connection upgraded to WS connection: {user_agent:?}");

    let mut heartbeat = Instant::now();
    let mut heartbeat_interval = interval(config::HEARTBEAT_INTERVAL);
    let mut close_stream = false;

    while !close_stream {
        select! {
            Some(Ok(msg)) = ws_stream.recv() => {
                match msg {
                    Message::Ping(_) => {
                        heartbeat = Instant::now();
                        ws_stream.send(Message::Pong(vec![])).await?;
                    }
                    Message::Pong(_) => {
                        heartbeat = Instant::now();
                    }
                    Message::Close(_) => {
                        close_stream = true;
                    }
                    Message::Text(_) | Message::Binary(_) => {
                        warn!("Text and binary messages are not supported");
                    }
                }
            }
            Ok(()) = rx.changed() => {
                let data = rx.borrow().clone();
                ws_stream.send(Message::Text(data)).await?;
            }
            _ = heartbeat_interval.tick() => {
                if Instant::now().duration_since(heartbeat) > config::CLIENT_TIMEOUT {
                    info!("Websocket Client heartbeat failed, disconnecting!");
                    close_stream = true;
                } else {
                    ws_stream.send(Message::Ping(vec![])).await?;
                }
            }
        }
    }

    Ok(())
}

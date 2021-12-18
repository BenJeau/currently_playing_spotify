use futures_util::{SinkExt, StreamExt};
use log::{info, warn};
use std::{net::SocketAddr, time::Instant};
use tokio::{net::TcpStream, select, sync::watch::Receiver, time::interval};
use tokio_tungstenite::tungstenite::{self, Message};

use crate::config;

pub async fn accept_connection(
    stream: TcpStream,
    addr: SocketAddr,
    mut rx: Receiver<String>,
) -> tungstenite::Result<()> {
    info!("Establishing connection: {}", addr);

    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    let mut heartbeat = Instant::now();
    let mut heartbeat_interval = interval(config::HEARTBEAT_INTERVAL);
    let mut close_stream = false;

    let data = rx.borrow().clone();
    ws_stream.send(Message::Text(data)).await?;

    while !close_stream {
        select! {
            Some(Ok(msg)) = ws_stream.next() => {
                match msg {
                    Message::Text(_) => {
                        warn!("Text messages are not supported");
                    }
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
                    Message::Binary(_) => {
                        warn!("Binary messages are not supported");
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

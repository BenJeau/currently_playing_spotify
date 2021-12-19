use clap::Parser;
use env_logger::Env;
use log::info;
use tokio::{net::TcpListener, sync::watch};

use crate::{
    auth::{query_periodically_spotify_api, SpotifyAuth},
    connection::accept_connection,
};

mod auth;
mod config;
mod connection;
mod song;
mod utils;

/// Simple Rust websocket proxy server using Actix to know what track the specified user is currently listening
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Benoît J. <benoit@jeaurond.dev>")]
struct Opts {
    /// Maximum interval in seconds which the Spotify API will be called
    #[clap(short, long, env = "INTERVAL_QUERY_SECS", default_value = "2")]
    interval: u64,

    /// Authentication code from the Spotify user taken from the Authentication authentication flow
    #[clap(long, env = "SPOTIFY_AUTH_CODE")]
    auth_code: String,

    /// Spotify application client id
    #[clap(long, env = "SPOTIFY_CLIENT_ID")]
    client_id: String,

    /// Spotify application client secret
    #[clap(long, env = "SPOTIFY_CLIENT_SECRET")]
    client_secret: String,

    /// Websocket server port
    #[clap(short, long, env = "WEBSOCKET_PORT", default_value = "8080")]
    port: String,

    /// Websocket server address
    #[clap(short, long, env = "WEBSOCKET_ADDRESS", default_value = "0.0.0.0")]
    address: String,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let Opts {
        interval,
        auth_code,
        client_id,
        client_secret,
        port,
        address,
    } = Opts::parse();

    let (tx, rx) = watch::channel("".to_string());
    let spotify_auth = SpotifyAuth::new(auth_code, client_id, client_secret).await;

    tokio::task::spawn(query_periodically_spotify_api(interval, spotify_auth, tx));
    info!("Spawned background task querying Spotify's API");

    let websocket_addr = format!("{}:{}", address, port);
    let listener = TcpListener::bind(&websocket_addr)
        .await
        .expect("Failed to bind to address");
    info!("Websocket listening on: {}", websocket_addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, addr, rx.clone()));
    }
}

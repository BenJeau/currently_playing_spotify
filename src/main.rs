use axum::{http::Method, routing::get, Router};
use clap::Parser;
use tokio::sync::watch;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{
    auth::{query_periodically_spotify_api, SpotifyAuth},
    connection::handler,
    opts::Opts,
};

mod auth;
mod config;
mod connection;
mod error;
mod opts;
mod song;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Opts {
        interval,
        username,
        password,
        port,
        address,
        cors_origin,
        compact,
    } = Opts::parse();

    let (tx, rx) = watch::channel("".to_string());
    let spotify_auth = SpotifyAuth::new(&username, &password, compact).await;

    tokio::task::spawn(query_periodically_spotify_api(interval, spotify_auth, tx));
    info!("Spawned background task querying Spotify's API");

    let mut cors = CorsLayer::new().allow_methods(vec![Method::GET]);

    if let Some(cors_origin) = cors_origin {
        cors = cors.allow_origin([cors_origin.parse().unwrap()]);
    } else {
        cors = cors.allow_origin(Any);
    }

    let app = Router::new()
        .route("/ws", get(handler))
        .layer(cors)
        .with_state(rx);

    let addr = format!("{address}:{port}");
    info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

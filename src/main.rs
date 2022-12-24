use axum::{routing::get, Extension, Router};
use clap::Parser;
use http::Method;
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
mod opts;
mod song;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Opts {
        interval,
        auth_code,
        client_id,
        client_secret,
        port,
        address,
        cors_origin,
    } = Opts::parse();

    let (tx, rx) = watch::channel("".to_string());
    let spotify_auth = SpotifyAuth::new(auth_code, client_id, client_secret).await;

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
        .layer(Extension(rx));

    let addr = format!("{address}:{port}").parse().unwrap();
    info!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

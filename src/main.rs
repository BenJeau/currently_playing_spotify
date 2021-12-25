use axum::{routing::get, AddExtensionLayer, Router};
use clap::StructOpt;
use tokio::sync::watch;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
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
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "currently-playing-spotify=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();

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

    let app = Router::new()
        .route("/ws", get(handler))
        .layer(AddExtensionLayer::new(rx))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let addr = format!("{}:{}", address, port).parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

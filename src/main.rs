use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use env_logger::Env;
use std::sync::Mutex;

use crate::auth::SpotifyAuth;
use crate::song::Song;

mod auth;
mod routes;
mod song;
mod utils;

/// Simple Rust HTTP proxy server using Actix to know what track the specified user is currently listening
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Benoît J. <benoit@jeaurond.dev>")]
struct Opts {
    /// Maximum interval in seconds which the Spotify API will be called. Requests made during the interval will return the cached result.
    #[clap(short, long, env = "INTERVAL_QUERY_SECS", default_value = "1")]
    interval: i64,

    /// Authentication code from the Spotify user taken from the Authentication authentication flow
    #[clap(short, long, env = "SPOTIFY_AUTH_CODE")]
    auth_code: String,

    /// Spotify application client id
    #[clap(alias = "id", long, env = "SPOTIFY_CLIENT_ID")]
    client_id: String,

    /// Spotify application client secret
    #[clap(short = 's', long, env = "SPOTIFY_CLIENT_SECRET")]
    client_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Opts {
        interval,
        auth_code,
        client_id,
        client_secret,
    } = Opts::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let currently_playing: web::Data<Mutex<Option<Song>>> = web::Data::new(Mutex::new(None));
    let spotify_auth = web::Data::new(Mutex::new(
        SpotifyAuth::new(auth_code, interval, client_id, client_secret).await,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(currently_playing.clone())
            .app_data(spotify_auth.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(routes::index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

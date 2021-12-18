use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::Mutex;

use crate::auth::SpotifyAuth;
use crate::song::Song;

mod auth;
mod routes;
mod song;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        let currently_playing: web::Data<Mutex<Option<Song>>> = web::Data::new(Mutex::new(None));
        let spotify_auth = web::Data::new(Mutex::new(SpotifyAuth::new()));

        App::new()
            .app_data(currently_playing)
            .app_data(spotify_auth)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(routes::index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

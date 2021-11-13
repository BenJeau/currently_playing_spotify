use actix_web::{get, web, Responder};
use log::info;
use std::sync::Mutex;

use crate::{auth::SpotifyAuth, song::Song};

#[get("/spotify/currently-playing")]
pub async fn index(
    currently_playing: web::Data<Mutex<Option<Song>>>,
    spotify_auth: web::Data<Mutex<SpotifyAuth>>,
) -> impl Responder {
    let mut currently_playing = currently_playing.lock().unwrap();
    let mut spotify_auth = spotify_auth.lock().unwrap();

    let song = match currently_playing.as_ref() {
        Some(song) if song.is_valid() => song.clone(),
        _ => {
            let song = spotify_auth.query_currently_playing().await.unwrap();
            *currently_playing = Some(song.clone());
            song
        }
    };

    web::Json(song)
}

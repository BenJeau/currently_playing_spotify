use axum::body::Bytes;
use http::{
    header::{ACCEPT, AUTHORIZATION},
    Request,
};
use librespot::{
    core::{config::SessionConfig, session::Session},
    discovery::Credentials,
};
use std::time::Duration;
use tokio::{sync::watch::Sender, time::interval};
use tracing::info;

use crate::{
    error::Result,
    song::{Song, SongContent},
};

#[derive(Clone)]
pub struct SpotifyAuth {
    session: Session,
    compact: bool,
}

impl SpotifyAuth {
    pub async fn new(username: &str, password: &str, compact: bool) -> Self {
        let session_config = SessionConfig::default();
        let session = Session::new(session_config, None);

        let credentials = Credentials::with_password(username, password);
        session
            .connect(credentials, false)
            .await
            .expect("Unable to connect with provided credentials");

        Self { session, compact }
    }

    async fn query_currently_playing(&self) -> Result<Bytes> {
        info!("Querying Spotify currently playing track API");

        let token = self
            .session
            .token_provider()
            .get_token("user-read-currently-playing")
            .await?;

        let request = Request::get("https://api.spotify.com/v1/me/player/currently-playing")
            .header(AUTHORIZATION, format!("Bearer {}", token.access_token))
            .header(ACCEPT, "application/json")
            .body(Default::default())?;

        let response = self.session.http_client().request_body(request).await?;

        Ok(response)
    }
}

pub async fn query_periodically_spotify_api(
    interval_time: u64,
    spotify_auth: SpotifyAuth,
    tx: Sender<String>,
) {
    let mut query_interval = interval(Duration::from_secs(interval_time));
    let mut previous_response: Option<SongContent> = None;

    loop {
        let song = spotify_auth.query_currently_playing().await.unwrap();

        let song_content = serde_json::from_slice::<SongContent>(&song).ok();

        if song_content != previous_response {
            let data = if spotify_auth.compact {
                song_content
                    .clone()
                    .map(|s| serde_json::to_value(s).unwrap())
            } else {
                Some(serde_json::from_slice(&song).unwrap())
            };

            let song = Song::new(data).unwrap();

            let _ = tx.send(serde_json::to_string(&song).unwrap());
            previous_response = song_content;
        }

        query_interval.tick().await;
    }
}

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::time::Duration;
use tokio::{sync::watch::Sender, time::interval};
use tracing::{error, info, warn};

use crate::{song::Song, utils::has_time_passed};

#[derive(Clone)]
pub struct SpotifyAuth {
    auth_code: String,
    expires_in: i64,
    fetched: DateTime<Utc>,
    refresh_token: Option<String>,
    access_token: Option<String>,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct SpotifyAuthCodeResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
}

#[derive(Deserialize)]
struct SpotifyAuthResponse {
    access_token: String,
    expires_in: i64,
}

impl SpotifyAuth {
    pub async fn new(auth_code: String, client_id: String, client_secret: String) -> SpotifyAuth {
        let mut auth = SpotifyAuth {
            auth_code,
            expires_in: 0,
            fetched: Utc::now(),
            access_token: None,
            refresh_token: None,
            client_id,
            client_secret,
        };

        auth.get_auth_tokens().await;

        auth
    }

    fn should_get_new_access_token(&self) -> bool {
        has_time_passed(self.fetched, self.expires_in)
    }

    async fn get_auth_tokens(&mut self) {
        info!("Querying Spotify auth tokens API");

        let response = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(self.client_id.clone(), Some(self.client_secret.clone()))
            .form(&[
                ("redirect_uri", "http://localhost:8888/callback"),
                ("grant_type", "authorization_code"),
                ("code", &self.auth_code),
            ])
            .send()
            .await;

        match response {
            Ok(data) => {
                let data = match data.json::<SpotifyAuthCodeResponse>().await {
                    Ok(auth) => auth,
                    Err(err) => {
                        panic!("Invalid authorization code: {err:?}");
                    }
                };

                self.access_token = Some(data.access_token);
                self.refresh_token = Some(data.refresh_token);
                self.expires_in = data.expires_in;
                self.fetched = Utc::now();
            }
            Err(err) => {
                error!("Error querying Spotify auth tokens API: {err:?}");
            }
        };
    }

    async fn get_new_access_token(&mut self) {
        let refresh_token = match self.refresh_token.clone() {
            Some(token) => token,
            None => {
                warn!("Not querying Spotify access token auth API, no refresh token saved");
                return;
            }
        };

        info!("Querying Spotify access token auth API");

        let response = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(self.client_id.clone(), Some(self.client_secret.clone()))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &refresh_token),
            ])
            .send()
            .await;

        match response {
            Ok(data) => {
                let body = data.text().await.unwrap();

                let data = match serde_json::from_str::<SpotifyAuthResponse>(&body) {
                    Ok(auth) => auth,
                    Err(err) => {
                        panic!("Error parsing body. Error: {err:?}. Body: {body:?}");
                    }
                };

                self.access_token = Some(data.access_token);
                self.expires_in = data.expires_in;
                self.fetched = Utc::now();
            }
            Err(err) => {
                error!("Error querying Spotify access token auth API: {err:?}");
            }
        };
    }

    async fn currently_playing_request(&self) -> Result<Song, reqwest::Error> {
        info!("Querying Spotify currently playing track API");

        let access_token = match self.access_token.clone() {
            Some(token) => token,
            None => {
                error!("Access token does not exist");
                return Ok(Song::new(None));
            }
        };

        let response = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        match response {
            Ok(data) => match serde_json::from_str(&data.text().await.unwrap()) {
                Ok(data) => {
                    info!("User is currently playing music");
                    Ok(Song::new(Some(data)))
                }
                _ => {
                    info!("User NOT currently playing music");
                    Ok(Song::new(None))
                }
            },
            Err(err) => {
                error!("Error querying Spotify currently playing track API: {err:?}");
                Err(err)
            }
        }
    }

    pub async fn query_currently_playing(&mut self) -> Option<Song> {
        if self.should_get_new_access_token() {
            self.get_new_access_token().await;
        }

        match self.currently_playing_request().await {
            Ok(song) => Some(song),
            _ => {
                self.get_new_access_token().await;
                match self.currently_playing_request().await {
                    Ok(song) => Some(song),
                    _ => None,
                }
            }
        }
    }
}

pub async fn query_periodically_spotify_api(
    interval_time: u64,
    mut spotify_auth: SpotifyAuth,
    tx: Sender<String>,
) {
    let mut query_interval = interval(Duration::from_secs(interval_time));

    loop {
        let song = spotify_auth.query_currently_playing().await;

        let _ = tx.send(serde_json::to_string(&song).unwrap());
        query_interval.tick().await;
    }
}

use awc::{error::SendRequestError, Client};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::env;

use crate::song::Song;

pub struct SpotifyAuth {
    client: Client,
    expires_in: i64,
    fetched: DateTime<Utc>,
    refresh_token: String,
    access_token: String,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct SpotifyAuthResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
}

impl SpotifyAuth {
    pub fn new() -> SpotifyAuth {
        SpotifyAuth {
            client: Client::new(),
            expires_in: 0,
            fetched: Utc::now(),
            access_token: env::var("SPOTIFY_ACCESS_TOKEN").unwrap(),
            refresh_token: env::var("SPOTIFY_REFRESH_TOKEN").unwrap(),
            client_id: env::var("SPOTIFY_CLIENT_ID").unwrap(),
            client_secret: env::var("SPOTIFY_CLIENT_SECRET").unwrap(),
        }
    }

    fn should_get_new_access_token(&self) -> bool {
        (self.fetched + Duration::seconds(self.expires_in)) > Utc::now()
    }

    async fn get_new_access_token(&mut self) {
        let response = self
            .client
            .get("https://accounts.spotify.com/api/token")
            .basic_auth(self.client_id.clone(), self.client_secret.clone())
            .send_form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", self.refresh_token.as_str()),
            ])
            .await;

        match response {
            Ok(mut data) => {
                let data = data.json::<SpotifyAuthResponse>().await.unwrap();

                self.access_token = data.access_token;
                self.refresh_token = data.refresh_token;
                self.expires_in = data.expires_in;
                self.fetched = Utc::now();
            }
            Err(error) => {
                dbg!(error);
            }
        };
    }

    async fn currently_playing_request(&self) -> Result<Song, SendRequestError> {
        let response = self
            .client
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .insert_header((
                "Authorization",
                format!("Bearer {}", self.access_token.clone()),
            ))
            .send()
            .await;

        match response {
            Ok(mut data) => Ok(Song::new(
                serde_json::from_slice(&data.body().await.unwrap()).unwrap(),
            )),
            Err(error) => {
                dbg!(&error);
                Err(error)
            }
        }
    }

    pub async fn query_currently_playing(&mut self) -> Option<Song> {
        if self.should_get_new_access_token() {
            self.get_new_access_token().await;
        }

        match self.currently_playing_request().await {
            Ok(song) => Some(song),
            Err(_) => {
                // TODO: Maybe the error is because we need a new access token (although I don't think this would trigger this error - maybe need to do some error checking in the response)
                self.get_new_access_token().await;
                match self.currently_playing_request().await {
                    Ok(song) => Some(song),
                    Err(_) => None,
                }
            }
        }
    }
}

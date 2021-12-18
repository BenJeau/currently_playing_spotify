use awc::{error::SendRequestError, Client};
use chrono::{DateTime, Utc};
use log::{error, info, warn};
use serde::Deserialize;

use crate::{song::Song, utils::has_time_passed};

#[derive(Clone)]
pub struct SpotifyAuth {
    auth_code: String,
    expires_in: i64,
    pub interval: i64,
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
    pub async fn new(
        auth_code: String,
        interval: i64,
        client_id: String,
        client_secret: String,
    ) -> SpotifyAuth {
        let mut auth = SpotifyAuth {
            auth_code,
            expires_in: 0,
            interval,
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

        let response = Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(self.client_id.clone(), self.client_secret.clone())
            .send_form(&[
                ("redirect_uri", "http://localhost:8888/callback"),
                ("grant_type", "authorization_code"),
                ("code", &self.auth_code),
            ])
            .await;

        match response {
            Ok(mut data) => {
                let data = match data.json::<SpotifyAuthCodeResponse>().await {
                    Ok(auth) => auth,
                    Err(err) => {
                        error!(
                            "Invalid authorization code, Spotify API status {:?} and response: {:?}",
                            data.status(),
                            data.body().await,
                        );
                        panic!("{:?}", err);
                    }
                };

                self.access_token = Some(data.access_token);
                self.refresh_token = Some(data.refresh_token);
                self.expires_in = data.expires_in;
                self.fetched = Utc::now();
            }
            Err(error) => {
                error!("Error querying Spotify auth tokens API: {:?}", error);
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

        let response = Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(self.client_id.clone(), self.client_secret.clone())
            .send_form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &refresh_token),
            ])
            .await;

        match response {
            Ok(mut data) => {
                let data = data.json::<SpotifyAuthResponse>().await.unwrap();

                self.access_token = Some(data.access_token);
                self.expires_in = data.expires_in;
                self.fetched = Utc::now();
            }
            Err(error) => {
                error!("Error querying Spotify access token auth API: {:?}", error);
            }
        };
    }

    async fn currently_playing_request(&self) -> Result<Song, SendRequestError> {
        info!("Querying Spotify currently playing track API");

        let access_token = match self.access_token.clone() {
            Some(token) => token,
            None => {
                error!("Access token does not exist");
                return Ok(Song::new(None));
            }
        };

        let response = Client::new()
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .insert_header(("Authorization", format!("Bearer {}", access_token)))
            .send()
            .await;

        match response {
            Ok(mut data) => match serde_json::from_slice(&data.body().await.unwrap()) {
                Ok(data) => {
                    info!("User is currently playing music");
                    Ok(Song::new(Some(data)))
                }
                _ => {
                    info!("User NOT currently playing music");
                    Ok(Song::new(None))
                }
            },
            Err(error) => {
                error!(
                    "Error querying Spotify currently playing track API: {:?}",
                    error
                );
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

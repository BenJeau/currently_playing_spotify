use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, PartialEq, Eq)]
pub struct Song {
    data: Option<SongContent>,
    fetched: DateTime<Utc>,
}

impl Song {
    pub fn new(data: Option<SongContent>) -> Song {
        Song {
            data,
            fetched: Utc::now(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SongContent {
    pub currently_playing_type: String,
    pub is_playing: bool,
    #[serde(flatten)]
    pub item: Item,
    pub progress_ms: i64,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub duration_ms: i64,
    pub name: String,
    #[serde(flatten)]
    pub external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Album {
    pub images: Vec<Image>,
    pub name: String,
    #[serde(flatten)]
    pub external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Artist {
    pub name: String,
    #[serde(flatten)]
    pub external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ExternalUrls {
    pub spotify: String,
}

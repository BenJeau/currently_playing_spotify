use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};
use tracing::info;

#[derive(Serialize, Clone, PartialEq, Eq)]
pub struct Song {
    data: Option<serde_json::Value>,
    fetched: DateTime<Utc>,
}

impl Song {
    pub fn new(song_data: Option<String>, compact: bool) -> Song {
        let data = match song_data {
            Some(song_data) => {
                if compact {
                    match serde_json::from_str::<SongContent>(&song_data) {
                        Ok(converted) => Some(serde_json::to_value(converted).unwrap()),
                        Err(_) => None,
                    }
                } else {
                    Some(serde_json::from_str(&song_data).unwrap())
                }
            }
            None => None,
        };

        if data.is_some() {
            info!("User is currently playing music");
        } else {
            info!("User NOT currently playing music");
        }

        Song {
            data,
            fetched: Utc::now(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(from = "SongContentUnflatten")]
pub struct SongContent {
    #[serde(flatten)]
    pub metadata: SongContentMetadata,
    #[serde(flatten)]
    pub item: Item,
}

impl From<SongContentUnflatten> for SongContent {
    fn from(s: SongContentUnflatten) -> Self {
        Self {
            metadata: s.metadata,
            item: s.item,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SongContentMetadata {
    pub currently_playing_type: String,
    pub is_playing: bool,
    pub progress_ms: i64,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SongContentUnflatten {
    #[serde(flatten)]
    pub metadata: SongContentMetadata,
    pub item: Item,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub duration_ms: i64,
    pub name: String,
    #[serde(rename(deserialize = "external_urls"))]
    #[serde(deserialize_with = "extenal_urls_to_url")]
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Album {
    #[serde(rename(deserialize = "images"))]
    #[serde(deserialize_with = "extract_image_url")]
    pub image_url: String,
    pub name: String,
    #[serde(rename(deserialize = "external_urls"))]
    #[serde(deserialize_with = "extenal_urls_to_url")]
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Artist {
    pub name: String,
    #[serde(rename(deserialize = "external_urls"))]
    #[serde(deserialize_with = "extenal_urls_to_url")]
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

fn extenal_urls_to_url<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let external_urls: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    Ok(external_urls
        .get("spotify")
        .ok_or(de::Error::custom("spotify does not exist in external_urls"))?
        .to_owned())
}

fn extract_image_url<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut images: Vec<Image> = Vec::deserialize(deserializer)?;
    images.sort_by(|a, b| b.height.cmp(&a.height));
    Ok(images
        .get(0)
        .ok_or(de::Error::custom("external_urls does not exist"))?
        .url
        .clone())
}

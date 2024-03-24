use serde::{de, Deserialize, Deserializer, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Result;

#[derive(Serialize, Clone, PartialEq, Eq)]
pub struct Song {
    pub data: Option<serde_json::Value>,
    fetched: u128,
}

impl Song {
    pub fn new(data: Option<serde_json::Value>) -> Result<Song> {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        Ok(Song {
            data,
            fetched: since_the_epoch.as_millis(),
        })
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct SongContentMetadata {
    #[serde(rename(serialize = "type"))]
    pub currently_playing_type: String,
    pub is_playing: bool,
    pub progress_ms: i64,
}

// Ignores the progress_ms field to avoid sending a new message every time the progress changes
impl PartialEq for SongContentMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.currently_playing_type == other.currently_playing_type
            && self.is_playing == other.is_playing
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct SongContentUnflatten {
    #[serde(flatten)]
    pub metadata: SongContentMetadata,
    pub item: Item,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub duration_ms: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Album {
    pub id: String,
    #[serde(rename(deserialize = "images"))]
    #[serde(deserialize_with = "extract_image")]
    pub image: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

fn extract_image<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
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

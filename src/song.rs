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
    #[serde(rename(serialize = "type"))]
    pub currently_playing_type: String,
    pub is_playing: bool,
    pub progress_ms: i64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SongContentUnflatten {
    #[serde(flatten)]
    pub metadata: SongContentMetadata,
    pub item: Item,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub duration_ms: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Album {
    pub id: String,
    #[serde(rename(deserialize = "images"))]
    #[serde(deserialize_with = "extract_image")]
    pub image: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

fn extract_image<'de, D>(deserializer: D) -> Result<String, D::Error>
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

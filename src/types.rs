use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};

pub type HackerNewsId = u32;

pub type HackerNewsItems = Vec<HackerNewsId>;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct HackerNewsStory {
    pub by: String,
    pub descendants: u32,
    pub id: HackerNewsId,
    pub kids: Vec<HackerNewsId>,
    pub score: u32,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    pub title: String,
    pub r#type: String,
    pub url: String,
}
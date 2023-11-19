use std::{time::Duration, future};

use reqwest::{self, Client};

use crate::types::{HackerNewsItems, HackerNewsId, HackerNewsStory};

use futures::future::join_all;

static API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

pub struct HackerNewsClient {
    client: Client,
}

impl HackerNewsClient {
    pub fn new() -> reqwest::Result<Self> {
        let client_builder = reqwest::ClientBuilder::new();
        let client = client_builder.build()?;

        Ok(Self { client })
    }

    pub async fn get_top_story_ids(&self) -> reqwest::Result<HackerNewsItems>{
        self.client.get(&format!("{}/topstories.json", API_BASE_URL)).send().await?.json::<HackerNewsItems>().await
    }

    pub async fn get_new_story_ids(&self) -> reqwest::Result<HackerNewsItems>{
        self.client.get(&format!("{}/newstories.json", API_BASE_URL)).send().await?.json::<HackerNewsItems>().await
    }

    pub async fn get_story(&self, id: HackerNewsId) -> reqwest::Result<HackerNewsStory> {
        self.client.get(&format!("{}/item/{}.json", API_BASE_URL, id)).send().await?.json::<HackerNewsStory>().await
    }

    pub async fn get_top_stories(&self) -> reqwest::Result<Vec<HackerNewsStory>> {
        let story_ids = &self.get_top_story_ids().await?[..10];

        let story_futures = story_ids[..usize::min(story_ids.len(), 10)].iter().map(|&story_id| self.get_story(story_id));

        let stories = join_all(story_futures)
            .await
            .into_iter()
            .filter_map(|story| story.ok())
            .collect();

        Ok(stories)
    }

    pub async fn get_new_stories(&self) -> reqwest::Result<Vec<HackerNewsStory>> {
        let story_ids = &self.get_new_story_ids().await?[..10];

        let story_futures = story_ids[..usize::min(story_ids.len(), 10)].iter().map(|&story_id| self.get_story(story_id));

        let stories = join_all(story_futures)
            .await
            .into_iter()
            .filter_map(|story| story.ok())
            .collect();

        Ok(stories)
    }

}

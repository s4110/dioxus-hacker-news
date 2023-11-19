use dioxus::prelude::*;
use crate::hacker_news_client::HackerNewsClient;
use crate::components::story_list::StoryList;

pub fn Home(cx: Scope) -> Element {
    let stories = use_future!(
        cx,
        |()| async move {
            let client = HackerNewsClient::new().unwrap();
            client.get_top_stories().await
        }
    );

    match stories.value() {
        Some(Ok(list)) => {
            render!(rsx!( StoryList { items: list } ))
        }
        Some(Err(err)) => {
            render!(rsx! { div { "Err" } })
        }
        None => {
            render!(rsx! { div { "Loading..." } })
        }
    }
}
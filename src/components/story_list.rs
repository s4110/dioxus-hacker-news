use dioxus::prelude::*;

use crate::types::HackerNewsStory;
use chrono::{DateTime, Utc};

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    pub items: &'a Vec<HackerNewsStory>,
}

fn get_host_str(url: &str) -> String {
    let parsed_url = url::Url::parse(url);

    match parsed_url {
        Ok(url) => match url.host_str() {
            Some(str) => str.trim_start_matches("www.").to_string(),
            None => "".to_string(),
        },
        Err(_) => "".to_string()
    }
}

fn get_score_str(score: &u32) -> String {
    format!("{score} {}", if *score == 1 {"point"} else {"points"})
}

fn get_comments_str(kids: &Vec<u32>) -> String {
    format!("{} {}", kids.len(), if kids.len() == 1 {"comment"} else {"comments"})
}

fn get_time_str(time: DateTime<Utc>) -> String {
    time.format("%Y/%m/%d %H:%M").to_string()
}

pub fn StoryList<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        ul {
            cx.props.items.iter().map(|item| {
                rsx! {
                li {
                    class: "mb-[5px] last:mb-0",
                    div {
                        a {
                            class: "text-black font-normal text-[13px]",
                            href: "{item.url}",
                            target: "_blank",
                            "{item.title}"
                        }
                        a {
                            class: "ml-[5px] text-[10px] hover:underline",
                            href: "https://news.ycombinator.com/from?site={get_host_str(&item.url)}",
                            "({get_host_str(&item.url)})"
                        }
                        div {
                            class: "text-[10px] mt-[2px]",
                            "{get_score_str(&item.score)} by ",
                            a{
                                class: "hover:underline",
                                href: "https://news.ycombinator.com/user?id={item.by}",
                                "{item.by}"
                            },
                            " {get_time_str(item.time)} {get_comments_str(&item.kids)}"
                        }
                    }
                }
            }})
        }
    })
}
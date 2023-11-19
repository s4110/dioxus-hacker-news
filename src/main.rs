#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod page;
mod components;
mod layouts;
mod types;
mod hacker_news_client;

use page::home::Home;
use page::news::News;
use page::not_found::NotFound;
use layouts::default_layout::DefaultLayout;

fn main() {
    dioxus_logger::init(LevelFilter::max()).expect("failed to init logger");
    dioxus_web::launch(App);
}

#[rustfmt::skip]
#[derive(Clone, Routable)]
enum Route {
    #[layout(DefaultLayout)]
        #[route("/news")]
        News {},
        #[route("/")]
        Home {},
        #[route("/:..route")]
        NotFound {
            route: Vec<String>,
        },    
}

fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}





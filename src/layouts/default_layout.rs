use dioxus::html::GlobalAttributes;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

use crate::components::navi::Navi;

pub fn DefaultLayout(cx: Scope) -> Element {
    render! {
        div { class: "w-[85%] mx-auto",
            Navi {}
            div { class: "bg-[#f6f6ef]", Outlet::<Route> {} }
        }
    }
}
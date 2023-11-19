use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Navi(cx: Scope) -> Element {
    cx.render(rsx! {
        nav { class: "p-[2px] bg-[#ff6600] text-black",
            ul { class: "flex",
                li { class: "font-bold mr-[5px]", a { href: "/", "Hacker News" } }
                li { class: "mr-[5px]",
                    Link { to: "/news", "new" }
                }
            }
        }
    })
}
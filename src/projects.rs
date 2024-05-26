use dioxus::prelude::*;

#[component]
pub fn Projects(project: String) -> Element {
    rsx!{
        link { href: "/styles.css" },
        h1 { "{project}" }
    }
}

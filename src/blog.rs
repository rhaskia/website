use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Blog(id: String) -> Element {
    rsx! {
        div {
            class: "page blog",
            "Blog post {id}"
        }
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div {
            class: "page blog-list",
            h1 { "my blogs" }
        }
    }
}

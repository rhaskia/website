use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        "Blog post {id}"
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        h1 { "my blogs" }
    }
}

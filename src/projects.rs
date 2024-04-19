use dioxus::prelude::*;

#[component]
pub fn Projects(project: String) -> Element {
    rsx!{
        h1 { "{project}" }
    }
}

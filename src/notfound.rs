use dioxus::prelude::*;

#[component]
pub fn Page404(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "{segments:?} Not Found" }
    }
}

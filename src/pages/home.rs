use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx!{
        div {
            class: "homepage",
            section {
                h2 {
                    "idk"
                }
            }
        }
    }
}

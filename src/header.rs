use dioxus::prelude::*;

pub fn Header() -> Element {
    rsx!{
        header {
            class: "header",
            div {
                class: "header-left",
                "rhaskia"
            }
            div {
                class: "header-right",
            }
        }
    }
}

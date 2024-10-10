use dioxus::prelude::*;
use crate::Window;

#[component]
pub fn Terminal() -> Element {
    rsx! {
        Window {
            title: "terminal",
            window_color: "#272736",
            x: 0.0,
            y: 0.0,
            width: 400.0,
            height: 500.0,
            pre {
            }
            input {
            }
        }
    }
}

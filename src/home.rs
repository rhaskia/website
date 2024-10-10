use dioxus::prelude::*;
use crate::Window;
use crate::Terminal;

#[component]
pub fn Home() -> Element {
    rsx! {
        Window {
            title: "about me",
            window_color: "#f2f2e7",
            x: 0.0,
            y: 0.0,
                width: 400.0,
                height: 500.0,
        }
        Terminal {}
    }
}

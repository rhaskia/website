use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx!{
        div {
            class: "homepage",
            section {
                "Email: " 
                a { 
                    href: "mailto:rhaskiagamedev@gmail.com",
                    "rhaskiagamedev@gmail.com" 
                }
                br {}
                br {}
                "Github: "
                a {
                    href: "https://github.com/rhaskia",
                    "https://github.com/rhaskia"
                }
            }
        }
    }
}

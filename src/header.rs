use dioxus::prelude::*;
use crate::Route;

pub fn Header() -> Element {
    rsx!{
        header {
            class: "header",
            div {
                class: "header-left",
                Link {
                    to: Route::Home {},
                    "home"
                }
            }
            div {
                class: "header-right",
                Link {
                    to: Route::BlogList {},
                    "󱉟 blog"
                }
            }
        }
        Outlet::<Route>{}
    }
}

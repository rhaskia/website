use dioxus::prelude::*;
use crate::Route;

pub fn NvimBar() -> Element {
    rsx!{
        pre {
           class: "topbar",
           span {
               background: "green",
               color: "white",
               " NORMAL "
           }
        }
    }
}

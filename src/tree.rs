use dioxus::prelude::*;
use crate::Route;
use dioxus::router::components::IntoRoutable;

#[component] 
pub fn Tree() -> Element {
    rsx! {
        pre {
            id: "tree",
            class: "tree",
            r#"~/website"#

            TreeFolder {
                to: Route::BlogList {}.into(),
                name: "blog",
                rec: 1,
                BlogTree {}
            }
        }
    }
}

pub fn BlogTree() -> Element {
    rsx! {
        span {
            " │ 󰈙 "
            Link {
                to: Route::Blog { id: "1".to_string() },
                "blog 1"
            }
        }
        span {
            " ╰ 󰈙 "
            Link {
                to: Route::Blog { id: "1".to_string() },
                "blog 2"
            }
        }
    }
}

#[component] 
pub fn TreeFolder(to: IntoRoutable, name: String, children: Element, rec: usize) -> Element {
    let mut open = use_signal(|| false);
    let symbol = use_memo(move || if open() { "   " } else { "   " });

    rsx! {
        span {
            style: "--rec: {rec}",
            button {
                //display: "inline",
                class: "folder-button",
                onclick: move |_| open.set(!open()),
                "{symbol}"
            }
            Link {
                to,
                "{name}"
            }
        }
        if open() {
            {children}
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            class: "page home",
            h1 { "About me" }
            pre {
                "hi! I'm rhaskia, a programmer and game developer"
            }
        }
    }
}

use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx!{
        div {
            class: "homepage",
            section {
                p {
                    "Hi! I'm Rhaskia and I make things."
                    "You can find some of my projects "
                    Link { to: Route::Projects {}, "here" }
                    br {} 
                    "I am very interested in programming and mathematics, and try to mix the two whenever I can"
                    br {} 
                    "Some of the programming languages I know include Rust (my current favourite), C++, C#, Python, and Javascript."
                    br {} 
                    "Some languages I'm looking to learn at some point in the future include Zig, Odin, and Haskell."
                }
            }
        }
    }
}

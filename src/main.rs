mod components;
mod pages;

use dioxus::{document::Link, prelude::*};
use dioxus_router::prelude::*;

use components::Navbar;
use pages::Home;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Link { href: "assets/styles.css" }
        Link { rel:"preconnect", href:"https://fonts.googleapis.com" }
        Link { rel:"preconnect", href:"https://fonts.gstatic.com", crossorigin: "true" }
        Link { 
            href:"https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap",
            rel:"stylesheet"
        }

        Router::<Route> {}
    }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
}

#[component]
fn Blog() -> Element {
    rsx! {
        "hi"
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    todo!()
}

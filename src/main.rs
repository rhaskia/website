use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    //#[redirect("/:..segments", |segments: Vec<String>| Route::Home {})]
    Home {},
    #[route("/blog")]
    Blog {},
}

#[component]
fn Home() -> Element {
    rsx!{
        h2 {
            "hello"
        }
    }
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

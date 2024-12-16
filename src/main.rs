use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(App);
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

fn Home() -> Element {
    todo!()
}

fn Blog() -> Element {
    todo!()
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    todo!()
}

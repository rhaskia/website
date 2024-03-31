#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod blog;
mod home;
mod header;
use blog::{Blog, BlogList};
use header::Header;
use home::Home;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/blog/")]
    BlogList {}
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Header {}
        Router::<Route> {}
    }
}


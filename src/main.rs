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
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
        #[route("/blog/")]
        BlogList {}
}

#[component]
pub fn App() -> Element {
    rsx!{ Router::<Route> {} }
}

pub fn Wrapper() -> Element {
    rsx!{
        Header {}
    }
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}


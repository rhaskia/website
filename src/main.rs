#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod blog;
mod home;
mod tree;
mod nvimbar;
use blog::{Blog, BlogList};
use home::Home;
use tree::Tree;
use nvimbar::NvimBar;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: String },
        #[route("/blog/")]
        BlogList {}
}

#[component]
pub fn App() -> Element {
    rsx!{ Router::<Route> {} }
}

#[component]
pub fn Wrapper() -> Element {
    rsx!{
        div {
            class: "page-tree-div",
            Tree {}
            Outlet::<Route> {} 
        }
        NvimBar {}
    }
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}


#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use log::LevelFilter;

mod blog;
mod home;
mod tree;
mod nvimbar;
mod notfound;
mod projects;
use blog::{Blog, BlogList};
use home::Home;
use tree::Tree;
use nvimbar::NvimBar;
use notfound::Page404;
use projects::Projects;

#[derive(Clone, Routable, Debug, PartialEq, Deserialize, Serialize)]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: String },
        #[route("/blogs/")]
        BlogList {},
        #[route("/:..segments")]
        Page404 { segments: Vec<String> },
        #[route("/projects/:project")]
        Projects { project: String },
}

#[component]
pub fn App() -> Element {
    rsx!{ 
        Router::<Route> {}
    }
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

    LaunchBuilder::fullstack().launch(App);
}


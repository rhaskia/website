mod home;
mod window;
mod terminal;
use home::Home;
use terminal::Terminal;
use window::Window;

use dioxus::prelude::*;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Deserialize, Serialize)]
enum Route {
    #[layout(Wrapper)]
    // // #[route("/:..segments")]
    // // Page404 { segments: Vec<String> },
    #[route("/")]
    Home {},
    // #[route("/blogpost/:id")]
    // Blog { id: String },
    // // BlogList {},
    // #[route("/projects/:project")]
    // Projects { project: String },
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        div {
            class: "page-tree-div",
            Outlet::<Route> {}
            TaskBar {}
        }
    }
}

#[component]
pub fn TaskBar() -> Element {
    rsx! {
        div {
            class: "taskbar",
        }
    }
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    LaunchBuilder::fullstack().launch(App);
}

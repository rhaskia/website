use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;
use crate::Language;
use crate::Project;
use crate::pages::blog::{get_blogs, to_title_case};

#[component]
pub fn Home() -> Element {
    let recent_blogs: Signal<Vec<(&str, &str)>> = use_signal(|| get_blogs().into_iter().take(5).collect());

    rsx!{
        div {
            class: "homepage",
            section {
                p {
                    span {
                        "Hi! I'm Rhaskia and I make things, you can find some of my projects "
                        Link { to: Route::Projects {}, "here." }
                    } 
                    br { }
                    br { }
                    span {
                        line_height: "1.5",
                        "I am interested in programming, linux, the terminal and mathematics, and try to incorporate them into my projects when I can. "
                        "Some of the programming languages I know include Rust (my current favourite), C++, C#, Python, and Javascript. "
                        "Some languages I'm looking to learn at some point in the future include Zig, Odin, and Haskell."
                    }
                    br {}
                    br {} 
                }
                h3 { "Some of my favourite ", Link { to: Route::Projects {}, "projects" }, " I've worked on." },
                section {
                    class: "projectcontainer preview",
                    Project { 
                        name: "PreTTY",
                        description: "Customizable terminal emulator made with web technologies.",
                        timeframe: "2024 Feb - Ongoing",
                        link: "https://github.com/rhaskia/preTTY",
                        langs: vec![Language::Rust, Language::Html, Language::Css],
                    },
                    Project { 
                        name: "Music Player",
                        description: "Music player made for desktop and android with radio system for offline files.",
                        timeframe: "2024 December - Ongoing",
                        link: "https://github.com/rhaskia/trackfish",
                        langs: vec![Language::Rust],
                    }   
                    Project {
                        name: "Quantum Simulator",
                        description: "Interactive quantum computer simulator",
                        timeframe: "2025 January",
                        link: "https://github.com/rhaskia/quantum",
                        langs: vec![Language::Rust, Language::Html, Language::Css],
                    }
                    Project { 
                        name: "Wave Simulator",
                        description: "A wave simulator made using numerical differentation.",
                        timeframe: "2024 Aug - Oct",
                        link: "https://github.com/rhaskia/wavesim",
                        langs: vec![Language::Rust]
                    }   
                    Project { 
                        name: "Treestack",
                        description: "Programming language with tree-based memory.",
                        timeframe: "2024 June - July",
                        link: "https://github.com/rhaskia/treestack",
                        langs: vec![Language::Rust]
                    }   
                    Project { 
                        name: "Rocket Metrics",
                        description: "Metrics for a model rocket that I designed and launched.",
                        timeframe: "2024 April - Oct",
                        link: "https://github.com/rhaskia/rocketmetrics",
                        langs: vec![Language::CPlusPlus]
                    }  
                }
                h3 { "Recent " Link { to: Route::Blog {}, "blog posts" } }
                for blog in recent_blogs() {
                    div {
                        class: "bloglink",
                        span { 
                            display: "inline",
                            margin: 0,
                            a { href: "/blog/{blog.0}", "{to_title_case(blog.0)}" }
                        }
                        time { datetime: blog.1.clone(), "{blog.1}" }
                    }
                }
            }
        }
    }
}

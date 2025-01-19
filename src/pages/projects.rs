use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx!{
        div {
            class: "homepage",
            section {
                Project { 
                    name: "Watch Face",
                    description: "Custom watch face for infinitime with music controls",
                    timeframe: "2024 December",
                    langs: vec![Language::CPlusPlus]
                }   
                Project { 
                    name: "Music Player",
                    description: "Music player made for desktop and android with radio system for offline files.",
                    timeframe: "2024 December",
                    langs: vec![Language::Rust]
                }   
                Project { 
                    name: "Wave Simulator",
                    description: "A wave simulator made using numerical differentation.",
                    timeframe: "2024 Aug - Oct",
                    langs: vec![Language::Rust]
                }   
                Project { 
                    name: "Dioxus Form",
                    description: "An automatic rust struct to html form serializer.",
                    timeframe: "2024 July - Aug",
                    langs: vec![Language::Rust, Language::Html]
                }   
                Project { 
                    name: "Web Lang",
                    description: "Golfing language made to compile into html, css and javascript",
                    timeframe: "2024 July - Aug",
                    langs: vec![Language::Rust, Language::Html, Language::Css, Language::Javascript]
                }   
                Project { 
                    name: "Treestack",
                    description: "Programming language with tree-based memory.",
                    timeframe: "2024 June - July",
                    langs: vec![Language::Rust]
                }   
                Project { 
                    name: "Cell Lang",
                    description: "Esolang built upon ideas of cellular automata.",
                    timeframe: "2024 May - June",
                    langs: vec![Language::Rust]
                }   
                Project {
                    name: "Markdown Editor",
                    description: "Terminal markdown editor with support for double height headers",
                    timeframe: "2024 May",
                    langs: vec![Language::Rust],
                }
                Project { 
                    name: "Rocket Metrics",
                    description: "Metrics for a model rocket that I designed and launched.",
                    timeframe: "2024 April - Oct",
                    langs: vec![Language::CPlusPlus]
                }  
                Project {
                    name: "Tex Shell",
                    description: "Calculator shell meant to render and computer latex equations.",
                    timeframe: "2024 March",
                    langs: vec![Language::OCaml]
                }
                Project { 
                    name: "PreTTY",
                    description: "Customizable terminal emulator made with web technologies.",
                    timeframe: "2024 Feb - Ongoing",
                    langs: vec![Language::Rust, Language::Html, Language::Css]
                }   
                Project { 
                    name: "Neorg Obsidian",
                    description: "Obsidian plugin for neorg file type.",
                    timeframe: "2024 January",
                    langs: vec![Language::Javascript, Language::C]
                }   
            }
        }
    }
}

#[component]
pub fn Project(name: String, description: String, timeframe: String, langs: Vec<Language>, children: Element) -> Element {
    rsx! {
        div {
            class: "projectblock",
            div {
                class: "projectheader",
                div {
                    h3 {
                        display: "inline",
                        "{name}"
                    }
                    for lang in langs {
                        Tag { lang }
                    }
                }
                span { "{timeframe}" }
            }

            p { "{description}" }
            {{ children }}
        }
    }
}

#[component]
pub fn Tag(lang: Language) -> Element {
    rsx! {
        div {
            class: "langtag",
            background: lang.colour(),
            color: lang.fg(),
            "{lang:?}"
        }
    } 
}

#[derive(Debug, PartialEq, Clone)]
pub enum Language {
    Rust,
    Javascript,
    Html, 
    Css,
    CSharp,
    CPlusPlus,
    Haskell,
    OCaml,
    C,
}

impl Language {
    pub fn colour(&self) -> &str {
        match self {
            Language::Rust => "#dea584",
            Language::Javascript => "#f1e05a",
            Language::Html => "#dd4b24",
            Language::Css => "#0068ba",
            Language::CSharp => todo!(),
            Language::CPlusPlus => "#6196ca",
            Language::Haskell => todo!(),
            Language::OCaml => "#ef7a08",
            Language::C => "#014483",
        }
    }

    pub fn fg(&self) -> &str {
        match self {
            Language::Rust => "#1d2021",
            Language::Javascript => "#1d2021",
            Language::Html => "#ffffeb",
            Language::Css => "#ffffeb",
            Language::CSharp => todo!(),
            Language::CPlusPlus => "#ffffeb",
            Language::Haskell => todo!(),
            Language::OCaml => "#ffffeb",
            Language::C => "#ffffeb",
        }
    }
}

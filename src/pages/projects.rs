use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx!{
        div {
            class: "homepage",
            section {
                class: "projectcontainer",
                Project { 
                    name: "PreTTY",
                    description: "Customizable terminal emulator made with web technologies.",
                    timeframe: "2024 Feb - Ongoing",
                    langs: vec![Language::Rust, Language::Html, Language::Css],
                    link: "https://github.com/rhaskia/preTTY",
                    img { src: "https://github.com/rhaskia/preTTY/raw/main/example.png" }
                }  
                Project { 
                    name: "Music Player",
                    description: "Music player made for desktop and android with radio system for offline files.",
                    timeframe: "2024 December - Ongoing",
                    langs: vec![Language::Rust],
                    link: "https://github.com/rhaskia/trackfish",
                    img { src: "https://github.com/rhaskia/trackfish/raw/main/exampleimage.png" }
                }   
                Project {
                    name: "Quantum Simulator",
                    description: "Interactive quantum computer simulator",
                    timeframe: "2025 January",
                    langs: vec![Language::Rust, Language::Html, Language::Css],
                    link: "https://github.com/rhaskia/quantum",
                    img { src: "https://github.com/rhaskia/quantum/raw/main/example.png" }
                }
                Project { 
                    name: "Watch Face",
                    description: "Custom watch face for infinitime with music controls",
                    timeframe: "2024 December",
                    langs: vec![Language::CPlusPlus],
                    link: "https://github.com/rhaskia/InfiniTime",
                    img { src: "https://github.com/rhaskia/InfiniTime/raw/main/face.png" }
                }   
                Project { 
                    name: "Wave Simulator",
                    description: "A wave simulator made using numerical differentation.",
                    timeframe: "2024 Aug - Oct",
                    link: "https://github.com/rhaskia/wavesim",
                    langs: vec![Language::Rust]
                }   
                Project { 
                    name: "Dioxus Form",
                    description: "An automatic rust struct to html form serializer.",
                    timeframe: "2024 July - Aug",
                    link: "https://github.com/rhaskia/dioxus-form",
                    langs: vec![Language::Rust, Language::Html]
                }   
                Project { 
                    name: "Web Lang",
                    description: "Golfing language made to compile into html, css and javascript",
                    timeframe: "2024 July - Aug",
                    link: "https://github.com/rhaskia/sillyweblang",
                    langs: vec![Language::Rust, Language::Javascript]
                }   
                Project { 
                    name: "Treestack",
                    description: "Programming language with tree-based memory.",
                    timeframe: "2024 June - July",
                    link: "https://github.com/rhaskia/treestack",
                    langs: vec![Language::Rust]
                }   
                Project { 
                    name: "Cell Lang",
                    description: "Esolang built upon ideas of cellular automata.",
                    timeframe: "2024 May - June",
                    link: "https://github.com/rhaskia/celllang",
                    langs: vec![Language::Rust]
                }   
                Project {
                    name: "Markdown Editor",
                    description: "Terminal markdown editor with support for double height headers",
                    timeframe: "2024 May",
                    link: "https://github.com/rhaskia/md_editor",
                    langs: vec![Language::Rust],
                }
                Project { 
                    name: "Rocket Metrics",
                    description: "Metrics for a model rocket that I designed and launched.",
                    timeframe: "2024 April - Oct",
                    link: "https://github.com/rhaskia/rocketmetrics",
                    langs: vec![Language::CPlusPlus]
                }  
                Project {
                    name: "Tex Shell",
                    description: "Calculator shell meant to render and computer latex equations.",
                    timeframe: "2024 March",
                    link: "https://github.com/rhaskia/tex_shell",
                    langs: vec![Language::OCaml]
                }
                Project { 
                    name: "Neorg Obsidian",
                    description: "Obsidian plugin for neorg file type.",
                    timeframe: "2024 January",
                    link: "https://github.com/rhaskia/trackfish",
                    langs: vec![Language::Javascript, Language::C]
                }   
            }
        }
    }
}

#[component]
pub fn Project(
    name: String, 
    description: String, 
    timeframe: String, 
    langs: Vec<Language>, 
    link: String,
    children: Element) -> Element {
    rsx! {
        a {
            href: link,
            class: "projectblock",
            div {
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
                }
                small { "{timeframe}" }
                br { }

                p { "{description}" }
                {{ children }}
            }
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

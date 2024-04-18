use crate::blog::{get_blogs, BlogFile};
use crate::Route;
use dioxus::prelude::*;
use dioxus::router::components::IntoRoutable;

#[component]
pub fn Tree() -> Element {
    let blogs = use_resource(get_blogs);

    rsx! {
        pre {
            id: "tree",
            class: "tree",
            r#"~/website"#

            if let Some(Ok(blog)) = &*blogs.read_unchecked() {
                TreeFolder { lead: "assets/blog/", file: blog.clone(), rec: 0, indent: "", icon: "" }
            }
        }
    }
}

#[component]
pub fn TreeFolder(lead: String, file: BlogFile, rec: usize, indent: String, icon: String) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        match file {
            BlogFile::Folder { ref path, ref children } => rsx! { 
                div {
                    onclick: move |_| open.set(!open()),
                    {format!("{indent}{}", if open() { " " } else { " " })},
                    if open() {
                        span { style: "color: var(--light-blue)", " {path}" }
                    } else {
                        span { style: "color: var(--light-blue)", " {path}" }
                    }
                }
                if open() {
                    for (i, child) in children.iter().enumerate() {
                        TreeFolder { 
                            lead: "{lead}{path}/",
                            file: child.clone(),
                            rec: rec + 1,
                            indent: indent.to_owned() + if i >= child.len() - 1 {"╰ "} else {"│ "},
                            icon: icon.clone()
                        }
                    }
                }
            },
            BlogFile::File { path } => rsx! { div { "{indent}{icon} {path}" } },
        }
    }
}

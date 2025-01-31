use chrono::{DateTime, Local};
use dioxus::{prelude::*, logger::tracing::info};
use serde::{Deserialize, Serialize};
use std::path::Path;
use yaml_front_matter::YamlFrontMatter;
use web_sys::window;

pub fn get_blogs() -> Vec<(&'static str, &'static str)> {
    vec![("tokenizer-compress", "2024-04-06")]
}

#[component]
pub fn Blog() -> Element {
    let blogs = use_signal(get_blogs);

    rsx! {
        div {
            class: "blogpage",
            section {
                id: "blogsection",
                h2 { "Blog Posts" }
                for blog in blogs() {
                    div {
                        class: "bloglink",
                        h3 { 
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

#[component]
pub fn BlogPost(segments: Vec<String>) -> Element {
    let segments = use_signal(|| segments);
    let file = use_signal(|| segments()[0].clone());
    let title = use_signal(|| to_title_case(&file()));
    let post: Resource<Result<(String, Metadata), anyhow::Error>> = use_resource(move || async move { 
        let domain = window().unwrap().location().origin().unwrap();
        let url = format!("{domain}/assets/blog/{file}.md"); 
        info!("{url}");
        let result = reqwest::get(url)
            .await?.text().await?; 
        Ok(load_blog(result))
    });

    rsx! {
        div {
            class: "blogpage",
            section {
                match &*post.read_unchecked() {
                    Some(Ok((html, yaml))) => rsx!{
                        h2 { "{title}" }
                        h3 { "{yaml.description}" }
                        p { "Published on {yaml.published}" }
                        hr {}
                        p {
                            dangerous_inner_html: "{html}",
                        }
                    },
                    Some(Err(err)) => rsx!{
                        "{err:?}"
                    },
                    _ => rsx!{ "..." },
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlogInfo {
    time: String,
    name: String,
    link: String,
    file: String,
}

#[derive(Deserialize, Serialize)]
pub struct Metadata {
    published: String,
    description: String,
    tags: Vec<String>,
}

// async fn get_blogs() -> Result<Vec<BlogInfo>, ServerFnError> {
//     let blog_files = std::fs::read_dir("./blog");
//     let mut blogs = Vec::new();
//
//     for file in blog_files? {
//         let file = file?;
//         let system_time = file.metadata()?.created()?;
//         let datetime: DateTime<Local> = system_time.into();
//         let time = datetime.format("%Y-%m-%d").to_string();
//         let link = file.path().file_prefix().unwrap().to_str().unwrap().to_string();
//         let file = file.path().display().to_string();
//         let name = to_title_case(&link);
//
//         let blog = BlogInfo { time, name, link, file };
//
//         blogs.push(blog);
//     }
//
//     Ok(blogs)
// }

fn load_blog(markdown: String) -> (String, Metadata) {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let yaml_data = YamlFrontMatter::parse::<Metadata>(&markdown).unwrap().metadata;

    (html_output, yaml_data)
}

pub fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    let mut last_space = true;

    for c in s.chars() {
        if c == '-' {
            result.push(' ');
            last_space = true;
            continue;
        }

        if last_space {
            result.push(c.to_uppercase().collect::<String>().chars().next().unwrap());
            last_space = false;
        } else {
            result.push(c);
        }
    }

    result
}

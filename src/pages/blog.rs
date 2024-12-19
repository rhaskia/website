use chrono::{DateTime, Local};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use yaml_front_matter::YamlFrontMatter;

#[component]
pub fn Blog() -> Element {
    let blogs = use_resource(get_blogs);

    rsx! {
        div {
            class: "blogpage",
            section {
                id: "blogsection",
                h2 { "Blog Posts" }


                match &*blogs.read_unchecked() {
                    Some(Ok(response)) => rsx!{
                        for blog in response {
                            div {
                                class: "bloglink",
                                time { datetime: blog.time.clone(), "{blog.time}" }
                                Link { to: "/post/{blog.link}", "{blog.name}" }
                            }
                        }
                    },
                    _ => rsx!{ "..." }
                }
            }
        }
    }
}

#[component]
pub fn BlogPost(segments: Vec<String>) -> Element {
    let segments = use_signal(|| segments);
    let title = use_signal(|| to_title_case(&segments()[0]));
    let post = use_resource(move || async move { load_blog(segments()[0].clone()).await });

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

#[server]
async fn get_blogs() -> Result<Vec<BlogInfo>, ServerFnError> {
    let blog_files = std::fs::read_dir("./blog");
    let mut blogs = Vec::new();

    for file in blog_files? {
        let file = file?;
        let system_time = file.metadata()?.created()?;
        let datetime: DateTime<Local> = system_time.into();
        let time = datetime.format("%Y-%m-%d").to_string();
        let link = file.path().file_prefix().unwrap().to_str().unwrap().to_string();
        let file = file.path().display().to_string();
        let name = to_title_case(&link);

        let blog = BlogInfo { time, name, link, file };

        blogs.push(blog);
    }

    Ok(blogs)
}

#[server]
async fn load_blog(link: String) -> Result<(String, Metadata), ServerFnError> {
    let path_str = "./blog/".to_string() + &link + ".md";
    let path = Path::new(&path_str);
    let blog_folder = Path::new("./blog/");

    if !path_is_within_folder(&path, &blog_folder) {
        return Err(ServerFnError::ServerError("Access denied".to_string()));
    }

    let file = std::fs::read_to_string(path)?;
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&file, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let yaml_data = YamlFrontMatter::parse::<Metadata>(&file).unwrap().metadata;

    Ok((html_output, yaml_data))
}

fn path_is_within_folder(path: &Path, folder: &Path) -> bool {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let folder = folder.canonicalize().unwrap_or_else(|_| folder.to_path_buf());

    path.starts_with(&folder)
}

fn to_title_case(s: &str) -> String {
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

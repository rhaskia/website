use chrono::{DateTime, Local};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
                            time { datetime: blog.time.clone(), "{blog.time}" }
                            Link { to: "/post/{blog.link}", "{blog.name}" }
                        }
                    },
                    _ => rsx!{ "Loading blogs..." }
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

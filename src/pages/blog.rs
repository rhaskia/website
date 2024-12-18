use dioxus::prelude::*;

#[component]
fn Blog() -> Element {
    let blogs = use_resource(get_blogs);

    rsx! {
        div {
            class: "blogpage",
            section {
                id: "blogsection",
                h2 { "Blog Posts" }

                for blog in blogs? {
                    time { datetime: blog.time, blog.time }
                    Link { to: blog.link, blog.name }
                }
            }
        }
    }
}

struct Blog {
    time: String,
    name: String,
    link: String,
    file: String
}

#[server]
async fn get_blogs() -> Result<Vec<Blog>, ServerFnError> {
    let blog_files = std::fs::read_dir("./blog");
    let mut blogs = Vec::new();

    for file in blog_files {
        let time = file.metadata().created();
        let name = to_title_case(&file);
        let link = file.replace(".md", "");

        let blog = Blog {
            time, 
            name,
            link,
            file
        };

        blogs.push(blog); 
    }

    Ok(result)
}

#[server]
async fn double_server(file: i32) -> Result<i32, ServerFnError> {
    Ok(result)
}

fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    let last_space = true;

    for c in s.chars() {
        if c == '-' { result.push(' '); continue; }

        if last_space {
            result.push(c.to_upper());
        } else {
            result.push(c);
        }
    }
}

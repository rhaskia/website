use std::fs;
use std::fs::ReadDir;
use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::Route;

#[component]
pub fn Blog(id: String) -> Element {
    rsx! {
        div {
            class: "page blog",
            "Blog post {id:?}"
        }
    }
}

#[component]
pub fn BlogList() -> Element {
    let blogs = use_resource(get_blogs);

    rsx! {
        div {
            class: "page blog-list",
            h1 { "my blogs" }
            if let Some(Ok(r)) = &*blogs.read_unchecked() {
                
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum BlogFile {
    Folder { path: String, children: Vec<BlogFile> },
    File { path: String }
}

impl BlogFile {
    pub fn set_path(&mut self, p: String) {
        match self {
            Self::Folder { path, children } => *path = p.to_string(),
            Self::File { path } => *path = p.to_string(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            BlogFile::Folder { path: _, ref children } => 1 + children.iter().map(|c| c.len()).sum::<usize>(),
            BlogFile::File { path: _ } => 1,
        }
    }
}

#[server]
pub async fn get_blogs() -> Result<BlogFile, ServerFnError> {
    let mut root = fs::read_dir("./assets/blog")?; 
    Ok(dir_get_blogs(&mut root)?)
}

pub fn dir_get_blogs(dir: &mut ReadDir) -> Result<BlogFile, ServerFnError> {
    let mut children = Vec::new();
    while let Some(Ok(dir)) = dir.next() {
        let file = dir.file_type()?;
        let name = dir.file_name().to_str().unwrap().to_string();

        if file.is_dir() {
            let mut folder = dir_get_blogs(&mut fs::read_dir(dir.path())?)?;
            folder.set_path(name);
            children.push(folder);
        } else {
            let path = dir.path();
            let name = path.file_stem().unwrap().to_str().unwrap();
            children.push(BlogFile::File { path: name.to_string() });
        }
    }
    Ok(BlogFile::Folder { path: String::from("blog"), children })
}

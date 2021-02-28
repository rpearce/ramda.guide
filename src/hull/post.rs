use pulldown_cmark;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PostFrontMatter {
    pub author: String,
    pub author_twitter: String,
    pub description: String,
    #[serde(default = "default_fm_str")]
    pub keywords: String,
    pub published_at: String,
    pub slug: String,
    pub title: String,
    #[serde(default = "default_fm_str")]
    pub updated_at: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Post {
    pub content_html: String,
    pub data: PostFrontMatter,
}

fn default_fm_str() -> String {
    String::new()
}

pub fn load(src: &str) -> Result<Vec<Post>, io::Error> {
    // Setup Markdown
    let md_opts = get_md_opts();

    let path = Path::new(src);
    let mut posts: Vec<Post> = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension().unwrap() != "md" {
            continue;
        }

        let post = parse(&path, md_opts).expect(&format!("Failed to get post: {:#?}", path));

        posts.push(post);
    }

    return Ok(posts);
}

fn get_md_opts() -> pulldown_cmark::Options {
    let mut md_opts = pulldown_cmark::Options::empty();
    md_opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    md_opts
}

fn parse(path: &PathBuf, md_opts: pulldown_cmark::Options) -> Result<Post, io::Error> {
    let file_content = fs::read_to_string(path)?;
    let (front_matter, content) = parse_front_matter(file_content)?;
    let content_html = parse_markdown(&content, md_opts);

    let post = Post {
        content_html,
        data: front_matter,
    };

    Ok(post)
}

fn parse_front_matter(file_content: String) -> Result<(PostFrontMatter, String), io::Error> {
    let split: Vec<&str> = file_content
        .split("+++")
        .filter(|&x| x != "")
        .map(|x| x.trim())
        .collect();

    let front_matter: PostFrontMatter = toml::from_str(split[0])?;
    let content = split[1];

    Ok((front_matter, content.to_string()))
}

fn parse_markdown(content: &str, md_opts: pulldown_cmark::Options) -> String {
    let md_parser = pulldown_cmark::Parser::new_ext(content, md_opts);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, md_parser);

    html
}

pub fn setup(dir: &str) -> Result<(), io::Error> {
    let path = Path::new(dir);

    if path.exists() {
        fs::remove_dir_all(path).expect("Failed to remove posts output");
        println!("Removed... {:#?}", path);
    }

    fs::create_dir(path).expect("Failed to create posts output");
    println!("Created {:#?}...", path);

    Ok(())
}

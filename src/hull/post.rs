use super::config::Config;
use super::minify;
use super::template;
use pulldown_cmark;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};
use tera::Context as TeraContext;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PostFrontMatter {
    pub author: String,
    pub author_email: String,
    pub author_twitter: String,
    pub author_uri: String,
    pub description: String,
    pub keywords: String,
    pub published_at: String,
    pub slug: String,
    pub title: String,
    pub updated_at: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Post {
    pub content_html: String,
    pub data: PostFrontMatter,
}

//#[serde(default = "default_fm_str")]
//fn default_fm_str() -> String {
//    String::new()
//}

pub fn create_all(hull_opts: &Config) -> Result<Vec<Post>, io::Error> {
    let posts = load(&hull_opts)?;
    setup(&hull_opts)?;
    create_index(&hull_opts, &posts)?;
    create_posts(&hull_opts, &posts)?;

    Ok(posts)
}

pub fn setup(hull_opts: &Config) -> Result<(), io::Error> {
    let dir = &hull_opts.posts.output;
    let path = Path::new(dir);

    if path.exists() {
        fs::remove_dir_all(path).expect("Hull: failed to remove posts output");
        println!("Hull: removed {:#?}", path);
    }

    fs::create_dir(path).expect("Hull: failed to create posts output");
    println!("Hull: created {:#?}", path);

    Ok(())
}

pub fn create_index(hull_opts: &Config, posts: &Vec<Post>) -> Result<(), io::Error> {
    let out_dir = &hull_opts.posts.output;
    let out_path = Path::new(out_dir).join("index.html");

    let mut ctx = TeraContext::new();
    ctx.insert("posts", &posts);

    let html = template::render("index.html", &ctx).and_then(minify::html)?;

    fs::write(&out_path, &html).expect(&format!("Hull: failed to write {:#?}", out_path));
    println!("Hull: wrote {:#?}", out_path);

    Ok(())
}

pub fn create_posts(hull_opts: &Config, posts: &Vec<Post>) -> Result<(), io::Error> {
    let domain = &hull_opts.site.url;
    let out_dir = &hull_opts.posts.output;
    let path = &hull_opts.posts.meta.path;

    for post in posts {
        let out_path = Path::new(&out_dir).join(format!("{}.html", post.data.slug));
        let url = format!("{}/{}/{}.html", domain, path, post.data.slug);

        let mut ctx = TeraContext::new();
        ctx.insert("author", &post.data.author);
        ctx.insert("author_twitter", &post.data.author_twitter);
        ctx.insert("content_html", &post.content_html);
        ctx.insert("description", &post.data.description);
        ctx.insert("keywords", &post.data.keywords);
        ctx.insert("site", &hull_opts.posts.meta.title);
        ctx.insert("title", &post.data.title);
        ctx.insert("published_at", &post.data.published_at);
        ctx.insert("updated_at", &post.data.updated_at);
        ctx.insert("url", &url);

        let html = template::render("post.html", &ctx).and_then(minify::html)?;

        fs::write(&out_path, &html).expect(&format!("Hull: failed to write {:#?}", out_path));
        println!("Hull: wrote {:#?}", out_path);
    }

    Ok(())
}

pub fn load(hull_opts: &Config) -> Result<Vec<Post>, io::Error> {
    let src = &hull_opts.posts.source;
    let md_opts = get_md_opts();
    let path = Path::new(src);
    let mut posts: Vec<Post> = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension().unwrap() != "md" {
            continue;
        }

        let post = parse(&path, md_opts).expect(&format!("Hull: failed to get post: {:#?}", path));

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

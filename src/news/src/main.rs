use handlebars::Handlebars;
use pulldown_cmark;
use serde::{Deserialize, Serialize};
//use std::env;
use std::{fs, io};
use std::{
    path::{Path, PathBuf},
    process::exit,
};
use toml;

#[derive(Debug, Default, Deserialize, Serialize)]
struct Page {
    author: String,
    description: String,
    content_html: String,
    keywords: String,
    og_image_alt: String,
    og_image_src: String,
    published_at: String,
    site: String,
    title: String,
    twitter_author: String,
    twitter_image_alt: String,
    twitter_image_src: String,
    page_type: String,
    updated_at: String,
    url: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct FrontMatter {
    description: String,
    #[serde(default = "default_fm_str")]
    keywords: String,
    published_at: String,
    slug: String,
    title: String,
    #[serde(default = "default_fm_str")]
    updated_at: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Post {
    content_html: String,
    data: FrontMatter,
}

fn default_fm_str() -> String {
    String::new()
}

fn main() -> io::Result<()> {
    // Arguments
    //let args: Vec<String> = env::args().collect();
    //let dir = &args[1];

    let out_path = Path::new("./web/news/");

    // Reset output dir

    if out_path.exists() {
        match fs::remove_dir_all(out_path) {
            Ok(_) => println!("Removed {:#?}", out_path),
            Err(_) => {
                println!("Failed to remove {:#?}", out_path);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    match fs::create_dir(out_path) {
        Ok(_) => println!("Created {:#?}", out_path),
        Err(_) => {
            println!("Failed to create {:#?}", out_path);
            println!("Exiting...");
            exit(1)
        }
    };

    // Markdown

    let mut md_opts = pulldown_cmark::Options::empty();
    md_opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);

    // Load template files

    let t_dir = Path::new("./src/news/src/templates");
    let template_default = fs::read_to_string(t_dir.join("default.hbs"))?;
    let template_index = fs::read_to_string(t_dir.join("index.hbs"))?;
    let template_post = fs::read_to_string(t_dir.join("post.hbs"))?;

    // Handlebars opts

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_template_string("t_default", template_default)
        .unwrap();
    handlebars
        .register_template_string("t_index", template_index)
        .unwrap();
    handlebars
        .register_template_string("t_post", template_post)
        .unwrap();

    // Load Posts

    let mut posts: Vec<Post> = vec![];

    for entry in fs::read_dir("./src/news/src/posts")? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension().unwrap() != "md" {
            continue;
        }

        match parse_post(&path, md_opts) {
            Ok(post) => posts.push(post),
            Err(_) => println!("Failed to get post: {:#?}", path),
        };
    }

    // Build index page

    let news_html = handlebars
        .render("t_index", &posts)
        .unwrap_or_else(|err| err.to_string());

    let index_page = Page {
        author: "Robert W. Pearce".to_string(),
        content_html: news_html,
        description: "News and updates about Ramda Guide".to_string(),
        keywords: "ramda, ramda guide, javascript, rss, atom, feed, news, functional programming"
            .to_string(),
        site: "Ramda Guide".to_string(),
        title: "Ramda Guide - News".to_string(),
        twitter_author: "Robert W. Pearce".to_string(),
        updated_at: "2021-02-05T12:00:00Z".to_string(),
        url: "https://ramda.guide/news/index.html".to_string(),
        ..Default::default()
    };

    let news_page_html = handlebars
        .render("t_default", &index_page)
        .unwrap_or_else(|err| err.to_string());

    let index_path = out_path.join("index.html");

    match fs::write(&index_path, &news_page_html) {
        Ok(_) => println!("Wrote {:#?}", index_path),
        Err(_) => {
            println!("Failed to write {:#?}", index_path);
            println!("Exiting...");
            exit(1)
        }
    };

    // Build post pages

    for post in posts {
        let post_html = handlebars
            .render("t_post", &post)
            .unwrap_or_else(|err| err.to_string());

        let post_page = Page {
            author: "Robert W. Pearce".to_string(),
            content_html: post_html,
            description: post.data.description,
            page_type: "article".to_string(),
            site: "Ramda Guide".to_string(),
            title: post.data.title,
            twitter_author: "Robert W. Pearce".to_string(),
            updated_at: "2021-02-05T12:00:00Z".to_string(),
            url: format!("https://ramda.guide/news/{}.html", post.data.slug),
            ..Default::default()
        };

        let post_page_html = handlebars
            .render("t_default", &post_page)
            .unwrap_or_else(|err| err.to_string());

        let post_path = out_path.join(format!("{}.html", post.data.slug));

        match fs::write(&post_path, &post_page_html) {
            Ok(_) => println!("Wrote {:#?}", post_path),
            Err(_) => {
                println!("Failed to write {:#?}", post_path);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    // TODO: RSS & Atom feeds

    Ok(())
}

fn parse_post(path: &PathBuf, md_opts: pulldown_cmark::Options) -> Result<Post, io::Error> {
    let file_content = fs::read_to_string(path)?;
    let (front_matter, content) = parse_front_matter(file_content)?;
    let content_html = parse_markdown(&content, md_opts);

    let post = Post {
        content_html,
        data: front_matter,
    };

    Ok(post)
}

fn parse_front_matter(file_content: String) -> Result<(FrontMatter, String), io::Error> {
    let split: Vec<&str> = file_content
        .split("+++")
        .filter(|&x| x != "")
        .map(|x| x.trim())
        .collect();

    let front_matter: FrontMatter = toml::from_str(split[0])?;
    let content = split[1];

    Ok((front_matter, content.to_string()))
}

fn parse_markdown(content: &str, md_opts: pulldown_cmark::Options) -> String {
    let md_parser = pulldown_cmark::Parser::new_ext(content, md_opts);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, md_parser);

    html
}

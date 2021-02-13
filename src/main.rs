use handlebars::Handlebars;
use mdbook::config::Config as MDBookConfig;
use mdbook::MDBook;
use pulldown_cmark;
use serde::{Deserialize, Serialize};
//use std::env;
use std::{fs, io, str::FromStr};
use std::{
    path::{Path, PathBuf},
    process::exit,
};
use toml;

#[derive(Debug, Deserialize)]
struct HullConfigPost {
    source: String,
    output: String,
}

#[derive(Debug, Deserialize)]
struct HullConfigBasic {
    enabled: bool,
    output: String,
}

#[derive(Debug, Deserialize)]
struct HullConfigFeeds {
    atom: HullConfigBasic,
    json: HullConfigBasic,
    rss: HullConfigBasic,
}

#[derive(Debug, Deserialize)]
struct HullConfig {
    feeds: HullConfigFeeds,
    posts: HullConfigPost,
    sitemap: HullConfigBasic,
}

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
    // Book

    match MDBook::load("./src/book") {
        Ok(mdbook) => match mdbook.build() {
            Ok(_) => println!("Wrote {:#?}", "./web/book"),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to create book");
                println!("Exiting...");
                exit(1)
            }
        },
        Err(err) => {
            println!("{:#?}", err);
            println!("Failed to load book data");
            println!("Exiting...");
            exit(1)
        }
    }

    // Load config

    let hull_opts: HullConfig = match fs::read_to_string("./hull.toml") {
        Ok(content) => toml::from_str(content.as_str())?,
        Err(err) => {
            println!("{:#?}", err);
            println!("Failed to read hull.toml");
            println!("Exiting...");
            exit(1)
        }
    };

    // Establish paths

    let posts_source = Path::new(hull_opts.posts.source.as_str());
    let posts_output = Path::new(hull_opts.posts.output.as_str());
    let sitemap_output = Path::new(hull_opts.sitemap.output.as_str());
    let feed_atom_output = Path::new(hull_opts.feeds.atom.output.as_str());
    let feed_json_output = Path::new(hull_opts.feeds.json.output.as_str());
    let feed_rss_output = Path::new(hull_opts.feeds.rss.output.as_str());

    // Setup Markdown

    let md_opts = get_md_opts();

    // TODO: Do I really need handlebars? Just use String?
    //       * https://www.steadylearner.com/blog/read/How-to-automate-building-sitemaps-with-Rust
    //       * https://docs.rs/mdbook/0.4.6/mdbook/

    // Handlebars templates

    let t_dir = Path::new("./src/templates");
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

    // Recreate posts output dir

    if posts_output.exists() {
        match fs::remove_dir_all(posts_output) {
            Ok(_) => println!("Removed {:#?}", posts_output),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to remove {:#?}", posts_output);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    match fs::create_dir(posts_output) {
        Ok(_) => println!("Created {:#?}", posts_output),
        Err(err) => {
            println!("{:#?}", err);
            println!("Failed to create {:#?}", posts_output);
            println!("Exiting...");
            exit(1)
        }
    };

    // Remove sitemap if enabled

    if hull_opts.sitemap.enabled && sitemap_output.exists() {
        match fs::remove_file(sitemap_output) {
            Ok(_) => println!("Removed {:#?}", sitemap_output),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to remove {:#?}", sitemap_output);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    // Remove feeds if each is enabled

    if hull_opts.feeds.atom.enabled && feed_atom_output.exists() {
        match fs::remove_file(feed_atom_output) {
            Ok(_) => println!("Removed {:#?}", feed_atom_output),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to remove {:#?}", feed_atom_output);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    if hull_opts.feeds.json.enabled && feed_json_output.exists() {
        match fs::remove_file(feed_json_output) {
            Ok(_) => println!("Removed {:#?}", feed_json_output),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to remove {:#?}", feed_json_output);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    if hull_opts.feeds.rss.enabled && feed_rss_output.exists() {
        match fs::remove_file(feed_rss_output) {
            Ok(_) => println!("Removed {:#?}", feed_rss_output),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to remove {:#?}", feed_rss_output);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    // Load Posts

    let mut posts: Vec<Post> = vec![];

    for entry in fs::read_dir(posts_source)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension().unwrap() != "md" {
            continue;
        }

        match parse_post(&path, md_opts) {
            Ok(post) => posts.push(post),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to get post: {:#?}", path)
            }
        };
    }

    // Build index page

    let news_html = handlebars
        .render("t_index", &posts)
        .unwrap_or_else(|err| err.to_string());

    // TODO: get data from somewhere so this is generic
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

    let index_path = posts_output.join("index.html");

    match fs::write(&index_path, &news_page_html) {
        Ok(_) => println!("Wrote {:#?}", index_path),
        Err(err) => {
            println!("{:#?}", err);
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

        let post_path = posts_output.join(format!("{}.html", post.data.slug));

        match fs::write(&post_path, &post_page_html) {
            Ok(_) => println!("Wrote {:#?}", post_path),
            Err(err) => {
                println!("{:#?}", err);
                println!("Failed to write {:#?}", post_path);
                println!("Exiting...");
                exit(1)
            }
        };
    }

    // TODO sitemap
    // TODO rss
    // TODO atom
    // TODO json

    Ok(())
}

fn get_md_opts() -> pulldown_cmark::Options {
    let mut md_opts = pulldown_cmark::Options::empty();
    md_opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    md_opts
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

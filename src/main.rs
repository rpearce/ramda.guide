use chrono;
use handlebars::Handlebars;
use pulldown_cmark;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use std::{path::Path, process::exit};

mod book;
mod config;
mod feed;
mod posts;
mod sitemap;
use posts::Post;
use sitemap::HullSitemapEntry;

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

fn main() -> io::Result<()> {
    // Book
    let book = book::load("./src/book")?;

    // Load config
    let hull_opts = config::load("./hull.toml")?;

    // Establish paths

    let posts_source = Path::new(hull_opts.posts.source.as_str());
    let posts_output = Path::new(hull_opts.posts.output.as_str());
    //let sitemap_output = Path::new(hull_opts.sitemap.output.as_str());
    let feed_output = Path::new(hull_opts.feed.output.as_str());

    // Setup Markdown

    let md_opts = get_md_opts();

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
    posts::setup(&hull_opts.posts.output)?;

    // Recreate sitemap output file
    sitemap::clear(&hull_opts.sitemap.output, hull_opts.sitemap.enabled)?;

    // Recreate feed output file
    feed::clear(&hull_opts.feed.output, hull_opts.feed.enabled)?;

    // Load Posts

    let mut posts: Vec<Post> = vec![];

    for entry in fs::read_dir(posts_source)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension().unwrap() != "md" {
            continue;
        }

        match posts::parse(&path, md_opts) {
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

    fs::write(&index_path, &news_page_html).expect("Failed to write posts index file");
    println!("Wrote {:#?}...", index_path);

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

    // Generate sitemap.xml

    let now = chrono::prelude::Utc::now().to_string();
    let mut sitemap_entries: Vec<HullSitemapEntry> = vec![];

    sitemap_entries.push(HullSitemapEntry {
        loc: "https://ramda.guide".to_string(),
        lastmod: now.clone(),
        changefreq: "weekly".to_string(),
        priority: "1.0".to_string(),
    });

    book.iter()
        .filter_map(|x| match *x {
            mdbook::book::BookItem::Chapter(ref chapter) => chapter.path.clone(),
            mdbook::book::BookItem::Separator => None,
            mdbook::book::BookItem::PartTitle(_) => None,
        })
        .for_each(|x| {
            let stem = x.file_stem().unwrap().to_str().unwrap().to_string();
            let loc = format!("https://ramda.guide/book/{}.html", stem);

            sitemap_entries.push(HullSitemapEntry {
                loc,
                lastmod: now.clone(),
                changefreq: "weekly".to_string(),
                priority: "0.8".to_string(),
            });
        });

    // TODO: add news entries to sitemap, too

    let sitemap_xml: String = sitemap::build(&sitemap_entries);
    // TODO: save sitemap.xml

    println!("{:#?}", sitemap_xml);

    // TODO feed

    Ok(())
}

fn get_md_opts() -> pulldown_cmark::Options {
    let mut md_opts = pulldown_cmark::Options::empty();
    md_opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    md_opts
}

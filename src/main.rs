use chrono;
use handlebars::Handlebars;
use std::path::Path;
use std::{fs, io};

mod book;
mod hull;
use hull::pages::Page as HullPage;
use hull::sitemap::Entry as HullSitemapEntry;

fn main() -> io::Result<()> {
    // Book

    let book = book::load("./src/book")?;

    // Load config

    let hull_opts = hull::config::load("./hull.toml")?;

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

    hull::posts::setup(&hull_opts.posts.output)?;

    // Recreate sitemap output file

    if hull_opts.sitemap.enabled {
        hull::sitemap::clear(&hull_opts.sitemap.output)?;
    }

    // Recreate feed output file

    if hull_opts.feed.enabled {
        hull::feed::clear(&hull_opts.feed.output)?;
    }

    // Load Posts

    let posts = hull::posts::load(&hull_opts.posts.source)?;

    // Build index page

    let news_html = handlebars
        .render("t_index", &posts)
        .unwrap_or_else(|err| err.to_string());

    // TODO: get data from somewhere so this is generic
    let index_page = HullPage {
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

    let index_path = Path::new(&hull_opts.posts.source).join("index.html");

    fs::write(&index_path, &news_page_html).expect("Failed to write posts index file");
    println!("Wrote {:#?}...", index_path);

    // Build post pages

    for post in posts {
        let post_html = handlebars
            .render("t_post", &post)
            .unwrap_or_else(|err| err.to_string());

        let post_page = HullPage {
            author: post.data.author,
            content_html: post_html,
            description: post.data.description,
            page_type: "article".to_string(),
            site: hull_opts.posts.meta.title.to_string(),
            title: post.data.title,
            twitter_author: post.data.author_twitter.to_string(),
            updated_at: post.data.updated_at.to_string(),
            url: format!("{}/{}.html", hull_opts.posts.meta.url, post.data.slug),
            ..Default::default()
        };

        let post_page_html = handlebars
            .render("t_default", &post_page)
            .unwrap_or_else(|err| err.to_string());

        let post_path = Path::new(&hull_opts.posts.output).join(format!("{}.html", post.data.slug));
        fs::write(&post_path, &post_page_html).expect(&format!("Failed to write {:#?}", post_path));
        println!("Wrote {:#?}", post_path);
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

    let sitemap_xml: String = hull::sitemap::build(&sitemap_entries);
    // TODO: save sitemap.xml

    println!("{:#?}", sitemap_xml);

    // TODO feed

    Ok(())
}

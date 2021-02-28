use chrono;
use lazy_static::lazy_static;
use std::{fs, io, path::Path, process::exit};
use tera::{Context as TeraContext, Tera};

mod book;
mod hull;
use hull::sitemap::Entry as HullSitemapEntry;

lazy_static! {
    pub static ref HULL_TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Hull template parsing error(s): {}", e);
                exit(1)
            }
        }
    };
}

fn main() -> io::Result<()> {
    let now = chrono::prelude::Utc::now();

    // Book

    let book = book::load("./src/book")?;

    // Load config

    let hull_opts = hull::config::load("./hull.toml")?;

    // Recreate posts output dir

    hull::post::setup(&hull_opts.posts.output)?;

    // Recreate sitemap output file

    if hull_opts.sitemap.enabled {
        hull::sitemap::clear(&hull_opts.sitemap.output)?;
    }

    // Recreate feed output file

    if hull_opts.feed.enabled {
        hull::feed::clear(&hull_opts.feed.output)?;
    }

    // Load Posts

    let posts = hull::post::load(&hull_opts.posts.source)?;

    // Create Posts Index Page

    let mut news_idx_ctx = TeraContext::new();
    news_idx_ctx.insert("posts", &posts);

    let news_html = match HULL_TEMPLATES.render("index.html", &news_idx_ctx) {
        Ok(x) => x,
        Err(err) => {
            println!("Hull template error: {:#?}", err);
            exit(1)
        }
    };

    let news_html_min = hull::minify::html(news_html)?;

    let index_path = Path::new(&hull_opts.posts.output).join("index.html");

    fs::write(&index_path, &news_html_min).expect(&format!("Failed to write {:#?}", index_path));

    println!("Wrote {:#?}...", index_path);

    //// Build post pages

    //for post in posts {
    //    let post_html = handlebars
    //        .render("t_post", &post)
    //        .unwrap_or_else(|err| err.to_string());

    //    let post_page = HullPage {
    //        author: post.data.author,
    //        content_html: post_html,
    //        description: post.data.description,
    //        page_type: "article".to_string(),
    //        site: hull_opts.posts.meta.title.to_string(),
    //        title: post.data.title,
    //        twitter_author: post.data.author_twitter.to_string(),
    //        updated_at: post.data.updated_at.to_string(),
    //        url: format!("{}/{}.html", hull_opts.posts.meta.url, post.data.slug),
    //        ..Default::default()
    //    };

    //    let post_page_html = handlebars
    //        .render("t_default", &post_page)
    //        .unwrap_or_else(|err| err.to_string());

    //    let post_path = Path::new(&hull_opts.posts.output).join(format!("{}.html", post.data.slug));
    //    fs::write(&post_path, &post_page_html).expect(&format!("Failed to write {:#?}", post_path));
    //    println!("Wrote {:#?}", post_path);
    //}

    //// Generate sitemap.xml

    //let mut sitemap_entries: Vec<HullSitemapEntry> = vec![];

    //sitemap_entries.push(HullSitemapEntry {
    //    loc: "https://ramda.guide".to_string(),
    //    lastmod: now.clone(),
    //    changefreq: "weekly".to_string(),
    //    priority: "1.0".to_string(),
    //});

    //book.iter()
    //    .filter_map(|x| match *x {
    //        mdbook::book::BookItem::Chapter(ref chapter) => chapter.path.clone(),
    //        mdbook::book::BookItem::Separator => None,
    //        mdbook::book::BookItem::PartTitle(_) => None,
    //    })
    //    .for_each(|x| {
    //        let stem = x.file_stem().unwrap().to_str().unwrap().to_string();
    //        let loc = format!("https://ramda.guide/book/{}.html", stem);

    //        sitemap_entries.push(HullSitemapEntry {
    //            loc,
    //            lastmod: now.clone(),
    //            changefreq: "weekly".to_string(),
    //            priority: "0.8".to_string(),
    //        });
    //    });

    //// TODO: add news entries to sitemap, too

    //let sitemap_xml: String = hull::sitemap::build(&sitemap_entries);
    //// TODO: save sitemap.xml

    //println!("{:#?}", sitemap_xml);

    //// TODO feed

    Ok(())
}

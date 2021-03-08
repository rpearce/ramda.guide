use chrono;
use mdbook::book::BookItem;
use std::io;

mod book;
mod hull;
use hull::sitemap::Entry as HullSitemapEntry;

fn main() -> io::Result<()> {
    let now = chrono::prelude::Utc::now();
    let book = book::load("./src/book")?;
    let hull_opts = hull::config::load("./hull.toml")?;

    // Recreate posts output dir

    hull::post::setup(&hull_opts)?;

    // Remove sitemap output file

    if hull_opts.sitemap.enabled {
        hull::sitemap::remove(&hull_opts)?;
    }

    // Remove feed output file

    if hull_opts.feed.enabled {
        hull::feed::remove(&hull_opts)?;
    }

    // Load posts

    let posts = hull::post::load(&hull_opts)?;

    // Create posts index page

    hull::post::create_index(&hull_opts, &posts)?;

    // Build post pages

    hull::post::create_posts(&hull_opts, &posts)?;

    // Generate sitemap

    let mut sitemap_entries = vec![HullSitemapEntry {
        loc: hull_opts.site.url.clone(),
        lastmod: now.to_string(),
        changefreq: "weekly".to_string(),
        priority: "0.6".to_string(),
    }];

    let book_entries: Vec<HullSitemapEntry> = book
        .iter()
        .filter_map(|x| match *x {
            BookItem::Chapter(ref chapter) => chapter.path.clone(),
            BookItem::Separator => None,
            BookItem::PartTitle(_) => None,
        })
        .map(|x| HullSitemapEntry {
            loc: format!(
                "{}/book/{}.html",
                &hull_opts.site.url,
                x.file_stem().unwrap().to_str().unwrap().to_string()
            ),
            lastmod: now.to_string(),
            changefreq: "daily".to_string(),
            priority: "0.9".to_string(),
        })
        .collect();

    let post_entries: Vec<HullSitemapEntry> = posts
        .iter()
        .map(|x| hull::sitemap::entry_from_post(&hull_opts, &x))
        .collect();

    sitemap_entries.extend(book_entries);
    sitemap_entries.extend(post_entries);

    hull::sitemap::create_sitemap(&hull_opts, &sitemap_entries)?;

    // TODO feed

    Ok(())
}

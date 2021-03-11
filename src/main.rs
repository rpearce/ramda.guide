use chrono;
use std::io;

mod book;
mod hull;
use hull::sitemap::Entry as HullSitemapEntry;

fn main() -> io::Result<()> {
    let now = chrono::prelude::Utc::now();
    let book_data = book::load("./src/book")?;
    let hull_opts = hull::config::load("./hull.toml")?;

    // Load posts

    let posts = hull::post::load(&hull_opts)?;

    // Recreate posts output dir

    hull::post::setup(&hull_opts)?;

    // Create posts index page

    hull::post::create_index(&hull_opts, &posts)?;

    // Build post pages

    hull::post::create_posts(&hull_opts, &posts)?;

    // Generate sitemap

    if hull_opts.sitemap.enabled {
        hull::sitemap::remove(&hull_opts)?;

        let mut sitemap_entries = vec![HullSitemapEntry {
            loc: hull_opts.site.url.clone(),
            lastmod: now.to_string(),
            changefreq: "weekly".to_string(),
            priority: "0.6".to_string(),
        }];

        let sitemap_book_entries: Vec<HullSitemapEntry> = book::get_chapter_paths(book_data)
            .iter()
            .map(|x| book::to_sitemap_entry(&hull_opts, x, now.to_string()))
            .collect();

        let sitemap_post_entries: Vec<HullSitemapEntry> = posts
            .iter()
            .map(|x| hull::sitemap::entry_from_post(&hull_opts, &x))
            .collect();

        sitemap_entries.extend(sitemap_book_entries);
        sitemap_entries.extend(sitemap_post_entries);

        hull::sitemap::create(&hull_opts, &sitemap_entries)?;
    }

    // Generate feed

    if hull_opts.feed.enabled {
        hull::feed::remove(&hull_opts)?;

        hull::feed::create(&hull_opts, &posts)?;
    }

    Ok(())
}

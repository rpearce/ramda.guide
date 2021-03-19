use chrono;
use std::io;

mod book;
mod hull;
use hull::sitemap::Entry as HullSitemapEntry;

fn main() -> io::Result<()> {
    let now = chrono::prelude::Utc::now();
    let book_data = book::load("./src/book")?;
    let hull_opts = hull::config::load("./hull.toml")?;

    // Load posts and create output

    let posts = hull::post::create_all(&hull_opts)?;

    // Generate sitemap

    if hull_opts.sitemap.enabled {
        let mut entries = vec![HullSitemapEntry {
            loc: hull_opts.site.url.clone(),
            lastmod: now.to_string(),
            changefreq: "weekly".to_string(),
            priority: "0.6".to_string(),
        }];

        let book_entries: Vec<HullSitemapEntry> = book::get_chapter_paths(book_data)
            .iter()
            .map(|x| book::to_sitemap_entry(&hull_opts, x, now.to_string()))
            .collect();

        let post_entries: Vec<HullSitemapEntry> = posts
            .iter()
            .map(|x| hull::sitemap::entry_from_post(&hull_opts, &x))
            .collect();

        entries.extend(book_entries);
        entries.extend(post_entries);

        hull::sitemap::recreate(&hull_opts, &entries)?;
    }

    // Generate feed

    if hull_opts.feed.enabled {
        hull::feed::recreate(&hull_opts, &posts)?;
    }

    Ok(())
}

use chrono;
use std::{fs, io, path::Path, process::exit};

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

    // Load Posts

    let posts = hull::post::load(&hull_opts)?;

    // Create Posts Index Page

    hull::post::create_index(&hull_opts, &posts)?;

    // Build post pages

    hull::post::create_posts(&hull_opts, &posts)?;

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

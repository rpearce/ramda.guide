use mdbook::{book::BookItem, MDBook};
use std::{io, path::PathBuf};

use super::hull::{config::Config as HullConfig, sitemap::Entry as HullSitemapEntry};

pub fn load(src: &str) -> Result<MDBook, io::Error> {
    let book = MDBook::load(src).expect("Failed to load book data");
    book.build().expect("Failed to create book");
    println!("Wrote {:#?}", "./web/book");

    Ok(book)
}

pub fn get_chapter_paths(book: MDBook) -> Vec<PathBuf> {
    book.iter()
        .filter_map(|x| match *x {
            BookItem::Chapter(ref chapter) => chapter.path.clone(),
            BookItem::Separator => None,
            BookItem::PartTitle(_) => None,
        })
        .collect()
}

pub fn to_sitemap_entry(
    hull_opts: &HullConfig,
    chapter_path: &PathBuf,
    now: String,
) -> HullSitemapEntry {
    let chapter_path_string = chapter_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let loc = format!("{}/book/{}.html", &hull_opts.site.url, chapter_path_string);

    HullSitemapEntry {
        loc,
        lastmod: now.to_string(),
        changefreq: "daily".to_string(),
        priority: "0.9".to_string(),
    }
}

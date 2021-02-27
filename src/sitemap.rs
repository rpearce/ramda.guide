use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Default)]
pub struct HullSitemapEntry {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: String,
}

pub fn build(entries: &Vec<HullSitemapEntry>) -> String {
    let items: String = entries.iter().map(to_entry).collect();

    to_sitemap(items)
}

fn to_sitemap(content: String) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  {}
</urlset>
"#,
        content
    )
}

fn to_entry(entry: &HullSitemapEntry) -> String {
    format!(
        r#"
<url>
  <loc>{}</loc>
  <lastmod>{}<lastmod>
  <changefreq>{}</changefreq>
  <priority>{}</priority>
</url>
"#,
        entry.loc, entry.lastmod, entry.changefreq, entry.priority
    )
}

pub fn clear(src: &str) -> Result<(), io::Error> {
    let path = Path::new(src);

    if path.exists() {
        fs::remove_file(path).expect("Failed to remove sitemap output");
        println!("Removed... {:#?}", path);
    }

    Ok(())
}

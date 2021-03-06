use super::config::HullConfig;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, Default)]
pub struct Entry {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: String,
}

pub fn build(entries: &Vec<Entry>) -> String {
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

fn to_entry(entry: &Entry) -> String {
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

pub fn remove(hull_opts: &HullConfig) -> Result<(), io::Error> {
    let src = &hull_opts.sitemap.output;
    let path = Path::new(src);

    if path.exists() {
        fs::remove_file(path).expect("Hull: failed to remove sitemap output");
        println!("Hull: removed {:#?}", path);
    }

    Ok(())
}

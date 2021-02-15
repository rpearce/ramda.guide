fn main() -> io::Result<()> {
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

    let sitemap_book_items: String = sitemap_entries.iter().map(build_entry).collect();

    let sitemap_xml = build_sitemap(sitemap_book_items);
    println!("{:#?}", sitemap_xml);
    //let sitemap_post_items: String =
}

fn build_sitemap(content: String) -> String {
    let template = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  {}
</urlset>
"#;

    format!(template, content)
}

fn build_entry(entry: HullSitemapEntry) -> String {
    let template = r#"
<url>
  <loc>{}</loc>
  <lastmod>{}<lastmod>
  <changefreq>{}</changefreq>
  <priority>{}</priority>
</url>
"#;
    format!(
        template,
        entry.loc, entry.lastmod, entry.changefreq, entry.priority
    )
}

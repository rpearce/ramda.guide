use super::{config::Config, post::Post, template};
use serde::Serialize;
use std::path::Path;
use std::{fs, io};
use tera::Context as TeraContext;

#[derive(Debug, Default, Serialize)]
pub struct Entry {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: String,
}

pub fn create(hull_opts: &Config, entries: &Vec<Entry>) -> Result<(), io::Error> {
    let src = &hull_opts.sitemap.output;
    let path = Path::new(src);

    let mut ctx = TeraContext::new();
    ctx.insert("entries", &entries);

    let xml = template::render("sitemap.xml", &ctx)?;

    fs::write(&path, &xml).expect(&format!("Hull: failed to write {:#?}", path));
    println!("Hull: wrote {:#?}", path);

    Ok(())
}

pub fn remove(hull_opts: &Config) -> Result<(), io::Error> {
    let src = &hull_opts.sitemap.output;
    let path = Path::new(src);

    if path.exists() {
        fs::remove_file(path).expect("Hull: failed to remove sitemap output");
        println!("Hull: removed {:#?}", path);
    }

    Ok(())
}

pub fn entry_from_post(hull_opts: &Config, post: &Post) -> Entry {
    let domain = &hull_opts.site.url;
    let path = &hull_opts.posts.meta.path;
    let loc = format!("{}/{}/{}.html", domain, path, post.data.slug);

    let lastmod = if post.data.published_at.is_empty() {
        post.data.updated_at.clone()
    } else {
        post.data.published_at.clone()
    };

    Entry {
        loc,
        lastmod,
        changefreq: hull_opts.sitemap.posts.changefreq.clone(),
        priority: hull_opts.sitemap.posts.priority.clone(),
    }
}

use super::config::Config;
use super::post::Post;
use super::template;
use serde::Serialize;
use std::path::Path;
use std::{fs, io};
use tera::Context as TeraContext;

#[derive(Debug)]
pub struct Feed {
    pub domain: String,
    pub entries: Entry,
    pub last_updated_at: String,
    pub rights: String,
    pub site: String,
    pub start_year: String,
    pub subtitle: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub author_email: String,
    pub author_name: String,
    pub author_uri: String,
    pub content_html: String,
    pub link: String,
    pub published_at: String,
    pub published_at_year: String,
    pub title: String,
    pub updated_at: String,
    pub uri: String,
}

pub fn recreate(hull_opts: &Config, posts: &Vec<Post>) -> Result<(), io::Error> {
    remove(&hull_opts)?;
    create(&hull_opts, &posts)?;

    Ok(())
}

pub fn create(hull_opts: &Config, posts: &Vec<Post>) -> Result<(), io::Error> {
    let out_path = Path::new(&hull_opts.feed.output);
    let path = &hull_opts.posts.meta.path;
    let entries: Vec<Entry> = posts
        .iter()
        .map(|x| {
            let file_name = format!("{}.html", x.data.slug);
            let uri = format!("{}/{}", path, file_name);
            let link = format!("{}/{}", hull_opts.site.url, uri);
            let updated_at = if x.data.updated_at.is_empty() {
                x.data.published_at.clone()
            } else {
                x.data.updated_at.clone()
            };

            Entry {
                author_email: x.data.author_email.clone(),
                author_name: x.data.author.clone(),
                author_uri: x.data.author_uri.clone(),
                content_html: x.content_html.clone(),
                link,
                published_at: x.data.published_at.clone(),
                published_at_year: x.data.published_at.clone().chars().take(4).collect(),
                title: x.data.title.clone(),
                updated_at,
                uri,
            }
        })
        .rev()
        .collect();

    let start_year = entries
        .iter()
        .min_by_key(|x| x.published_at_year.clone())
        .unwrap()
        .published_at_year
        .clone();

    let last_updated_at = entries
        .iter()
        .max_by_key(|x| x.updated_at.clone())
        .unwrap()
        .updated_at
        .clone();

    let mut ctx = TeraContext::new();
    ctx.insert("site", &hull_opts.site.url);
    ctx.insert("title", &hull_opts.feed.meta.title);
    ctx.insert("subtitle", &hull_opts.feed.meta.subtitle);
    ctx.insert("rights", &hull_opts.feed.meta.rights);
    ctx.insert("domain", &hull_opts.feed.meta.domain);
    ctx.insert("start_year", &start_year);
    ctx.insert("last_updated_at", &last_updated_at);
    ctx.insert("entries", &entries);

    let atom = template::render("feed.atom", &ctx)?;

    fs::write(&out_path, &atom).expect(&format!("Hull: failed to write {:#?}", path));
    println!("Hull: wrote {:#?}", out_path);

    Ok(())
}

pub fn remove(hull_opts: &Config) -> Result<(), io::Error> {
    let src = &hull_opts.feed.output;
    let path = Path::new(src);

    if path.exists() {
        fs::remove_file(path).expect("Hull: failed to remove feed output");
        println!("Hull: removed {:#?}", path);
    }

    Ok(())
}

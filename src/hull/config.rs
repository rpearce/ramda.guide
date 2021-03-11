use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize)]
pub struct ConfigPostMeta {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigPost {
    pub source: String,
    pub output: String,
    pub meta: ConfigPostMeta,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFeedMeta {
    pub title: String,
    pub subtitle: String,
    pub rights: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFeed {
    pub enabled: bool,
    pub output: String,
    pub meta: ConfigFeedMeta,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSite {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSitemapPosts {
    pub changefreq: String,
    pub priority: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSitemap {
    pub enabled: bool,
    pub output: String,
    pub posts: ConfigSitemapPosts,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub feed: ConfigFeed,
    pub posts: ConfigPost,
    pub site: ConfigSite,
    pub sitemap: ConfigSitemap,
}

pub fn load(src: &str) -> Result<Config, io::Error> {
    let cfg_content = fs::read_to_string(src).expect("Hull: failed to read hull config file");
    let cfg = toml::from_str(cfg_content.as_str()).expect("Hull: failed to parse hull config");

    Ok(cfg)
}

use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize)]
pub struct HullConfigMeta {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct HullConfigPost {
    pub source: String,
    pub output: String,
    pub meta: HullConfigMeta,
}

#[derive(Debug, Deserialize)]
pub struct HullConfigBasic {
    pub enabled: bool,
    pub output: String,
}

#[derive(Debug, Deserialize)]
pub struct HullConfig {
    pub feed: HullConfigBasic,
    pub posts: HullConfigPost,
    pub sitemap: HullConfigBasic,
}

pub fn load(src: &str) -> Result<HullConfig, io::Error> {
    let cfg_content = fs::read_to_string(src).expect("Failed to read hull config file");
    let cfg = toml::from_str(cfg_content.as_str()).expect("Failed to parse hull config");

    Ok(cfg)
}

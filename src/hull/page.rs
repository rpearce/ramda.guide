use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fs, io, process::exit};
use tera::{Context as TeraContext, Tera};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Page {
    pub author: String,
    pub description: String,
    pub content_html: String,
    pub keywords: String,
    pub og_image_alt: String,
    pub og_image_src: String,
    pub published_at: String,
    pub site: String,
    pub title: String,
    pub twitter_author: String,
    pub twitter_image_alt: String,
    pub twitter_image_src: String,
    pub page_type: String,
    pub updated_at: String,
    pub url: String,
}

lazy_static! {
    pub static ref HULL_TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Hull: template parsing error(s): {}", e);
                exit(1)
            }
        }
    };
}

pub fn build(template: &str, page: &Page) -> Result<String, tera::Error> {
    TeraContext::from_serialize(&page).and_then(|ctx| HULL_TEMPLATES.render(template, &ctx))
}

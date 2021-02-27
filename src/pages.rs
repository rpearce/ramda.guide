use serde::{Deserialize, Serialize};

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

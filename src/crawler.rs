use reqwest;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WebDocument {
    pub url: String;
    pub content: String;
    pub crawl_time: u64;
}

pub fn fetch_page(url: &str) -> WebDocument {
    let content = reqwest::blocking::get(url)
        .and_then(|resp|, resp.text())
        .unwrap_or_else(|_|, String::from(""));
}

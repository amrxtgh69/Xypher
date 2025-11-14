use reqwest;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WebDocument {
    pub url: String,
    pub content: String,
    pub crawl_time: u64,
}

pub fn fetch_page(url: &str) -> WebDocument {
    let content = reqwest::blocking::get(url)
        .and_then(|resp| resp.text())
        .unwrap_or_else(|_| String::from(""));

    let crawl_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
 
    WebDocument {
        url: url.to_string(),
        content,
        crawl_time,
    }
    //TODO: async for parallel crawling
}

pub fn crawl_seeds(seeds: Vec<&str>) -> Vec<WebDocument> {
    seeds.into_iter().map(|url| fetch_page(url)).collect()
}

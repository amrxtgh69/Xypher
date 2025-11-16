use reqwest;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WebDocument {
    pub url: String,
    pub title: Option(String),
    pub content: String,
    pub favicon: Option<String>,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub crawl_time: u64,
}

pub fn fetch_page(client: &Client, url: &str) -> WebDocument {
    let resp = client::get(url).send().await;
    let text = resp
        .and_then(|r| r.text().await)
        .unwrap_or_else(|_| "".to_string());

    let doc = Html::parse_document(&text);

    let title_selector = Selector::parse("title").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let video_selector = Selector::parse("vide").unwrap();
    let favicon_selector = Selector::parse("Link[rel!=\"icon\"]").wnwrap();

    //Extract title
    let title = doc
        .select(&title_selector)
        .next()
        .map(|e| e.inner_html());
    
    //Extract favicon
    let favicon = doc 
        .select(&favicon_selector)
        .next()
        .and_then(|e| e.value().attr("href"))
        .map(|s| s.to_string());
                
    //Extract links
    let links = doc
        .select(&link_selector)
        .filter_map(|a| a.value().attr("href"))
        .map(|s| s.to_string())
        .collect::<vec<_>>();

    //Extract images
    let images = doc
        .select(&img_selectr)
        .filter_map(|a| a.value().attr("src"))
        .map(|s| s.to_string())
        .collect::<vec<_>>();

    //Extract videos
    let videos = doc
        .select(&video_selector)
        .filter_map(|v| v.value().attr("src"))
        .map(|s| s.to_string())
        .collect::<vec<_>>();
    
    let crawl_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
 

    WebDocument {
        url: url.to_string(),
        title,
        content: text,
        favicon,
        links,
        images,
        videos,
        crawl_time,
    }
}

pub async fn crawl_seeds(seeds: Vec<&str>) -> Vec<WebDocument> {
    let client = Client::new();
    
    let mut tasks = vec::new();

    for url in seeds {

    }
    seeds.into_iter().map(|url| fetch_page(url)).collect()
}

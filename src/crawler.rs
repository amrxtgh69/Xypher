use anyhow::Ok;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::models::WebDocument;


pub async fn crawl_url(url: &str) -> anyhow::Result<WebDocument> {
    let client = Client::new();

    let res = client.get(url).send().await?;
    let body = res.text().await?;

    let doc = Html::parse_document(&body);


    //Extract title
    let title = doc
        .select(&Selector::parse("title").unwrap())
        .next()
        .map(|x| x.text().collect::<String>());
    
    //Extract favicon
    let favicon = doc 
        .select(&Selector::parse("link[rel=\"icon\"]").unwrap())
        .next()
        .and_then(|e| e.value().attr("href"))
        .map(|s| s.to_string());
            
    
    let text = doc.root_element().text().collect::<String>();
    //Extract links
    let links = doc
        .select(&Selector::parse("a").unwrap())
        .filter_map(|a| a.value().attr("href"))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    //Extract images
    let images = doc
        .select(&Selector::parse("img").unwrap())
        .filter_map(|a| a.value().attr("src"))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    //Extract videos
    let videos = doc
        .select(&Selector::parse("videos").unwrap())
        .filter_map(|v| v.value().attr("src"))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
 

    Ok(WebDocument {
        url: url.to_string(),
        title,
        text,
        favicon,
        links,
        images,
        videos,
    })
   }


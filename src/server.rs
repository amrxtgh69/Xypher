use std::sync::Arc;
use axum::{routing::{post, get}, Json, Router, extract::State};
use tokio::sync::{Mutex};

use crate::{crawler::crawl_url, indexer::Indexer};
use crate::models::WebDocument;


pub async fn start_server() {
    let indexer = Arc::new(Mutex::new(Indexer::new()));
    
    let app = Router::new()
        .route("/crawl", post(crawl_handler))
        .route("/search", get(search_handler))
        .with_state(indexer);

    println!("server listening on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn crawl_handler(State(indexer): State<Arc<Mutex<Indexer>>>, Json(payload): Json<Vec<String>>) -> Json<Vec<WebDocument>> {
    let mut docs = Vec::new();
    
    for url in payload {
        if let Ok(doc) = crawl_url(&url).await {
            docs.push(doc.clone());
            indexer.lock().await.add(doc).unwrap();
        }
    }
    Json(docs)
}

async fn search_handler(State(_indexer): State<Arc<Mutex<Indexer>>>) -> Json<String> {
    Json("Search logic to do".to_string())
}
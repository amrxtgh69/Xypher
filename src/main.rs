mod crawler;
mod indexer;
mod seeds;

use axum::{Router, routing::get, extract::Query, response::Html};
use::std::sync::Arc;
use std::path::Path;
use tantivy::schema::*;
use tantivy::{Index};
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    index: Arc<Mutex<Index>>,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let index_path = Path::new("./search_index");

    //build schema
    let mut schema_builder = Schema::builder();
    let _url_field = schema_builder.add_text_field("url", STORED);
    let _content_field = schema_builder.add_text_field("content", TEXT | STORED);
    let schema = schema_builder.build();

    let index = if index_path.exists() {
        Index::open_in_dir(&index_path).unwrap()
    } else {
        std::fs::create_dir_all(index_path).unwrap();
        Index::create_in_dir(&index_path, schema.clone()).unwrap()
    };

    let shared_state = AppState {
        index: Arc::new(Mutex::new(index)),
    };
    println!("Starting crawl...");
    let seeds = seeds::get_seeds_urls();
    let documents = crawler::crawl_seeds(seeds).await;
    println!("Finished crawl, got {} numbers of documents", documents.len());
    for doc in documents {
        let mut ind_guard = shared_state.index.lock().await;
        let _ = indexer::index_document(&mut *ind_guard, &doc);
    }

    let app = Router::new()
        .route("/", get(serve_home))
        .route("/search", get(search_handler))
        .with_state(shared_state.clone());

    println!("Server listening on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

 async fn serve_home() -> Html<String> {
    Html(std::fs::read_to_string("src/templates/index.html").unwrap())
}

async fn search_handler(Query(params): Query<SearchQuery>, axum::extract::State(state): axum::extract::State<AppState>) -> Html<String> {
    let Some(query) = params.q else {
        return Html("<h3>Please enter the query</h3>".into());
    };

    let index = state.index.lock().await;
    let results = indexer::search_index(&index, &query).unwrap_or_else(|_| vec![]);
    
    let mut html = format!("<h2>Result for '{}'</h2>", query);
    for url in results {
        html.push_str(&format!("<p><a href='{}'>{}</a></p>", url, url));
    }

    Html(html)
}



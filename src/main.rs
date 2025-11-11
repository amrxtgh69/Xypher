mod crawler;
mod indexer;

use std::path::Path;
use tantivy::schema::*;
use tantivy::{Index};
use std::io::{self, Write};

fn main() -> tantivy::Result<()> {
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
    
    println!("Enter the seed-urls seperated by comma");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let seeds: Vec<String> = input.trim().split(',').map(|s| s.trim().to_string()).collect();

    for seed_url in seeds {
        let doc = crawler::fetch_page(&seed_url);
        indexer::index_document(&index, &doc)?;
    }

    loop {
        print!("Enter search keyword or enter 'exit'");
        io::stdout().flush().unwrap();
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let query = query.trim();
        if query == "exit" { break; };

        indexer::search_index(&index, query)?;
    }
    Ok(())
}

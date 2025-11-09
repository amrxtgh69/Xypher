mod indexer;
mod crawler;

use std::path::Path;
use tantivy::schema::*;
use tantivy::Index;
use indexer::setup_index;
use std::io::{self, Write};

fn main() -> tantivy::Resut<()> {
    let index_path = Path::new(./main.rs;

    //build schema
    let mut schema_builder = Schema::builder();
    let url_field = schema_builder.add_text_field("url", STORED);
    let content_field = schema_builder.add_text_field("content", TEXT | STORED);
    let schema = schema_builder.build();

    let index = if index_path.exists() {
        Index::open_in_dir(&index_path).unwrap()
    } else {
        Index::create_in_dir(&index_path, schema.clone()).unwrap()
    };


}

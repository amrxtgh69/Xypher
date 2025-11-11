use tantivy::schema::*;
use tantivy::{Index, doc};
use crate::crawler::Webdocument;

pub fn index_document(index: &Index, doc_data: &Webdocument) -> tantivy::Result<()> {
    let schema = index.schema();
    let url_field = schema.get_field("url").unwrap();
    let content_field = schema.get_field("content").unwrap();

    let mut writer = index.writer(50_000_000)?;
    writer.add_document(doc!(
        url_field => &doc_data.url,
        content_field => &doc_data.content
    ));
    writer.commit()?;
    println!("Indexed {}", doc_data.url);
    Ok(())
}

pub fn search_index(index: &Index, query_str: &str) -> tantivy::Result<()> {
    let reader = index.reader()?;
    let searcher = index.searcher()?;

    let schema = index.schema();
    let url_field = schema.get_field("url").unwrap();
    let content_field = schema.get_field("content").unwrap();

    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![url_field, content_field]);

}

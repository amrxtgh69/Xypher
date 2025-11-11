use tantivy::{Index, doc};
use tantivy::schema::Value;
use crate::crawler::WebDocument;

pub fn index_document(index: &Index, doc_data: &WebDocument) -> tantivy::Result<()> {
    let schema = index.schema();
    let url_field = schema.get_field("url").unwrap();
    let content_field = schema.get_field("content").unwrap();
    let mut writer = index.writer(50_000_000)?;
    writer.add_document(doc!(
        url_field => doc_data.url.clone(),
        content_field => doc_data.content.clone()
    ))?;
    writer.commit()?;
    println!("Indexed {}", doc_data.url);
    Ok(())
}

pub fn search_index(index: &Index, query_str: &str) -> tantivy::Result<()> {
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    let url_field = schema.get_field("url").unwrap();
    let content_field = schema.get_field("content").unwrap();
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![url_field, content_field]);
    let query = query_parser.parse_query(query_str)?;
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;
    
    println!("Top results");
    for (_score, doc_address) in top_docs {
        let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;
        if let Some(url_value) = retrieved_doc.get_first(url_field) {
            let url_text = url_value.as_str().unwrap_or("N/A");
            println!("Found url: {}", url_text);
        }
    }
    Ok(())
}

use tantivy::{Index, doc};
use tantivy::schema::Value;
use crate::crawler::WebDocument;
use tantivy::Document;

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

pub fn index_documents(index: &Index, docs:Vec<WebDocument>) -> tantivy::Result<()> {
    for doc in docs {
        index_document(index, &doc)?;
    }
    Ok(())
}

pub fn search_index(index: &Index, query_str: &str) -> tantivy::Result<Vec<String>> {
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let schema = index.schema();
    let url_field = schema.get_field("url").unwrap();
    let content_field = schema.get_field("content").unwrap();
    
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![url_field, content_field]);
    let query = query_parser.parse_query(query_str)?;
    
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;
    
    let mut results = vec![];
    for (_score, doc_address) in top_docs {
        let retrieved_doc: Document = searcher.doc(doc_address)?;
        if let Some(url_val) = retrieved_doc.get_first(url_field) {
            results.push(url_val.text().unwrap().to_string());
        }
    }
    Ok(results)
}

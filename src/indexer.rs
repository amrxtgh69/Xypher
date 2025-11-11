use tantivy::schema::*;
use tantivy::{Index, doc};
use crate::crawler::Webdocument;

pub fn index_document(index: &Index, doc_data: &Webdocument) -> tantivy::Result<()> {
    let schema = index.schema();
    let url_field = schema.get_field("url").unwrap();

    let mut writer = index.writer(50_000_000)?;
    writer.add_document(doc!(
        url_field => &doc_data.url,
        content_field => &doc_data.content
    ));
    writer.commit()?;
    println!("Indexed {}", doc_data.url);


    Ok(())
}



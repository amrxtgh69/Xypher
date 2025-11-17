use anyhow::Result;
use tantivy::{
    Index, schema::{STORED, STRING, Schema, TEXT}
};
use crate::models::WebDocument;

pub struct Indexer {
    index: Index,
}

impl Indexer {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        let _url = schema_builder.add_text_field("url", STRING | STORED);

        let _title = schema_builder.add_text_field("title", TEXT | STORED);
        let _text = schema_builder.add_text_field("text", TEXT);
        let _favicon = schema_builder.add_text_field("favicon", STORED);

        let _images = schema_builder.add_text_field("images", STORED);
        let _videos = schema_builder.add_text_field("videos", STORED); 

        let schema = schema_builder.build();
        let index = Index::create_in_dir("./tantivy_index", schema.clone()).unwrap();

        Self { index }
    }

    pub fn add(&self, doc: WebDocument) -> Result<()> {
        let schema = self.index.schema();
        let mut writer = self.index.writer(50_000_000)?;

        let mut d = tantivy::doc!();
        d.add_text(schema.get_field("url").unwrap(), doc.url);

        if let Some(t) = doc.title { d.add_text(schema.get_field("title").unwrap(), t); }
        if let Some(f) = doc.favicon { d.add_text(schema.get_field("favicon").unwrap(), f); }
        
        d.add_text(schema.get_field("text").unwrap(), doc.text);

        d.add_text(schema.get_field("images").unwrap(), serde_json::to_string(&doc.images)?);
        d.add_text(schema.get_field("videos").unwrap(), serde_json::to_string(&doc.videos)?);

        let _ = writer.add_document(d);
        writer.commit()?;
        Ok(())
    }
}


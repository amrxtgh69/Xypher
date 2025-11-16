use anyhow::Result;
use tantivy::{
    Document, Index, schema::{STORED, STRING, Schema, SchemaBuilder, TEXT}
};
use crate::models::WebDocument;

pub struct Indexer {
    index: Index,
}

impl Indexer {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        let url = schema_builder.add_text_field("url", STRING | STORED);

        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let text = schema_builder.add_text_field("text", TEXT);
        let favicon = schema_builder.add_text_field("favicon", STORED);

        let images = schema_builder.add_text_field("images", STORED);
        let videos = schema_builder.add_text_field("videos", STORED); 

        let schema = schema_builder.build();
        let index = Index::create_in_dir("./tantivy_index", schema.clone()).unwrap();

        Self { index }
    }

    pub fn add(&self, doc: WebDocument) -> Result<()> {
        let schema = self.index.schema();
        let mut writer = self.index.writer(50_000_000)?;

        let mut d = Document::default();
        d.add_text(schema.get_field("url").unwrap(), doc.url);

        if let Some(t) = doc.title { d.add_text(schema.get_field("title").unwrap(), t); }
        if let Some(f) = doc.favicon { d.add_tex(schema.get_field("favicon").unwrap(), f); }
        
        d.add_text(schema.get_field("text").unwrap(), doc.text);

        d.add_text(schema.get_field("images").unwrap(), serde_json::to_string(&doc.images)?);
        d.add_text(schema.get_field("videos").unwrap(), serde_json::to_string(&doc.videos)?);

        writer.add_document(d);
        writer.commit()?;
        Ok(())
    }
}


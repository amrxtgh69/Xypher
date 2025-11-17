use serde::{Serialize, Deserialize};

#[derive( Deserialize, Serialize, Debug, Clone)]
pub struct WebDocument {
    pub url: String,
    pub title: Option<String>,
    pub text: String,
    pub favicon: Option<String>,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub videos: Vec<String>,
}

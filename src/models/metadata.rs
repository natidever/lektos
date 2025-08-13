use pyo3::prelude::*;

use crate::extractors::{common::CommonExtracor, ogp::OgpExtractor, schema::SchemaExtractor};

pub trait MetadataExtractor: Send + Sync {
    fn extract(&self, html: &str) -> ExtractionResult;
    fn priority(&self) -> u8;
    fn name(&self) -> &'static str;
}
#[derive(Debug, Default)]
#[pyclass]
pub struct ExtractionResult {
    #[pyo3(get)]
    pub title: Option<FieldResult>,
    #[pyo3(get)]
    pub author: Option<FieldResult>,
    #[pyo3(get)]
    pub description: Option<FieldResult>,
    #[pyo3(get)]
    pub date: Option<FieldResult>,
    #[pyo3(get)]
    pub publisher: Option<FieldResult>,
    #[pyo3(get)]
    pub confidence: f32, // 0.0-1.0

    // new field image

    #[pyo3(get)]
    pub image_url:Option<FieldResult>
}

pub struct FeedExtractionResult {
    pub title: String,
    pub author: String,
    pub description: String,
    pub date: String,
    // pub publisher: String,
    pub url: String, // 0.0-1.0
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct FieldResult {
    #[pyo3(get)]
    pub value: String,
    #[pyo3(get)]
    pub source: String, // 0.0-1.0
}

pub fn get_extractors() -> Vec<Box<dyn MetadataExtractor>> {
    vec![
        Box::new(OgpExtractor),
        Box::new(SchemaExtractor),
        Box::new(CommonExtracor),
    ]
}

use crate::extractors::{ogp::OgpExtractor, schema::SchemaExtractor};



pub trait MetadataExtractor: Send + Sync {
    fn extract(&self, html: &str) -> ExtractionResult;
    fn priority(&self) -> u8; // Lower = higher priority
    fn name(&self) -> &'static str;
}
#[derive(Debug,Default)]
pub struct ExtractionResult {
    pub title: Option<FieldResult>,
    pub author: Option<FieldResult>,
    pub description: Option<FieldResult>,
    pub date: Option<FieldResult>,
    pub publisher: Option<FieldResult>,
    pub confidence: f32, // 0.0-1.0
}



#[derive(Debug)]
pub struct FieldResult {
    pub value: String,
    pub source: String, // 0.0-1.0
}

pub fn get_extractors() -> Vec<Box<dyn MetadataExtractor>> {
    vec![
        Box::new(OgpExtractor),
        Box::new(SchemaExtractor)
       
    ]
}



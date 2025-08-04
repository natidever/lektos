use crate::models::metadata::{FieldResult, MetadataExtractor, get_extractors};
use pyo3::prelude::*;

#[derive(Debug, Default)]
#[pyclass]

pub struct Metadata {
    pub title: Option<FieldResult>,
    pub author: Option<FieldResult>,
    pub date: Option<FieldResult>,
    pub publisher: Option<FieldResult>,
    pub description: Option<FieldResult>,
}

// titel and which extractor it came from

#[pyclass]
pub struct MetadataPipeline {
    extractors: Vec<Box<dyn MetadataExtractor>>,
}

#[pymethods]
impl MetadataPipeline {
    #[new]
    pub fn new() -> Self {
        let mut extractors = get_extractors();

        extractors.sort_by_key(|extractor| extractor.priority());
        MetadataPipeline { extractors }
    }

    pub fn run(&self, html: &str) -> Metadata {
        let mut final_metadata = Metadata::default();
        let mut confidence = 0.0;
        let mut count = 0;

        for extractor in &self.extractors {
            let result = extractor.extract(html);
            Self::merge_field(&mut final_metadata.title, result.title);
            Self::merge_field(&mut final_metadata.author, result.author);
            Self::merge_field(&mut final_metadata.date, result.date);
            Self::merge_field(&mut final_metadata.publisher, result.publisher);
            confidence += result.confidence;
            count += 1;
        }

        final_metadata
    }
}

impl MetadataPipeline {
    fn merge_field(target: &mut Option<FieldResult>, new: Option<FieldResult>) {
        if target.is_none() {
            *target = new;
        }
    }
}

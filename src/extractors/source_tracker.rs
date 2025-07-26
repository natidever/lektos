use serde::Serialize;

use crate::{extractors::pipeline::Metadata, models::metadata::ExtractionResult};

#[derive(Debug, Default,Serialize)]
pub struct SourceTracker {
    pub ogp_titles: u64,
    pub schema_titles: u64,
    pub selector_titles: u64,

    pub ogp_authors: u64,
    pub schema_authors: u64,
    pub selector_authors: u64,

    pub ogp_dates: u64,
    pub schema_dates: u64,
    pub selector_dates: u64,

    pub ogp_publishers: u64,
    pub schema_publishers: u64,
    pub selector_publishers: u64,
}
impl SourceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, result: &Metadata) {
        if let Some(title) = &result.title {
            match title.source.as_str() {
                "ogp" => self.ogp_titles += 1,
                "schema" => self.schema_titles += 1,
                "common" => self.selector_titles += 1,
                _ => (),
            }
        }

        if let Some(author) = &result.author {
            match author.source.as_str() {
                "ogp" => self.ogp_authors += 1,
                "schema" => self.schema_authors += 1,
                "common" => self.selector_authors += 1,
                _ => (),
            }
        }

        if let Some(date) = &result.date {
            match date.source.as_str() {
                "ogp" => self.ogp_dates += 1,
                "schema" => self.schema_dates += 1,
                "common" => self.selector_dates += 1,
                _ => (),
            }
        }

        if let Some(publisher) = &result.publisher {
            match publisher.source.as_str() {
                "ogp" => self.ogp_publishers += 1,
                "schema" => self.schema_publishers += 1,
                "common" => self.selector_publishers += 1,
                _ => (),
            }
        }
    }
}

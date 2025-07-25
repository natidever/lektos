use std::result;

use scraper::Selector;

use crate::models::metadata::{ExtractionResult, FieldResult, MetadataExtractor};

// pub struct CommonSelector;

// impl CommonSelector {
//     pub fn new() -> Self {
//         CommonSelector
//     }

//     fn create_field_result(value: &str) -> FieldResult {
//         FieldResult {
//             value: value.to_string(),
//             source: "common_selector".to_string(),
//         }
//     }
// }

// impl MetadataExtractor for CommonSelector {

// fn extract(&self, html: &str) -> ExtractionResult {
//         let mut extraction_result = ExtractionResult::default();

//         let document = scraper::Html::parse_document(html);

//         let selector = match Selector::parse("meta[property]") {
//             Ok(selector) => selector,
//             Err(_) => return extraction_result, // Return empty result on error
//         };

//         let mut found_count = 0;

//         for element in document.select(&selector) {
//             let property = element.value().attr("property").unwrap_or("");
//             let content = element.value().attr("content").unwrap_or("");

//             for (og_prop, field) in OGP_MAPPING {
//                 if property == og_prop {
//                     let field_result = Self::create_field_result(content);
//                     match field {
//                         "title" => extraction_result.title = Some(field_result),
//                         "description" => extraction_result.description = Some(field_result),
//                         "author" => extraction_result.author = Some(field_result),
//                         "date" => extraction_result.date = Some(field_result),
//                         "publisher" => extraction_result.publisher = Some(field_result),
//                         _ => {}
//                     }

//                     found_count += 1;
//                     break;
//                 }
//             }
//         }

//         extraction_result.confidence = 0.2 + (0.2 * found_count as f32);
//         extraction_result
//     }

//     fn priority(&self) -> u8 {
//         3
//     }

//     fn name(&self) -> &'static str {
//         "common_selector"
//     }
// }







const OGP_MAPPING: [(&str, &str); 5] = [
    ("og:title", "title"),
    ("og:description", "description"),
    ("og:article:author", "author"),
    ("og:article:published_time", "date"),
    ("og:site_name", "publisher"),
];

pub struct OgpExtractor;

impl OgpExtractor {
    pub fn new() -> Self {
        OgpExtractor
    }

    fn create_field_result(value: &str) -> FieldResult {
        FieldResult {
            value: value.to_string(),
            source: "ogp".to_string(),
        }
    }
}

impl MetadataExtractor for OgpExtractor {
    fn extract(&self, html: &str) -> ExtractionResult {
        let mut extraction_result = ExtractionResult::default();

        let document = scraper::Html::parse_document(html);

        let selector = match Selector::parse("meta[property]") {
            Ok(selector) => selector,
            Err(_) => return extraction_result, // Return empty result on error
        };

        let mut found_count = 0;

        for element in document.select(&selector) {
            let property = element.value().attr("property").unwrap_or("");
            let content = element.value().attr("content").unwrap_or("");

            for (og_prop, field) in OGP_MAPPING {
                if property == og_prop {
                    let field_result = Self::create_field_result(content);
                    match field {
                        "title" => extraction_result.title = Some(field_result),
                        "description" => extraction_result.description = Some(field_result),
                        "author" => extraction_result.author = Some(field_result),
                        "date" => extraction_result.date = Some(field_result),
                        "publisher" => extraction_result.publisher = Some(field_result),
                        _ => {}
                    }

                    found_count += 1;
                    break;
                }
            }
        }

        extraction_result.confidence = 0.2 + (0.2 * found_count as f32);
        extraction_result
    }

    fn name(&self) -> &'static str {
        "ogp"
    }

    fn priority(&self) -> u8 {
        1
    }
}

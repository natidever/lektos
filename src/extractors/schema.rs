use std::collections::HashMap;

use scraper::{node::Doctype, Html, Selector};
use serde_json::{value, Value};

use crate::models::metadata::{ExtractionResult, FieldResult, MetadataExtractor};

const SCHEMA_MAPPING: [(&str, &str); 5] = [
    ("headline", "title"),
    ("author", "author"),
    ("description", "description"),
    ("datePublished", "date"),
    ("publisher", "publisher"),
];

pub struct SchemaExtractor;


impl SchemaExtractor {
    pub fn new() -> Self {
        SchemaExtractor
    }

    fn create_field_result(value: &str) -> FieldResult {
        FieldResult {
            value: value.to_string(),
            source: "schema".to_string(),
        }
    }


    fn parse_json_ld(script_content: &str) -> Option<HashMap<String, String>> {
        let json: Value = match serde_json::from_str(script_content) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let mut result = HashMap::new();
        let object = json.as_object()?;

        // Extract top-level fields
        for (field, target) in SCHEMA_MAPPING {
            if let Some(value) = object.get(field) {
                if let Some(s) = value.as_str() {
                    result.insert(target.to_string(), s.to_string());
                }
            }
        }

        // Special handling for nested author
        if let Some(author) = object.get("author") {
            if let Some(author_name) = author.get("name").and_then(Value::as_str) {
                result.insert("author".to_string(), author_name.to_string());
            }
        }

        // Special handling for nested publisher
        if let Some(publisher) = object.get("publisher") {
            if let Some(pub_name) = publisher.get("name").and_then(Value::as_str) {
                result.insert("publisher".to_string(), pub_name.to_string());
            }
        }

        Some(result)
    }
}





impl MetadataExtractor for SchemaExtractor{
    fn extract(&self, html: &str) -> ExtractionResult {

        let document = Html::parse_document(html);
        let mut result = ExtractionResult::default();
        let mut found_count = 0;


        let json_ld_selector = match  Selector::parse("script[type='application/ld+json']") {
            Ok(selector) => selector,
            Err(_) => return result, // Return empty result if selector parsing fails
            
        };
        for element in document.select(&json_ld_selector){
            if let Some(script_content)=element.text().next(){
                if let Some(fields)=Self::parse_json_ld(script_content){
                    for (field_name,value) in fields{
                        let field_result = Self::create_field_result(&value);

                        match field_name.as_str(){
                            "title"=> result.title = Some(field_result),
                            "author"=> result.author = Some(field_result),  
                            "date"=> result.date = Some(field_result),
                            "description"=> result.description = Some(field_result),
                            "publisher"=> result.publisher = Some(field_result),
                            _=>(),
                        }
                        found_count += 1;
                        


                    }

                }

            }


        }

        // TODO:Implement microdata scheme.or value here


        result.confidence = 0.15 + (0.2 * found_count as f32);
        result
       
       
    }

    fn priority(&self) -> u8 {
        2
    }

    fn name(&self) -> &'static str {
        "schema"
    }
}
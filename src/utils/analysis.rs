// how many blog site do we processsed how many of them are blog from how many we take out the structured data// src/analysis.rs
use csv::WriterBuilder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Result;

use serde_json;
use std::fs;

use crate::extractors::pipeline::MetadataPipeline;

#[derive(Serialize)]
pub struct BlogLog {
    pub url: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub publisher: String,
    pub word_count: usize,
}

pub fn log_blog_to_csv(log: &BlogLog, path: &str) -> Result<()> {
    println!("log_blog_to_csv called with log");
    let file = OpenOptions::new().append(true).create(true).open(path)?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(log)?;
    writer.flush()?;
    Ok(())
}




#[derive(Debug, Deserialize)]
struct Data {
    url: String,
    html: String,
}

fn analyze() {
    // reading from json getting the html
    // reading the json
    let urls = fs::read_to_string("src/analysis.json").expect("Unable to open html");

    let value: Vec<Data> = serde_json::from_str(&urls).expect("msg");

    for dats in value {
        let html = &body[html_start..];
        let html_content = String::from_utf8_lossy(&html);
        let file_html = html_content.to_string();
        let pipeline = MetadataPipeline::new();
        let metadata = pipeline.run(file_html.as_str());
        let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_str());

        let blog = Blog {
            title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
            author: metadata.author.map(|f| f.value).unwrap_or("Unknown".into()),

            date: metadata.date.map(|f| f.value).unwrap_or("".into()),
            publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
            content: blog_content,
        };
    }
}
// how many blog site do we processsed how many of them are blog from how many we take out the structured data// src/analysis.rs
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Result;

use serde_json;
use std::fs;

use crate::extractors::pipeline::MetadataPipeline;
use crate::models::blog::Blog;
use crate::utils::html_utils::BlogProcessor;

#[derive(Serialize)]
pub struct BlogLog {
    pub url: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub publisher: String,
    pub word_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct BlogResult {
    title: bool,
    author: bool,
    content: bool,
    publisher: bool,
}

impl Default for BlogResult {
    fn default() -> Self {
        Self {
            title: false,
            author: false,
            content: false,
            publisher: false,
        }
    }
}

#[derive(Default)]
pub struct FailedHTML{
    pub html:String
}

pub fn log_blog_to_csv(log: &BlogLog, path: &str) -> Result<()> {
    println!("log_blog_to_csv called with log");
    let file = OpenOptions::new().append(true).create(true).open(path)?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(log)?;
    writer.flush()?;
    Ok(())
}







pub fn analyze_result(blog: &Blog) -> BlogResult {
    let mut blog_result = BlogResult::default();
    if blog.title != "Untitled" {
        blog_result.title = true;
    }

    if blog.author != "Unknown" {
        blog_result.author = true;
    }

    if !blog.content.trim().is_empty() {
        blog_result.content = true;
    }

    if !blog.publisher.trim().is_empty() {
        blog_result.publisher = true;
    }

    blog_result
}

pub fn segregate_failed_htmls(blog:&Blog,html:&str){

    let mut failed_html = FailedHTML::default();

    if blog.author  == "Unknown" {
        let json_to_write = serde_json::to_string_pretty(html).expect("Failed to serialize");
        fs::write("faiedl_htmls.json", json_to_write).expect("unable to write on file");
        
         

    }


}
// how many blog site do we processsed how many of them are blog from how many we take out the structured data// src/analysis.rs
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::Result;
use url::{Host, Position, Url};

use crate::extractors::pipeline::MetadataPipeline;
use crate::models::blog::Blog;
use crate::utils::html_utils::BlogProcessor;

#[derive(Debug, Deserialize)]
pub struct AnalysisData {
    url: String,
    html: String,
}

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

#[derive(Default, Serialize, Deserialize)]
pub struct FailedHTML {
    pub url: String,
    pub html: String,
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

pub fn segregate_failed_htmls(blog: &Blog, html: &str, temp: &mut Vec<FailedHTML>, url: &str) {
    if blog.author == "Unknown" {
        let failed_html = FailedHTML {
            html: html.to_string(),
            url: url.to_string(),
        };

        temp.push(failed_html);
    }
}

// pub fn determine_embedding (){

// }

#[derive(Serialize, Deserialize, Debug)]
pub struct HtmlEntry {
    pub url: String,
    pub html: String,
}

pub fn extract_valid_htmls() {
    let all_data = fs::read_to_string("src/analysis.json").expect("Failed to read analysis.json");
    let all_htmls: Vec<HtmlEntry> =
        serde_json::from_str(&all_data).expect("Invalid analysis.json format");

    let failed_data =
        fs::read_to_string("failed_html.json").expect("Failed to read failed_htmls.json");
    let failed_htmls: Vec<HtmlEntry> =
        serde_json::from_str(&failed_data).expect("Invalid failed_htmls.json format");

    let failed_urls: HashSet<String> = failed_htmls.into_iter().map(|entry| entry.url).collect();

    let valid_htmls: Vec<HtmlEntry> = all_htmls
        .into_iter()
        .filter(|entry| !failed_urls.contains(&entry.url))
        .collect();

    let output =
        serde_json::to_string_pretty(&valid_htmls).expect("Failed to serialize valid HTMLs");
    fs::write("valid_htmls.json", output).expect("Failed to write valid_htmls.json");

    println!(
        "valid_htmls.json created with {} entries",
        valid_htmls.len()
    );
}

pub fn url_distribution(value: Vec<AnalysisData>) -> Vec<String> {
    let mut temp_check: Vec<String> = Vec::new();
    let mut list_of_host: Vec<String> = Vec::new();
    let mut duplicated_url = 0;
    let mut count = 0;

    let known_hosts = [
        "medium.com",
        "read.cv",
        "dev.to",
        "zeit.co",
        "blogspot.com",
        "ghost.io",
        "substack.com",
    ];

    for data in value {
        let parse_url = Url::parse(&data.url).expect("error parsing url");

        let host = parse_url.host().expect("Failed to get host").to_string();

        if known_hosts.iter().any(|kh| host.ends_with(kh)) {
            continue;
        }

        // check visited
        if temp_check.contains(&host) {
            duplicated_url += 1;
            continue;
        }
        temp_check.push(host.to_string());

        list_of_host.push(host.to_string());

        count += 1;
    }

    list_of_host
}

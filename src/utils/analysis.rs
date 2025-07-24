// how many blog site do we processsed how many of them are blog from how many we take out the structured data// src/analysis.rs
use csv::WriterBuilder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Result;

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
    let file = OpenOptions::new().append(true).create(true).open(path)?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(log)?;
    writer.flush()?;
    Ok(())
}

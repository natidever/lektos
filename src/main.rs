#![allow(unused)]
use crate::extractors::pipeline::MetadataPipeline;
use crate::extractors::runner::extractor_runner;
use crate::models::blog::Blog;
use crate::models::metadata::ExtractionResult;
use crate::models::metadata::FieldResult;
use crate::utils::analysis::BlogLog;
use crate::utils::analysis::log_blog_to_csv;
use crate::utils::find_blog_url::is_blog_url;
use crate::utils::find_feeds::extract_url;
use crate::utils::find_feeds::is_feed;
use crate::utils::html_utils::BlogProcessor;
use crate::utils::url_visit_check::UrlVisitTracker;
use crate::utils::valid_url_from_feeds::FeedUrlValidator;
use pyo3::ffi::printfunc;
use scraper::Selector;
use warc::WarcHeader;
use warc::WarcReader;
mod errors;
mod extractors;
mod models;
mod utils;

use std::{env, fs};

use anyhow::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    let vist_url_tracker = UrlVisitTracker::new();

    let feed_url_validator = FeedUrlValidator::new()?;

    let warc_name =
        "src/common_crawl_2025-26_warcfiles/CC-MAIN-20250612112840-20250612142840-00001.warc.gz";

    // let result = extractor_runner(&warc_name).await?;
    // println!("fff{:?}", result);

    Ok(())
}

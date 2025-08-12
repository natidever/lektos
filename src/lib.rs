#![allow(unused)]
pub mod errors;
pub mod extractors;
pub mod models;
pub mod utils;

use pyo3::prelude::*;

use crate::{
    extractors::{
        pipeline::{Metadata, MetadataPipeline},
        runner::core_extractor_runner,
    },
    models::metadata::FieldResult,
};

#[pymodule]
fn lektos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MetadataPipeline>()?;
    m.add_class::<Metadata>()?;
    m.add_class::<FieldResult>()?;

    Ok(())
}

#[pyfunction]
fn extractor_runner(warc_file_path: &str) {
    core_extractor_runner(warc_file_path);
}

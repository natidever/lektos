#![allow(unused)]
pub mod errors;
pub mod extractors;
pub mod models;
pub mod utils;
pub mod scylla;

// use anyhow::Ok;
// use anyhow::Ok;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes};
use pyo3::wrap_pyfunction;
use qdrant_client::qdrant::PointsOperationResponse; // Add this import at the top

use crate::utils::embed::Summary;
use crate::utils::find_blog_url::is_blog_url;
use crate::{
    extractors::{
        pipeline::{Metadata, MetadataPipeline},
        runner::extractor_runner,
    },
    models::metadata::FieldResult,
};

#[pymodule]
fn lektos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MetadataPipeline>()?;
    m.add_class::<Metadata>()?;
    m.add_class::<FieldResult>()?;

    m.add_function(wrap_pyfunction!(core_extractor_runner, m)?)?;
    m.add_function(wrap_pyfunction!(is_blog, m)?)?;

    Ok(())
}

#[pyfunction]
fn core_extractor_runner(py: Python<'_>, warc_file_path: &str) -> PyResult<Py<PyBytes>> {
    let buffer = extractor_runner(py, warc_file_path)?;
    Ok(PyBytes::new(py, &buffer).into())
}

#[pyfunction]
fn is_blog(url: &str) -> PyResult<bool> {
    Ok(is_blog_url(url))
}

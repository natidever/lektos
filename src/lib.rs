#![allow(unused)]
pub mod errors;
pub mod extractors;
pub mod models;
pub mod utils;

// use anyhow::Ok;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;
use qdrant_client::qdrant::PointsOperationResponse; // Add this import at the top

use crate::utils::embed::Summary;
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

    Ok(())
}
s


#[pyfunction]
fn core_extractor_runner(py:Python<'_>,warc_file_path:&str)->PyResult<Py<PyBytes>>{
    let buffer=extractor_runner(py, warc_file_path)?;
    Ok(PyBytes::new(py,&buffer).into())
}
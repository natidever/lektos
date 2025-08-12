#![allow(unused)]
pub mod errors;
pub mod extractors;
pub mod models;
pub mod utils;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;  // Add this import at the top

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

    m.add_function(wrap_pyfunction!(extractor_runner, m)?)?;


    Ok(())
}

#[pyfunction]
pub fn extractor_runner(warc_file_path: &str) -> PyResult<()> {
    core_extractor_runner(warc_file_path);
    Ok(())  // Must return PyResult
}




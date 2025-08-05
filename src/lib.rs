#![allow(unused)]
pub mod errors;
pub mod extractors;
pub mod models;
pub mod utils;

use pyo3::prelude::*;

use crate::{
    extractors::pipeline::{Metadata, MetadataPipeline},
    models::metadata::FieldResult,
};

#[pymodule]
fn lektos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MetadataPipeline>()?;
    m.add_class::<Metadata>()?;
    m.add_class::<FieldResult>()?;
    Ok(())
}

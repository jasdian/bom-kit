//! Error types for the BOM Kit library

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BomError>;

#[derive(Error, Debug)]
pub enum BomError {
    #[error("Part not found: {0}")]
    PartNotFound(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Incompatible units: cannot operate on {0} and {1}")]
    IncompatibleUnits(String, String),

    #[error("Invalid substitute ratio: {0}")]
    InvalidSubstituteRatio(String),

    #[error("Duplicate part ID: {0}")]
    DuplicatePartId(String),

    #[error("Invalid BOM structure: {0}")]
    InvalidStructure(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),
}

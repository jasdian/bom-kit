//! Error types for the BOM Kit library

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BomError {
    #[error("Part not found: {0}")]
    PartNotFound(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error(
        "Quantity calculation overflow during BOM explosion (likely due to deep hierarchy or cycle)"
    )]
    QuantityCalculationOverflow {
        /// Optional: the part IDs involved or multiplier values
        context: Option<String>,
    },

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

//! Part definitions and metadata

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Unique identifier for a part
pub type PartId = String;

/// Represents a part or assembly in the BOM
///
/// TODO: define Unit type here
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Part {
    /// Unique identifier
    pub id: PartId,
    /// Human-readable name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Part number (e.g., manufacturer part number)
    pub part_number: Option<String>,
    /// Manufacturer
    pub manufacturer: Option<String>, // struct possible
    /// Custom attributes
    pub attributes: HashMap<String, String>,
}

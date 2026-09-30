//! Substitute part management with conversion ratios

use crate::{PartId, Quantity};
use rust_decimal::Decimal;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Substitute {
    /// ID of the substitute part
    pub part_id: PartId,
    /// Conversion ratio within part group to substitute
    pub ratio: SubstituteRatio,
    /// Priority (lower = higher priority)
    pub priority: u32,
}

/// Conversion ratio between original and substitute parts
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SubstituteRatio {
    /// Simple 1:1 substitution
    OneToOne,
    /// Fixed ratio (e.g., 1 original = 2.5 substitute)
    Fixed(Decimal),
    /// Quantity-based conversion (e.g., 1L of original = 950ml of substitute)
    QuantityBased { original: Quantity, substitute: Quantity },
}

impl Substitute {
    /// Create a new 1:1 substitute
    pub fn one_to_one(part_id: impl Into<String>) -> Self {
        Substitute { part_id: part_id.into(), ratio: SubstituteRatio::OneToOne, priority: 0 }
    }
}

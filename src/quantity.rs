//! Quantity types and unit conversions

use rust_decimal::Decimal;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents different types of quantities
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Quantity {
    /// Discrete units (e.g., 5 screws)
    Units(u32),
}

/// Base unit types for quantity categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Unit {
    Count,
}

impl Default for Quantity {
    fn default() -> Self {
        Quantity::Units(1)
    }
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quantity::Units(n) => write!(f, "{} units", n),
        }
    }
}

impl Quantity {
    /// Convert to a base unit for comparison
    pub fn to_base_unit(&self) -> (Unit, Decimal) {
        match self {
            Quantity::Units(n) => (Unit::Count, Decimal::from(*n)),
        }
    }

    /// Check if two quantities are compatible for operations
    pub fn is_compatible(&self, other: &Self) -> bool {
        let (unit1, _) = self.to_base_unit();
        let (unit2, _) = other.to_base_unit();
        unit1 == unit2
    }

    /// Add two compatible quantities
    pub fn add(&self, other: &Self) -> Result<Self, crate::error::BomError> {
        if !self.is_compatible(other) {
            return Err(crate::error::BomError::IncompatibleUnits(
                format!("{:?}", self),
                format!("{:?}", other),
            ));
        }

        let (unit, val1) = self.to_base_unit();
        let (_, val2) = other.to_base_unit();
        let result = val1 + val2;

        Ok(match unit {
            Unit::Count => Quantity::Units(result.try_into().unwrap_or(u32::MAX)),
        })
    }

    /// Multiply quantity by a scalar
    pub fn multiply(&self, factor: Decimal) -> Self {
        match self {
            Quantity::Units(n) => {
                Quantity::Units((Decimal::from(*n) * factor).round().try_into().unwrap_or(u32::MAX))
            }
        }
    }
}

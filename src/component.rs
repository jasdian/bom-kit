//! Component representation in BOMs

use crate::{PartId, Quantity, Substitute};
use rust_decimal::Decimal;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A component in a BOM with quantity and possible substitutes
///
/// TODO: procurement_type - possible indication of sourcing
/// TODO: supplier can depend, based on the procurement_type
/// TODO: cost can be different, based on the process acquiring of that component - it shall be calculated
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Component {
    /// Part ID this component refers to
    pub part_id: PartId,
    /// Required quantity
    pub quantity: Quantity,
    /// Unit cost
    pub cost: Option<Decimal>,
    /// Supplier identifier or name
    pub supplier: Option<String>,
    /// Reference designator (e.g., "R1", "C1" for electronics)
    pub reference: Option<String>,
    /// Possible substitute parts
    pub substitutes: Vec<Substitute>,
    // /// Procurement type
    // pub procurement_type: ProcurementType,
    /// Eco-tracking
    pub sustainability: Option<Sustainability>,
}

// #[derive(Debug, Clone)]
// pub enum ProcurementType {
//     Purchased,
//     Manufactured,
//     Subcontracted,
//     Phantom,
// }

#[derive(Debug, Clone)]
pub struct Sustainability {
    pub carbon_footprint: Option<Decimal>,
    pub recyclable: bool,
    pub origin_country: Option<String>, // can be derived from the manufacturer
}

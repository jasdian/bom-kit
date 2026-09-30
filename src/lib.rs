//! # BOM Kit
//!
//! A flexible Bill of Materials (BOM) management library for assembly planning,
//! supporting various quantity types, substitutes, and unit conversions.
//!
//! ## Features
//!
//! - **Hierarchical BOMs**: Define products with assemblies and sub-assemblies
//! - **Flexible Quantities**: Support for discrete units, volumes (fluids), and weights (materials)
//! - **Substitutes**: Define alternative parts with conversion ratios
//! - **BOM Explosion**: Calculate total material requirements
//! - **Dependency Analysis**: Find where parts are used
//!
//! ## Example
//!
//! ```rust
//! use std::collections::HashMap;
//! use bom_kit::{Bom, Component, Part, Quantity};
//!
//! let part = |id: &str, name: &str| Part {
//!     id: id.into(),
//!     name: name.into(),
//!     description: None,
//!     part_number: None,
//!     manufacturer: None,
//!     attributes: HashMap::new(),
//! };
//! let component = |id: &str, n: u32| Component {
//!     part_id: id.into(),
//!     quantity: Quantity::Units(n),
//!     ..Default::default()
//! };
//!
//! // Create a simple BOM for a table
//! let mut bom = Bom::new(part("table", "Wooden Table"), Some(Quantity::Units(1)));
//!
//! // Add components
//! bom.components.push(component("top", 1));
//! bom.components.push(component("leg", 4));
//! bom.components.push(component("screw", 16));
//!
//! assert_eq!(bom.components.len(), 3);
//! ```

pub mod error;

pub mod bom;
pub mod explosion;
// pub mod inventory;

pub mod component;
pub mod part;
pub mod quantity;
pub mod substitute;

// pub mod loaders;
// pub mod export;

// pub mod factory;

pub use bom::{Bom, Dependencies};
pub use component::Component;
pub use error::BomError;
// pub use loaders::read_csv;
pub use part::{Part, PartId};
pub use quantity::{Quantity, Unit};
pub use substitute::{Substitute, SubstituteRatio};
// pub use explosion::{BomExplosion, ExplosionOptions};

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
//! use bom_kit::{Bom, Part, Quantity, Component};
//!
//! // Create a simple BOM for a table
//! let mut bom = Bom::new("table", "Wooden Table");
//!
//! // Add components
//! bom.add_component(Component::new("top", Quantity::Units(1)));
//! bom.add_component(Component::new("leg", Quantity::Units(4)));
//! bom.add_component(Component::new("screw", Quantity::Units(16)));
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

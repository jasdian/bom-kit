use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::{BomError, Component, Part, PartId, Quantity};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Bill of Materials structure
///
/// TODO: BoM can be shared across multiple Substitute - sharing/normalizing BoM
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bom {
    /// Root component/assembly
    pub root: Part,
    /// How much produces BoM with unit type
    pub quantity: Quantity,
    /// Components in this BOM
    pub components: Vec<Component>,
    /// Sub-BOMs for assembly components
    pub sub_boms: HashMap<PartId, Rc<Bom>>,
    /// Metadata
    pub metadata: BomMetadata,
    /// Level on which that part/assembly fits within the Bom hierarchy   
    pub bom_level: u32,
}

/// BOM metadata
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BomMetadata {
    pub version: String,
    pub revision: String,
    pub created_by: Option<String>,
    pub created_date: Option<String>,
    pub notes: Option<String>,
    pub attributes: HashMap<String, String>,
}

pub type Dependencies = HashMap<PartId, Vec<(PartId, Quantity)>>;

impl Bom {
    pub fn new(part: Part, quantity: Option<Quantity>) -> Self {
        Self {
            root: part,
            quantity: quantity.unwrap_or(Quantity::Units(1)),
            components: Vec::new(),
            sub_boms: HashMap::new(),
            metadata: BomMetadata::default(),
            bom_level: 0,
        }
    }

    /// Builds a full hierarchical Bom from flat dependencies with shared sub-BoMs
    ///
    /// - deps: Flat dependencies loaded from CSV or database
    /// - parts_db: Lookup table mapping PartId to full Part details
    /// - root_id: The top-level assembly/part to start building from
    ///
    /// Returns Rc<Bom> to enable sharing of identical sub-assemblies
    pub fn build_hierarchy(
        deps: &Dependencies,
        parts_db: &HashMap<PartId, Part>,
        root_id: PartId,
    ) -> Result<Rc<Self>, Box<dyn std::error::Error>> {
        let mut cache: HashMap<PartId, Rc<Bom>> = HashMap::new();

        fn build_recursive(
            id: &PartId,
            deps: &Dependencies,
            parts_db: &HashMap<PartId, Part>,
            cache: &mut HashMap<PartId, Rc<Bom>>,
            level: u32,
        ) -> Result<Rc<Bom>, Box<dyn std::error::Error>> {
            if let Some(cached) = cache.get(id) {
                return Ok(Rc::clone(cached));
            }
            let root_part = parts_db
                .get(id)
                .ok_or_else(|| format!("Part not found in parts database: {}", id))?
                .clone();

            let mut bom = Bom::new(root_part, None); // TODO quantity types
            bom.bom_level = level;

            // Get direct children if this is an assembly
            if let Some(children) = deps.get(id)
                && !children.is_empty()
            {
                for (child_id, qty) in children {
                    let component = Component {
                        part_id: child_id.clone(),
                        quantity: qty.clone(), // Direct clone of Quantity::Units(...); TODO: quantity
                        reference: None,       // TODO: reference
                        substitutes: vec![], // TODO: Rc for the parts group - store vec / ID for the group?
                        // Add other fields as needed with defaults
                        ..Default::default()
                    };
                    bom.components.push(component);

                    // Recursively build sub-Bom (shared if identical)
                    let sub_bom = build_recursive(child_id, deps, parts_db, cache, level + 1)?;
                    bom.sub_boms.insert(child_id.clone(), Rc::clone(&sub_bom));
                }
            }

            // Cache and return shared reference
            let rc_bom = Rc::new(bom);
            cache.insert(id.clone(), Rc::clone(&rc_bom));
            Ok(rc_bom)
        }

        build_recursive(&root_id, deps, parts_db, &mut cache, 0)
    }

    pub fn explode_quantities(self: &Rc<Self>) -> Result<HashMap<PartId, u32>, BomError> {
        let mut total: HashMap<PartId, u32> = HashMap::new();
        let mut queue: VecDeque<(Rc<Bom>, u32)> = VecDeque::new();
        let mut in_path: HashSet<PartId> = HashSet::new();

        let Quantity::Units(root_multiplier) = self.quantity;
        let root_id = self.root.id.clone();
        queue.push_back((Rc::clone(self), root_multiplier));
        in_path.insert(root_id.clone());

        while let Some((current_bom, current_multiplier)) = queue.pop_front() {
            let current_id: PartId = current_bom.root.id.clone();

            for component in &current_bom.components {
                let Quantity::Units(child_qty) = component.quantity;
                let child_multiplier = current_multiplier
                    .checked_mul(child_qty)
                    .ok_or(BomError::QuantityCalculationOverflow { context: None })?;

                let child_id = component.part_id.clone();

                if let Some(sub_bom) = current_bom.sub_boms.get(&child_id) {
                    if in_path.contains(&child_id) {
                        return Err(BomError::CircularDependency(
                            [current_id, child_id].join(" -> "),
                        ));
                    }
                    in_path.insert(child_id.clone());
                    queue.push_back((Rc::clone(sub_bom), child_multiplier));
                } else {
                    *total.entry(child_id).or_insert(0) += child_multiplier;
                }
            }
            in_path.remove(&current_id);
        }

        Ok(total)
    }
}

//! Components and related structs for the hulls of ships, stations, etc.
use crate::{component, unit::Distance};
use serde::{Serialize, Deserialize};

/// The `Hull` struct is the base component for all entities that have some kind
/// of hull, wether a ship or station.
/// 
/// It determines things like what components can be fitted to the entity
#[component]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hull {
    
}

/// The size of a hull cateforized into an enum
pub enum HullSize {
    Tiny,
}
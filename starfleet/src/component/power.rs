//! Components for entities that get energy from some source
use serde::{Serialize, Deserialize};
use uom::si::f32::Power;

/// The base component for all entities that use power (most entities will have this).
/// Other components like reactors can contribute or reduce the amount of power availible to the system
#[crate::component]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Powered {
    /// The power availible to this entity
    pwr: Power,
}


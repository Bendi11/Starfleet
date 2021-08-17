//! Various misc. components used commonly
use serde::{Deserialize, Serialize};
use crate::component;

use crate::state::Point;

/// This entity has a name that can be displayed
#[component]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name {
    /// The name of the entity
    pub name: String,
}

/// Allows an entity to have a position in a star system, which is synchronized every time the 
/// component changes
#[component]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    /// The location in the star system this is
    pub loc: Point,
}


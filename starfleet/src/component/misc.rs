//! Various misc. components used commonly
use serde::{Deserialize, Serialize};

/// This entity has a name that can be displayed
#[crate::component]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name {
    /// The name of the entity
    pub name: String,
}

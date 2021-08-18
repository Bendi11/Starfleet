//! The `state` module contains definitions for global state
//! contained in the engine

pub mod quadtree;
use indexmap::IndexMap;
use legion::Entity;
use quadtree::QuadTree;
pub use quadtree::{Point, Rect};
use serde::{Deserialize, Serialize};

use crate::gen::ProcGen;

/// The `State` struct holds all elements of global game state
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct State {
    /// The container for all star systems
    galaxy: Galaxy,
}



/// A star system contains any entities that are currently in the star system, and
/// is contained in the [Galaxy] struct
#[derive(Debug, Deserialize, Serialize)]
pub struct StarSystem {
    /// A map of entities to their locations
    entities: QuadTree<Entity>,
}

/// The `Galaxy` struct tracks where all star systems are in the game
#[derive(Debug, Deserialize, Serialize)]
pub struct Galaxy {
    /// A virtual map of star system indexes in the `star_map` hashmap
    stars: QuadTree<usize>,
    /// A map of star system names to star system data
    star_map: IndexMap<String, StarSystem>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            stars: QuadTree::new(Rect(Point(0., 0.), Point(10000., 10000.))),
            star_map: IndexMap::new()
        }
    }
}

impl ProcGen for StarSystem {
    fn generate() -> Self {
        Self {
            entities: QuadTree::new(Rect(Point(0., 0.), Point(0., 0.))),
        }
    }
}

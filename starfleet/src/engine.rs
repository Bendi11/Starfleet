//! The `engine` module provides the [Engine] type, which contains all global state,
//! handles any events that are raised by systems, and can save / load the game state to a 
//! file

use legion::{World, serialize::Canon};
use crossbeam_channel::Receiver;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

use crate::{event::Event, ser};

/// The `Engine` struct handles any events raised by systems, contains all global state, and
/// is responsible for serializing and deserializing the game state
#[derive(Debug)]
pub struct Engine {
    /// The [World] that contains all entities and component data
    world: World,
    /// The event queue that all events are sent down
    events: Receiver<Event>,
}

impl Engine {
    /// Returns a new Engine with no entities or systems
    #[inline]
    pub fn new_empty() -> Self {
        Self {
            world: World::default(),
            events: crossbeam_channel::unbounded().1
        }
    }

}

impl Serialize for Engine {
    /// Serialize this Engine using the given serializer
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let registry = ser::register_components();
        let entity_serializer = Canon::default();
        let serializable_world = self.world.as_serializable(legion::any(), &registry, &entity_serializer);

        let mut state = serializer.serialize_struct("Engine", 1)?;
        state.serialize_field("world", &serializable_world)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Engine {
    
    /// Deserialize an [Engine] from a given serde Deserializer
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: serde::Deserializer<'de> {
        const FIELDS: &[&str] = &["world"];
            
        //Deserialize keys in a key-value map 
        enum Field { World }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                    D: Deserializer<'de> {
                struct FieldVisitor;
                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`world`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                            E: serde::de::Error, {
                        match v {
                            "world" => Ok(Field::World),
                            _ => Err(serde::de::Error::unknown_field(v, FIELDS))
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        
        /// Struct who visits serialized values and attempts to deserialize an [Engine] from them
        struct EngineVisitor;
        impl<'de> serde::de::Visitor<'de> for EngineVisitor {
            type Value = Engine;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Engine")
            }

            /// Deserialize an [Engine] from a sequence of values
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                    A: serde::de::SeqAccess<'de>, {

                let registry = ser::register_components();
                let entity_deserializer = Canon::default();
                let deserializable= registry.as_deserialize(&entity_deserializer);
                let world = seq.next_element_seed(deserializable)?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                Ok(Engine {
                    world,
                    events: crossbeam_channel::unbounded().1
                })
            }

            /// Deserialize an [Engine] from a map of values
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                    A: serde::de::MapAccess<'de>, {
                let registry = ser::register_components();
                let entity_deserializer = Canon::default();
                let deserializable= registry.as_deserialize(&entity_deserializer);

                let mut world = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::World => {
                            if world.is_some() {
                                return Err(serde::de::Error::duplicate_field("world"))
                            }
                            world = Some(map.next_value_seed(deserializable)?);
                            break
                        }
                    }
                }
                let world = world.ok_or_else(|| serde::de::Error::missing_field("world"))?;
                
                Ok(Engine {
                    world,
                    events: crossbeam_channel::unbounded().1
                })
            }
        }

        
        deserializer.deserialize_struct("Engine", &["world"], EngineVisitor)
    }
}
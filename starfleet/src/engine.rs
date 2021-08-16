//! The `engine` module provides the [Engine] type, which contains all global state,
//! handles any events that are raised by systems, and can save / load the game state to a
//! file

//use crossbeam_channel::{Receiver, Sender};
use std::sync::mpsc::{Receiver, Sender, channel};
use legion::{serialize::Canon, Resources, Schedule, World};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use crate::{event::Event, register, state::State};

/// The `Engine` struct handles any events raised by systems, contains all global state, and
/// is responsible for serializing and deserializing the game state
#[derive(Debug)]
pub struct Engine {
    /// The [World] that contains all entities and component data
    world: World,
    /// The event queue that all events are sent down
    events: Receiver<Event>,
    /// A copy of the event sender for our event channel
    event_sender: Sender<Event>,
    /// All global game state
    state: State,
}

/// The `Schedules` struct holds a [Schedule](legion::Schedule) for each event that occurs
#[derive(Debug)]
pub struct Schedules {
    /// All systems to run on a tick
    pub tick: Schedule,
}

impl Engine {
    /// Run the main event loop
    pub fn run(&mut self) {
        let mut schedules = register::register_systems(); //Register all system functions
        let mut resource = Resources::default();
        resource.insert::<Sender<Event>>(self.event_sender.clone());

        let sender = self.event_sender.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(60));
            sender.send(Event::Tick).unwrap();
        });

        loop {
            match self.events.recv().unwrap() {
                Event::Tick => schedules.tick.execute(&mut self.world, &mut resource),
            }
        }
    }
}

impl Serialize for Engine {
    /// Serialize this Engine using the given serializer
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let registry = register::register_components();
        let entity_serializer = Canon::default();
        let serializable_world =
            self.world
                .as_serializable(legion::any(), &registry, &entity_serializer);

        let mut state = serializer.serialize_struct("Engine", 1)?;
        state.serialize_field("world", &serializable_world)?;
        state.serialize_field("state", &self.state)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Engine {
    /// Deserialize an [Engine] from a given serde Deserializer
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["world", "state"];

        //Deserialize keys in a key-value map
        enum Field {
            World,
            State,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`world`, `state`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "world" => Ok(Field::World),
                            "state" => Ok(Field::State),
                            _ => Err(serde::de::Error::unknown_field(v, FIELDS)),
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
                A: serde::de::SeqAccess<'de>,
            {
                let registry = register::register_components();
                let entity_deserializer = Canon::default();
                let deserializable = registry.as_deserialize(&entity_deserializer);
                let world = seq
                    .next_element_seed(deserializable)?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let state = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                let (send, rec) = channel();
                Ok(Engine {
                    world,
                    events: rec,
                    event_sender: send,
                    state,
                })
            }

            /// Deserialize an [Engine] from a map of values
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut world = None;
                let mut state = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::World => {
                            if world.is_some() {
                                return Err(serde::de::Error::duplicate_field("world"));
                            }
                            let registry = register::register_components();
                            let entity_deserializer = Canon::default();
                            let deserializable = registry.as_deserialize(&entity_deserializer);
                            world = Some(map.next_value_seed(deserializable)?);
                        }
                        Field::State => {
                            if state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state = Some(map.next_value()?);
                        }
                    }
                }
                let world = world.ok_or_else(|| serde::de::Error::missing_field("world"))?;
                let state = state.ok_or_else(|| serde::de::Error::missing_field("state"))?;

                let (send, rec) = channel();
                Ok(Engine {
                    world,
                    events: rec,
                    event_sender: send,
                    state,
                })
            }
        }

        deserializer.deserialize_struct("Engine", &["world", "state"], EngineVisitor)
    }
}

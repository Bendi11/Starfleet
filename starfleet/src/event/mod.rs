//! The `event` module provides definitions for all events that can be raised
//! by systems, and the additional state (if any) that is sent with the event

/// The `Event` enum is the type that all events are converted to so they can be sent
#[derive(Debug, Clone)]
pub enum Event {
    Tick,
}

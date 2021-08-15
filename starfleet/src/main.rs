use engine::Engine;
pub use starfleet_derive::{component, on_event};
pub mod register;
pub mod engine;
pub mod state;
pub mod event;
pub mod component;
pub mod system;

fn main() {
    let mut engine = Engine::new_empty();
    engine.run();
}

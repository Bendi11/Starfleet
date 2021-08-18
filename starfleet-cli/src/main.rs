pub mod shell;
use std::sync::Arc;
use parking_lot::Mutex;

fn main() {
    let engine = Arc::new(Mutex::new(starfleet::Engine::new_empty()));
    let shell = shell::Shell::new(engine.lock().event_sender.clone());
    shell.run(engine.clone()).unwrap();
}

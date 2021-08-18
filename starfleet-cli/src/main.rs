pub mod shell;
use std::sync::Arc;
use parking_lot::Mutex;

fn main() {
    let engine = Arc::new(Mutex::new(starfleet::Engine::new_empty()));
    let engine_mutex = engine.clone();
    let (sender, reciever) = std::sync::mpsc::channel();
    let sender_clone = sender.clone();
    let shell = shell::Shell::new(sender);
    //Spawn a thread for systems running
    std::thread::spawn(move || {
        starfleet::Engine::run(engine_mutex, sender_clone, reciever)
    });
    shell.run(engine.clone()).unwrap(); //Dedicate this thread to user interaction
}

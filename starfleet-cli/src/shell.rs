//! The [Shell] struct emulates a real shell, parsing commands with the [shellwords] crate and passing them to 
//! registered programs
use std::{collections::HashMap, io::Write, sync::mpsc::Sender};

use starfleet::{engine::Engine, event::Event};
use std::sync::Arc;
use parking_lot::Mutex;
use termcolor::{StandardStream, Color, WriteColor, ColorChoice, ColorSpec};

/// A struct that parses commands given to the program and runs the appropriate 
/// programs
#[derive(Clone)]
pub struct Shell {
    /// A map of program names to functions to run that take in the game state and command
    /// line arguments to produce a result
    pub programs: HashMap<String, fn(Arc<Mutex<Engine>>, &[String], &mut StandardStream) -> i32>,

    /// Event sender for sending the EXIT event
    sender: Sender<Event>,
}

impl Shell {
    /// Create a new [Shell] with the given event channel
    pub fn new(sender: Sender<Event>) -> Self {
        Self {
            sender,
            programs: HashMap::new()
        }
    }

    /// Loop endlessly, sending the EXIT event when the exit command is encountered
    pub fn run(&self, engine: Arc<Mutex<Engine>>) -> Result<(), std::io::Error> {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);     
        loop {
            let mut line = String::new();
            stdout.write_all(b"> ")?;
            stdout.flush()?;
            let stdin = std::io::stdin();
            stdin.read_line(&mut line)?;
            drop(stdin);
            let words = match shellwords::split(&line) {
                Ok(words) if words.len() > 0 => words,
                Ok(_) => {
                    stdout.write_all(&[b'\n'])?;
                    continue
                }   
                Err(_) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                    stdout.write_all(b"Error when parsing shell command: quotation marks mismatch\n")?;
                    stdout.reset()?;
                    continue
                }
            };

            match words[0].as_str() {
                "exit" => {
                    self.sender.send(Event::Exit).unwrap();
                    break
                },
                other => match self.programs.get(other) {
                    Some(prog) => { (prog)(engine.clone(), &words, &mut stdout); },
                    None => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                        stdout.write_fmt(format_args!("Error when running program: Command or program '{}' does not exist\n", &words[0]))?;
                        stdout.reset()?;
                    }
                }
            } 
        }
        Ok(())
    }
}


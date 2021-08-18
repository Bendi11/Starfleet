pub use starfleet_derive::{component, on_event};
pub mod component;
pub mod engine;
pub mod event;
pub mod gen;
pub mod register;
pub mod state;
pub mod system;

fn main() {
    use uom::si::f32::Power;

    let mut engine = engine::Engine::new_empty();
    let entity = engine.world.push((component::power::Powered {
        pwr: Power::new::<uom::si::power::watt>(100.)
    }, ));
    println!("World: {:#?}", engine.world);
}   

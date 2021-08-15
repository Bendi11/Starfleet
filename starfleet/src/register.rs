//! The `register` module provides platform-independent component and system registration for the `legion` crate
use legion::serialize::Registry;
use crate::engine::Schedules;

#[cfg(use_linkme)]
#[::linkme::distributed_slice]
pub static COMPONENT_HASHES: [fn(&mut Registry<u64>)] = [..];

#[cfg(use_linkme)]
#[::linkme::distributed_slice]
pub static SYSTEM_REGISTRARS: [fn(&mut SchedulesBuilder)] = [..];

/// A builder for the `Schedules` struct 
pub struct SchedulesBuilder {
    pub tick: legion::systems::Builder
}

impl SchedulesBuilder {
    pub fn build(mut self) -> Schedules {
        Schedules {
            tick: self.tick.build()
        }
    }
}

#[cfg(use_inventory)]
pub struct SystemRegistrarFunction(pub fn(&mut Schedules));

#[cfg(use_inventory)]
::inventory::collect!(SystemRegistrarFunction);

#[cfg(use_inventory)]
pub struct RegistrarFunction(pub fn(&mut Registry<u64>));

#[cfg(use_inventory)]
::inventory::collect!(RegistrarFunction);

/// Register all components using the `linkme` crate
#[cfg(use_linkme)]
pub fn register_components() -> Registry<u64> {
    let mut registry = Registry::new();
    for component_registrar in COMPONENT_HASHES {
        component_registrar(&mut registry);
    }
    registry
}

/// Register all systems using the `linkme` crate
#[cfg(use_linkme)]
pub fn register_systems() -> Schedules {
    let mut schedules = SchedulesBuilder {
        tick: legion::Schedule::builder()
    };
    for system_registrar in SYSTEM_REGISTRARS {
        system_registrar(&mut schedules);
    }
    schedules.build()
}

/// Register all components using the `inventory` crate
#[cfg(use_inventory)]
pub fn register_components() -> Registry<u64> {
    let mut registry = Registry::new();
    for component_registrar in inventory::iter::<RegistrarFunction> {
        component_registrar(&mut registry);
    }
    registry
}

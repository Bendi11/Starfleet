//! The `ser` module provides platform-independent component registration for the `legion` crate
use legion::serialize::Registry;

#[cfg(use_linkme)]
#[::linkme::distributed_slice]
pub static COMPONENT_HASHES: [fn(&mut Registry<u64>)] = [..];

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

/// Register all components using the `inventory` crate
#[cfg(use_inventory)]
pub fn register_components() -> Registry<u64> {
    let mut registry = Registry::new();
    for component_registrar in inventory::iter::<RegistrarFunction> {
        component_registrar(&mut registry);
    }
    registry
}

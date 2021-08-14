//! The `ser` module provides platform-independent component registration 
use legion::serialize::Registry;

#[cfg(use_linkme)]
#[::linkme::distributed_slice]
pub static COMPONENT_HASHES: [fn(&mut Registry<u64>)] = [..];

#[cfg(use_inventory)]
pub struct RegisterFunction(pub fn(&mut Registry<u64>));

#[cfg(use_inventory)]
::inventory::collect!(RegisterFunction);
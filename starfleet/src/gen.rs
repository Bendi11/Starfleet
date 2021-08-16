//! Traits for maintaining a uniform interface to procedurally generate entities
//! when starting a new game

/// Trait defining a common interface for procedural generation.
/// Trait used to define an interface only to encourage an
/// organized implementation of procedural generation
pub trait ProcGen {
    /// Procedurally generate a version of `Self` using no other state
    fn generate() -> Self;
}

/// A version of the [ProcGen] trait defining a way to procedurally generate a type using
/// external state
pub trait ProcGenSeeded {
    /// The type that will be passed to the `generate_seeded` function
    type Seed;
    /// Procedurally generate a version of `Self` using the passed `Seed` type
    fn generate_seeded(seed: Self::Seed) -> Self;
}


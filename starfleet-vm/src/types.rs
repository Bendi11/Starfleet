//! Data structures defining what type all variables are

use std::collections::HashMap;

/// An enum ensuring that only integer types of valid bit size are allowed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntWidth {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
    SixtyFour = 64
}

impl IntWidth {
    /// Return the maximum value an integer with the specified bit width
    pub fn max_val(&self, signed: bool) -> u64 {
        match signed {
            true => 2u64.pow(self as u32 - 2),
            false => 2u64.pow(self as u32)
        }
    }
}

/// A type containing multiple other types by name
#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    /// All field types of this data structure
    pub fields: HashMap<String, Type>,

    /// The type ID of this structure type
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// An integer type with specified width and signededness
    Int(bool, IntWidth),
    /// A true or false type with bit size 1
    Bool,
    /// An array containing type and with size
    Array(Box<Type>, u64),
    /// An structure type with type ID
    Struct(u64),
}



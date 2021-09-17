//! Bytecode instruction definitions 

/// A wrapper over the `u8` type, which holds associated constants that represent all instruction
/// values
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ins(u8);

/// Instruction metadata containing description of an instruction, name, and argument size in bytes
#[derive(Clone, Copy, Debug,)]
pub struct InsMeta {
    /// The name of this opcode
    pub name: &'static str,
    /// Instruction description
    pub desc: &'static str,
    /// Size of all arguments in bytes
    pub args: u8,
}

impl Ins {
    /// Get the metadata associated with this instruction
    pub const fn data(&self) -> InsMeta {
        match self {
            Self::HALT => InsMeta {
                name: "halt",
                desc: "stop the currently running program",
                args: 0,
            },

            _ => InsMeta {
                name: "invalid",
                desc: "invalid opcode value",
                args: 0
            }
        }
    }
}

impl std::ops::Deref for Ins {
    type Target = u8;
    fn deref(&self) -> Self::Target {
        self.0
    }
}

impl Ins {
    pub const HALT: Ins = Ins(0);
}

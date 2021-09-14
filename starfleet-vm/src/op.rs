//! Virtual machine opcode instruction definitions

use std::str::FromStr;

/// C-style enum that names all virtual machine instructions
///
/// Arguments are handled per bit in opcodes, because there are only 4 registers in the VM,
/// usually an opcode argument is only one byte, made of 4 2-bit pairs specifying registers / 
/// arguments in some special cases like [LCTINY](Opcode::LCTINY)
///
/// Arguments with register addresses are addressed like this: 
/// ```notrust
///  3 |  2 |  1  |  0
/// 00 | 00 | 00  | 00
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpCode {
    /// Stop program execution
    HALT,

    /// Load a constant 6-bit value into the registered specified by arg 0, using the remaining
    /// 6 bits in the argument byte for the loaded value
    LCTINY,
    /// Load the 8 bit value specified by arg 1 into the register specified by arg 0
    LCBYTE,
    /// Load the 16 bit value specified by args 1 and 2 into the register specified by arg 0
    LCWORD,
    /// Load the 32 bit value specified by args 1, 2, 3 and 4 into the register specified by arg 0
    LCDWORD,
    /// Load the 64 bit value specified by args 1, 2, 3, 4, 5, 6, 7, 8 into the register specified by arg 0
    LCQWORD,

    /// Add two unsigned values from registers arg0-1 and arg0-2 and store the result in arg0-0
    UADD,

    /// Add two signed value from registers arg0-1 and arg0-2 and store the result in arg0-0
    IADD,

    /// Subtract two unsigned values, arg0-1 - arg0-2 and store the result in arg0-0
    USUB,

    /// Subtract two signed values, arg0-1 - arg0-2 and store the result in arg0-0
    ISUB,
    
    /// Multiply unsiged values arg0-1 by arg0-2 and store the result in arg0-0
    UMUL,

    /// Multiply signed values arg0-1 by arg0-2 and store the result in arg0-0
    IMUL,

    /// Unsigned integer divide arg0-2 from arg0-1 and store the result in arg0-0
    UDIV,

    /// Signed integer divide arg0-2 from arg0-1 and store the result in arg0-0
    IDIV,
}


impl FromStr for OpCode {
    type Err = ();
    /// Convert an argument into an opcode value case-insensitive
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "halt" => Self::HALT,

            "lctiny" => Self::LCTINY,
            "lcbyte" => Self::LCBYTE,
            "lcword" => Self::LCWORD,
            "lcdword" => Self::LCDWORD,
            "lcqword" => Self::LCQWORD,

            "uadd" => Self::UADD,
            "iadd" => Self::IADD,
            "usub" => Self::USUB,
            "isub" => Self::ISUB,
            "umul" => Self::UMUL,
            "imul" => Self::IMUL,
            "udiv" => Self::UDIV,
            "idiv" => Self::IDIV,

            _ => return Err(())
        })
    }
}

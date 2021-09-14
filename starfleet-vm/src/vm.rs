//! Contains the data structure definition holding all virtual machine state
use thiserror::Error;
use crate::{format::Code, op::OpCode, util::{Bits, ReadExt}};
use std::mem;

/// Virtual machine state, containing methods for executing opcodes
#[derive(Clone, Debug)]
pub struct VM {
    /// The current stack that local variables are allocated on
    stack: Vec<u8>,

    /// The current stack pointer, pointing to the next free stack position
    sp: usize,

    /// The array of registers r0 - r3
    regs: [ u64 ; 4 ],
}

impl VM {
    /// Create a new VM with the given stack size
    pub fn new(stack_size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(stack_size),
            sp: 0,
            regs: [ 0u64 ; 4 ]
        }
    }

    /// Execute the given bytecode until a HALT instruction is encountered
    pub fn exec(&mut self, mut code: Code<'_>) -> VMResult<()> {
        loop {
            match unsafe { mem::transmute::<_, OpCode>(code.read_u8()?) } {
                OpCode::HALT => break,
                OpCode::LCTINY => {
                    let arg = code.read_u8()?;
                    let reg = arg.pairat(0);
                    let val = (arg & 0b11111100) >> 2; //Get the top 6 bits from the argument
                    self.regs[reg as usize] = val as u64;
                },
                OpCode::LCBYTE => {
                    let reg = code.read_u8()?.pairat(0);
                    let val = code.read_u8()?;
                    self.regs[reg as usize] = val as u64;
                },
                OpCode::LCWORD => {
                    let reg = code.read_u8()?.pairat(0);
                    let val = code.read_u16()?;
                    self.regs[reg as usize] = val as u64;
                },
                OpCode::LCDWORD => {
                    let reg = code.read_u8()?.pairat(0);
                    let val = code.read_u32()?;
                    self.regs[reg as usize] = val as u64;
                },
                OpCode::LCQWORD => {
                    let reg = code.read_u8()?.pairat(0);
                    let val = code.read_u64()?;
                    self.regs[reg as usize] = val;
                },
            }
        }

        Ok(())
    }
}

/// Enum representing all types of errors that can occur in the virtual machine
#[derive(Debug, Error)]
pub enum VMErr {
    /// We exceeded the stack size
    #[error("Stack overflow")]
    StackOverflow,

    #[error("Internal input / output error: {}", .0)]
    IO(#[from] std::io::Error),
}

pub type VMResult<T> = Result<T, VMErr>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_const() {
        let mut vm = VM::new(1024);
        vm.exec(Code::new(&[OpCode::LCTINY as u8, 0b0000101, OpCode::HALT as u8])).unwrap();
        assert_eq!(vm.regs[1], 1, "LCTINY opcode fails to load the correct constant value");

        vm.exec(Code::new(&[OpCode::LCBYTE as u8, 0b00000001, 142u8, OpCode::HALT as u8])).unwrap();
        assert_eq!(vm.regs[1], 142, "LCBYTE opcode fails to load the correct constant value: {:?}", vm);
    }
}
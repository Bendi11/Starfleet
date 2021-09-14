//! Module defining the data structure for compiled spark files
use std::io::{self, Read};

/// Structure holding bytecode and an instruction pointer that is always kept in sync 
pub struct Code<'code> {
    pub ip: usize,
    pub code: &'code [u8],
}

impl<'code> Code<'code> {
    /// Create a new `Code` instance from the given bytecode array
    pub fn new(code: &'code [u8]) -> Self {
        Self {
            code,
            ip: 0
        }
    }
    
    /// Read one byte from the source
    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8];
        self.code.read(&mut buf)?;
        self.ip += 1;
        Ok(buf[0])
    }

    // Read a word from the underlying reader
    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8 ; 2];
        self.code.read(&mut buf)?;
        self.ip += 2;
        Ok(u16::from_le_bytes(buf))
    }

    // Read a double word from the underlying reader
    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8 ; 4];
        self.code.read(&mut buf)?;
        self.ip += 4;
        Ok(u32::from_le_bytes(buf))
    }

    // Read a quad word from the underlying reader
    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8 ; 8];
        self.code.read(&mut buf)?;
        self.ip += 8;
        Ok(u64::from_le_bytes(buf))
    }
}

/// Holds all information in one compiled spark exe file
pub struct Exe {
    
}

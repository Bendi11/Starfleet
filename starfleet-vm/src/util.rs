//! Utility traits and functions helping various parts of the VM
use std::io::{self, Read};

/// A trait for reading multiple byte integer values from a reader using little-endian byte ordering
pub trait ReadExt: Read {
    /// Read one byte from the source
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    // Read a word from the underlying reader
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8 ; 2];
        self.read(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    // Read a double word from the underlying reader
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8 ; 4];
        self.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    // Read a quad word from the underlying reader
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8 ; 8];
        self.read(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

impl<R: Read> ReadExt for R {}

/// Trait defining functions for accessing specific bits of a number
pub trait Bits {
    /// Get the bit at a certain index
    fn bitat(&self, idx: u8) -> bool;

    /// Get the pair of bits at a certain index
    #[inline(always)]
    fn pairat(&self, idx: u8) -> u8 {
        (if self.bitat(idx + 1) { 1u8 } else { 0u8 } << 1) |
        (if self.bitat(idx) { 1u8 } else { 0u8 })
    }
}

impl Bits for u8 {
    #[inline(always)]
    fn bitat(&self, idx: u8) -> bool {
        ( (self >> idx) & 1) > 0
    }
}
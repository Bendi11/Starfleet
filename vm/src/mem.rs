//! Memory data structures for constant pool and local variable memory
use thiserror::Error;

/// A chunk of allocated memory for the VM, with bounds checking and 
/// easier access to writes
#[derive(Clone, Debug, )]
pub struct Mem {
    /// A heap-allocated byte array that is indexed by addresses
    mem: Vec<u8>,
}

/// The data type used to index [Mem] 
pub type Addr = usize;

/// A result type with an `Err` variant of [MemErr]
pub type MemResult<T> = Result<T, MemErr>;

impl Mem {
    /// Create a new `Mem` with the given size
    pub fn new(size: usize) -> Self {
        Self {
            mem: Vec::with_capacity(size)
        }
    }
    
    /// Read `size` bytes from memory at address `addr`, returning a slice of the memory at the
    /// given address, or `Err`if the read was out of bounds
    pub fn read_at(&self, addr: Addr, size: usize) -> MemResult<&[u8]> {
        if addr + size >= self.mem.len() {
            Err(MemErr::ReadOOB(addr))
        } else {
            Ok(&self.mem[addr..(addr + size)])
        }
    }
    
    /// Write `data` to memory at address `addr`, returning `Err` if the write is out of bounds
    pub fn write_at(&mut self, addr: Addr, data: &[u8]) -> MemResult<()> {
        if addr + data.len() >= self.mem.len() {
            Err(MemErr::WriteOOB(addr))
        } else {
            Ok((&mut self.mem[addr..data.len()]).copy_from_slice(data))
        }
    }
}

/// All errors possible when reading or writing memory
#[derive(Clone, Debug, Error)]
pub enum MemErr {
    #[error("out of bounds read at {:#X}", .0)]
    ReadOOB(Addr),
    
    #[error("out of bounds write at {:#X}", .0)]
    WriteOOB(Addr),
}



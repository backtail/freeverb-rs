#![no_std]

pub(crate) mod memory_slice;

pub use crate::memory_slice::mem_slice::{self, MemSlice};
pub use crate::memory_slice::mut_mem_slice::{self, MutMemSlice};
pub use crate::memory_slice::MemSliceError;

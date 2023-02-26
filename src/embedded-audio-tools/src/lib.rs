#![no_std]

pub(crate) mod all_pass;
pub(crate) mod comb;
pub(crate) mod delay_line;
pub(crate) mod memory;

pub use all_pass::AllPass;
pub use comb::Comb;
pub use delay_line::DelayLine;
pub use memory::mem_slice::MemSlice;
pub use memory::mut_mem_slice::MutMemSlice;

pub mod mut_mem_slice {
    pub use crate::memory::mut_mem_slice::from_slice;
}

pub mod mem_slice {
    pub use crate::memory::mem_slice::from_slice;
}

pub mod errors {
    pub use crate::memory::MemSliceError;
}

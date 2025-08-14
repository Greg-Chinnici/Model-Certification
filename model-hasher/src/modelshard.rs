use std::path::Path;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct TensorMeta {
    pub name: String,
    pub start: u64, // byte offset in file's data section
    pub len: u64,   // byte length of tensor
}


pub trait ModelShard {
    fn open(path: &Path) -> Result<Self> where Self: Sized;

    /// Returns all tensors in this shard
    fn tensors(&self) -> &[TensorMeta];

    /// Absolute offset of the shard's data section start
    fn data_section_start(&self) -> u64;

    fn mmap(&self) -> Result<&memmap2::Mmap>;
}

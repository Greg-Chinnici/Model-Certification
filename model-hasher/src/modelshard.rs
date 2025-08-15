use std::path::Path;
use anyhow::Result;
use safetensors::{tensor::Dtype};


#[derive(Clone, Debug)]
pub struct TensorMeta {
    pub dtype: Dtype,
    pub shape: Vec<usize>,
    pub data_offsets: (usize, usize),
}


pub trait ModelShard {
    fn open(path: &Path) -> Result<Self> where Self: Sized;

    /// Returns all tensors in this shard
    fn tensors(&self) -> &[TensorMeta];

    /// Absolute offset of the shard's data section start
    fn data_section_start(&self) -> u64;
    fn read_tensor_bytes(meta: TensorMeta, span_n:usize , span_w:usize) -> anyhow::Result<()>;

    fn mmap(&self) -> Result<&memmap2::Mmap>;
}

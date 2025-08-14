use crate::modelshard::{ModelShard , TensorMeta};
use anyhow::Result;
use std::{fs::File, path::Path};
use memmap2::Mmap;
use serde_json::Value;
use safetensors::{self, SafeTensors, tensor::Metadata , tensor::Dtype};
use std::collections::HashMap;

pub struct SafetensorsShard {
    path: String,
    data_start: u64,
    tensors: HashMap<String, TensorMeta>,
    mmap: Mmap,
}

/*
pub struct TensorMeta {
    dtype: Dtype,
    shape: Vec<usize>,
    data_offsets: (usize, usize),
}
*/

impl ModelShard for SafetensorsShard {

    fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let (header_size , metadata): (usize, Metadata) = SafeTensors::read_metadata(&mmap)?;
        if let Some(meta_map) = metadata.metadata() {
            println!("__metadata__ entries:");
            for (k, v) in meta_map.iter() {
                println!("  {}: {}", k, v);
            }
        } else {
            println!("No __metadata__ present in header.");
        }

        println!("Tensors in file:");
        let tensors: HashMap<String, TensorMeta> = metadata
                .tensors()
                .iter()
                .map(|(name, info)| {
                    println!("Tensor: {}", name);
                    println!("  dtype: {:?}", info.dtype);
                    println!("  shape: {:?}", info.shape);
                    println!("  data_offsets: {:?}", info.data_offsets);
                    (
                        name.clone(),
                        TensorMeta {
                            dtype: info.dtype,
                            shape: info.shape.to_vec(),
                            data_offsets: info.data_offsets,
                        },
                    )
                })
                .collect();

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            data_start: 8 + header_size as u64,
            tensors: tensors,
            mmap,
        })
    }

    //fn read_tensor_bytes(&self, idx: usize, offset: usize, length: usize) -> Result<Vec<u8>> {
    //    unimplemented!()
    //}

    fn tensors(&self) -> &[TensorMeta]{unimplemented!()}
    /// Absolute offset of the shard's data section start
    fn data_section_start(&self) -> u64 {unimplemented!()}
    fn mmap(&self) -> Result<&memmap2::Mmap> {unimplemented!()}
}

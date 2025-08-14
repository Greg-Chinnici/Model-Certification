use crate::modelshard::{ModelShard, TensorMeta};
use anyhow::Result;
use std::{fs::File, path::Path};
use memmap2::Mmap;
use serde_json::Value;

pub struct SafetensorsShard {
    path: String,
    data_start: u64,
    tensors: Vec<TensorMeta>,
    mmap: Mmap,
}

impl ModelShard for SafetensorsShard {

    fn open(path: &Path) -> Result<Self> {
       let file = File::open(path)?;
       let mmap = unsafe { Mmap::map(&file)? };
       let header_len = u64::from_le_bytes(mmap[0..8].try_into()?);
       let header_json = &mmap[8..8 + header_len as usize];
       let parsed: Value = serde_json::from_slice(header_json)?;

       let mut tensors = Vec::new();
       for (name , meta) in parsed.as_object().unwrap() {
            let offsets = meta["data_offsets"].as_array().unwrap();
            let start = offsets[0].as_u64().unwrap();
            let end = offsets[1].as_u64().unwrap();
            tensors.push(TensorMeta {
                name: name.clone(),
                start,
                len: end - start,
            });
        }

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            data_start: 8 + header_len,
            tensors,
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

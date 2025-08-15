use crate::modelshard::{ModelShard , TensorMeta};
use crate::generators::{fibonacci::Fibonacci , stride::Stride};

use anyhow::Result;
use std::{fs::File, path::Path};
use memmap2::Mmap;
use safetensors::{self, SafeTensors, tensor::Metadata , tensor::Dtype};

// Use BTreeMap so it can keep order, without needing to resort after read in
// normal hashmaps dont guarentee order
use std::collections::BTreeMap;

pub struct SafetensorsShard {
    path: String,
    data_start: u64,
    tensors: BTreeMap<String, TensorMeta>,
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
        let mut tensors: BTreeMap<String, TensorMeta> = metadata
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

        // Need to sort the keys, for now it is not dterminsitc
        // rust hashmaps cant be directly sorted, os maybe use a similar structure
        // workaround for now could be to hash each tensor then sort those hashes then flatten them into 1 hash

        verify_and_print_tensors(&tensors);

        println!("There are {} different tensors", tensors.len());

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            data_start: 8 + header_size as u64,
            tensors: tensors,
            mmap,
        })
    }

    fn read_tensor_bytes(meta: TensorMeta, span_n:usize , span_w:usize) -> anyhow::Result<()> {
        // calc the total size of Tensor
        // if stride is not given, then use a generator sequence
        // read in bytes of len span_w
        let total_size:u64 = dtype_size_bytes(meta.dtype).unwrap() as u64 * meta.shape.iter().product() as u64;
        //  select the correct iterator implemtation if no good stride_n


        Ok(())
    }

    fn tensors(&self) -> &[TensorMeta]{unimplemented!()}
    /// Absolute offset of the shard's data section start
    fn data_section_start(&self) -> u64 {self.data_start}
    fn mmap(&self) -> Result<&memmap2::Mmap> {unimplemented!()}
}

fn dtype_size_bytes(dtype: Dtype) -> Option<usize> {
    match dtype {
        Dtype::F64 => Some(8),
        Dtype::F32 => Some(4),
        Dtype::F16 | Dtype::BF16 => Some(2),
        Dtype::I64 | Dtype::U64 => Some(8),
        Dtype::I32 | Dtype::U32 => Some(4),
        Dtype::I16 | Dtype::U16 => Some(2),
        Dtype::I8  | Dtype::U8  => Some(1),
        Dtype::BOOL => Some(1),
        _ => None, // In case new dtypes are added
    }
}

pub fn verify_and_print_tensors(tensors: &BTreeMap<String, TensorMeta>) {
    for (name, meta) in tensors {
        let elem_size = match dtype_size_bytes(meta.dtype) {
            Some(sz) => sz,
            None => {
                println!("Tensor: {} has unsupported dtype: {:?}", name, meta.dtype);
                continue;
            }
        };

        let num_elements: usize = meta.shape.iter().product();
        let expected_bytes = num_elements * elem_size;

        let (start, end) = meta.data_offsets;
        let actual_bytes = (end - start) as usize;

        println!("Tensor: {}", name);
        println!("  dtype: {:?}", meta.dtype);
        println!("  shape: {:?}", meta.shape);
        println!("  elements: {}", num_elements);
        println!("  bytes per element: {}", elem_size);
        println!("  expected bytes: {}", expected_bytes);
        println!("  actual bytes: {}", actual_bytes);
        println!(
            "  VALID: {}\n",
            if expected_bytes == actual_bytes { "good section" } else { "bad section" }
        );
    }
}

use std::path::PathBuf;
use crate::modelshard::ModelShard;

/// Derive PRNG seed from dev key, version, date, salt
pub fn derive_seed(
    dev_key_thumbprint: &str,
    version: &str,
    release_date: &str,
    salt: &str
) -> [u8; 32] {
    unimplemented!()
}

/// Load model shards from paths and autodetect format
pub fn load_model_shards(paths: Vec<PathBuf>) -> anyhow::Result<Vec<Box<dyn ModelShard>>> {
    unimplemented!()
}

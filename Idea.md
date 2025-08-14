# Model Hasher (Rust)

## Overview
The Model Hasher is a Rust-based utility designed to quickly compute cryptographic hashes of machine learning model files, for a unique fingerprint. It is intended to be a fast, standalone component that the Python side of the project can call to check for differences in models to verfiy it has not been changed.  

All downloading, file management, and database storage are handled by Python. Rust’s role is limited to **hash computation and output formatting**.

---

## Plan

1. **CLI Interface**
   - Accept file path to a text file with a list of all model shards (safetensors , gguf).
   - Output hash results to stdout in a structured format, or a file with more context. 

2. **Hash Computation**
   - Read files in chunks to support large model files without excessive memory usage.
   - Compute cryptographic hashes using ChaCha, Blake3 , or some other (undecided for now) 
   - Ensure deterministic and repeatable output psued random seed made by certified parameters.

3. **Integration with Python**
   - Python handles downloading the model and writing it to disk.
   - Rust executable is called by Python (via `subprocess` ) to compute the hash.
   - Python reads the Rust output and stores it in SQLite or processes it as needed.

### CLI Parameters

## Source Files

| Short | Long             | Type   | Default | Description |
|-------|-----------------|--------|---------|-------------|
| `-m`  | `--model-list`   | String | `""`    | Path to a text file containing a list of model parts to hash. |

---

## Pseudo-Random Number Generator / Hash Inputs

| Short | Long                   | Type   | Default | Description |
|-------|-----------------------|--------|---------|-------------|
| `-s`  | `--salt`               | String | `""`    | Salt to use for hashing. Recommended at least 16 characters of hex. |
| `-d`  | `--dev-key-thumbprint` | String | `""`    | Dev key thumbprint to include in hashing (from certificate). |
| `-v`  | `--version`            | String | `""`    | Version string to include in hashing. |
| `-r`  | `--release-date`       | String | `""`    | Release date to use for hashing (UTC). |

---

## Seek Behavior

| Short | Long       | Type | Default | Description |
|-------|------------|------|---------|-------------|
| `-n`  | `--stride` | usize | 17      | Number of tensors to skip on each seek. |
| `-w`  | `--span`   | usize | 1024    | Digest size in bytes. |
| `-k`  | `--samples`| usize | 200     | Maximum number of samples to hash. |

---

## Config and Output

| Short | Long          | Type        | Default | Description |
|-------|---------------|------------|---------|-------------|
| N/A   | `--config`    | Option<String> | None    | Optional JSON config file containing all parameters. |
| N/A   | `--output-file` | Option<String> | None  | Optional path to save the hash output to a file instead of stdout. |

---

### Notes
- Most arguments have default values and are optional.
- `--config` allows passing all parameters via a JSON file for automation.


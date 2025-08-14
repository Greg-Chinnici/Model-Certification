use clap::Parser;
use std::{fs::File, path::PathBuf, fs};
use std::io::{self, BufRead, BufReader};

use serde_json;
use serde::Deserialize;

use crate::models::safetensors::SafetensorsShard;
use crate::modelshard::ModelShard;

mod fingerprint;
mod modelshard;
mod models;
mod utils;

#[derive(Parser)]
#[command(version,about = "Fingerprint open-weight models for integrity verification")]
struct Cli {
    /// Source Files
    #[arg(short='m', long="model-list", default_value = "", help = "path to text file containing list of model parts")]
    model_list_file: String,

    /// Psuedo-random number generator seed parts
    #[arg(short='s', long="salt", default_value = "", help = "salt to use for hashing (recommend at least 16 characters of hex)")]
    salt: String,
    #[arg(short='d', long="dev-key-thumbprint", default_value = "", help = "dev key thumbprint to use for hashing (from cert)")]
    dev_key_thumbprint: String,
    #[arg(short='v', long="model-version", default_value = "", help = "version to use for hashing")]
    model_version: String,
    #[arg(short='r', long="release-date", default_value = "", help = "release date to use for hashing (UTC)")]
    release_date: String,

    /// Seek Behavior
    #[arg(short='n', long="stride", default_value_t = 17 , help = "tensors to skip each seek")]
    stride_n: usize,
    #[arg(short='w', long="span", default_value_t = 1024 , help = "digest size in bytes")]
    span_w: usize,
    #[arg(short='k', long="samples", default_value_t = 200 , help = "max number of samples to hash")]
    samples: usize,

    /// Config and Output
    #[arg(long="config-file")]
    config_file: Option<String>,

    #[arg(long)] // all the parameters as above but in a json file
    output_file: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Config {
    model_list_file: Option<String>,
    salt: Option<String>,
    dev_key_thumbprint: Option<String>,
    model_version: Option<String>,
    release_date: Option<String>,
    stride_n: Option<usize>,
    span_w: Option<usize>,
    samples: Option<usize>,
    output_file: Option<Option<String>>,
}
impl Cli {
    fn merge_with_config(mut self, config: Config) -> Self {
        if let Some(v) = config.model_list_file { self.model_list_file = v; }
        if let Some(v) = config.salt { self.salt = v; }
        if let Some(v) = config.dev_key_thumbprint { self.dev_key_thumbprint = v; }
        if let Some(v) = config.model_version { self.model_version = v; }
        if let Some(v) = config.release_date { self.release_date = v; }
        if let Some(v) = config.stride_n { self.stride_n = v; }
        if let Some(v) = config.span_w { self.span_w = v; }
        if let Some(v) = config.samples { self.samples = v; }
        if let Some(v) = config.output_file { self.output_file = v; }
        self
    }
}

fn main() -> anyhow::Result<()>{
    let mut args = Cli::parse();
    if let Some(config_path) = &args.config_file {
        let json: String = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&json)?;
        args = args.merge_with_config(config);
    }

    // Each line of the file is an absolute path to a weight file
    println!("Model List {}", args.model_list_file);
    let file = File::open(args.model_list_file)?;
    let reader = BufReader::new(file);
    let paths: Vec<PathBuf> = reader
            .lines()
            .filter_map(|line| match line {
                Ok(s) if !s.trim().is_empty() => Some(PathBuf::from(s.trim())),
                _ => None,
            })
            .collect();
    // make an iter to go for all paths, just testing a single file now
    let model: SafetensorsShard = SafetensorsShard::open(paths.first().unwrap())?;

    // Hash metadata to Check for Shape changes. this is the most egregious and obvious way to alter a model
    //


    if let Some(path) = args.output_file {
        fs::write(path, "json output")?;
    } else {
        println!("{}", "json output std");
    }

    Ok(())
}

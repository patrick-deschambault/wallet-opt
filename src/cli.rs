use clap::Parser;
use config::{Config, File};
use serde::Deserialize;
use std::{error::Error, path::Path};

/// CLI options
#[derive(Parser)]
#[command(name = "Config Loader")]
#[command(about = "Load a TOML config file", long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long)]
    pub config: String,
}

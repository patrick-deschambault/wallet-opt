use clap::Parser;

/// CLI options
#[derive(Parser)]
#[command(name = "Config Loader")]
#[command(about = "Load a TOML config file", long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long)]
    pub config: String,
}

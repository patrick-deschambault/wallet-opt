use config::{Config, File};
use time::OffsetDateTime;
use tokio_test;
use yahoo_finance_api as yahoo;

use serde::Deserialize;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
struct Tickers {
    symbols: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config");

    let settings = load_all_toml_from_dir(path)?;

    let tickers: Tickers = settings.try_deserialize()?;

    for t in tickers.symbols {
        println!("Ticker : {}", t);
    }

    Ok(())
}

fn load_all_toml_from_dir<P: AsRef<Path>>(dir: P) -> Result<Config, Box<dyn Error>> {
    let mut builder = Config::builder();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "toml") {
            builder = builder.add_source(File::from(path));
        }
    }

    Ok(builder.build()?)
}

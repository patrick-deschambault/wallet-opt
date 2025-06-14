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

    let settings = load_all_toml_from_dir(path)?.ok_or("No config files found.")?;

    let tickers: Tickers = settings.try_deserialize()?;

    for t in tickers.symbols {
        println!("Ticker : {}", t);
    }

    Ok(())
}

fn load_all_toml_from_dir<P: AsRef<Path>>(dir: P) -> Result<Option<Config>, Box<dyn Error>> {
    let toml_paths: Vec<_> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "toml"))
        .collect();

    if toml_paths.is_empty() {
        return Ok(None);
    }

    let mut builder = Config::builder();
    for path in toml_paths {
        builder = builder.add_source(File::from(path));
    }

    Ok(Some(builder.build()?))
}

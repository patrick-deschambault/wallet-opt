use config::{Config, File};

use serde::Deserialize;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use yahoo_finance_api::YahooConnector;

use time::macros::datetime;

use clap::Parser;
use wallet_opt::{
    cli::Cli,
    holding::{self, Holding},
};

#[derive(Debug, Deserialize)]
struct Tickers {
    symbols: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let cli = Cli::try_parse()?;

    let date = datetime!(2025-06-13 00:00:00.00 UTC);

    let provider = YahooConnector::new()?;

    let holdings = holding::load_holdings_from_toml(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/holdings.toml"),
        &provider,
    )
    .await?;

    for h in holdings {
        let current_value = h.value_with_date(&provider, &date).await?;
        let initial_value = h.initial_value();

        let roi = ((current_value - initial_value) / initial_value) * 100.0;

        println!(
            "Ticker: {:?}, Initial Value: {:?}, Current Value: {:?}, ROI: {:?}",
            h.stock().ticker(),
            initial_value,
            current_value,
            roi,
        );
    }

    Ok(())
}

fn load_config_file<T>(path: impl AsRef<Path>) -> Result<T, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let settings = Config::builder()
        .add_source(File::from(path.as_ref()))
        .build()?;

    let deserialized: T = settings.try_deserialize()?;
    Ok(deserialized)
}

fn _load_all_toml_from_dir<P: AsRef<Path>>(dir: P) -> Result<Option<Config>, Box<dyn Error>> {
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

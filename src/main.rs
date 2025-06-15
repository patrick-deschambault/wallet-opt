use config::{Config, File};

use serde::Deserialize;
use std::{error::Error, fs, path::Path};

use time::macros::datetime;

use clap::Parser;
use wallet_opt::cli::Cli;

#[derive(Debug, Deserialize)]
struct Tickers {
    symbols: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use futures::future::try_join_all;
    use wallet_opt::stock::Stock;
    use yahoo_finance_api::YahooConnector;

    let cli = Cli::try_parse()?;

    let tickers: Tickers = load_config_file(cli.config)?;

    let start = datetime!(2025-06-13 00:00:00.00 UTC);

    let provider = YahooConnector::new()?;

    let stocks: Vec<Stock> = try_join_all(
        tickers
            .symbols
            .iter()
            .map(|tag| Stock::from_market(&provider, tag, &start)),
    )
    .await?;

    for s in stocks {
        println!("Ticker: {:?}, Price: {:?}", s.ticker(), s.price());
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

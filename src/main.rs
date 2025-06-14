use config::{Config, File};
use time::OffsetDateTime;
use tokio_test;
use yahoo_finance_api as yahoo;

use futures::stream::{self, StreamExt};
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

#[cfg(not(feature = "blocking"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use std::{collections::HashMap, hash::Hash};

    use time::macros::datetime;
    use yahoo_finance_api::YResponse;

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config");

    let settings = load_all_toml_from_dir(path)?.ok_or("No config files found.")?;

    let tickers: Tickers = settings.try_deserialize()?;

    let conn = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2000-07-25 00:00:00.00 UTC);
    let end = datetime!(2024-11-01 00:00:00.00 UTC);

    let y_responses: HashMap<_, _> = stream::iter(tickers.symbols.clone())
        .then(|t| {
            let conn = &conn;
            async move {
                match conn.get_quote_history(&t, start, end).await {
                    Ok(hist) => Some((t, hist)),
                    Err(_) => None,
                }
            }
        })
        .filter_map(|x| async move { x })
        .collect()
        .await;

    let dividends: HashMap<_, _> = y_responses
        .into_iter()
        .filter_map(|(ticker, hist)| {
            // If `dividends()` returns an error, skip this entry
            let total = hist.dividends().ok()?.iter().map(|x| x.amount).sum::<f64>();
            Some((ticker, total))
        })
        .collect();

    // Print
    for (ticker, dividend) in dividends {
        println!(
            "Ticker: {}, start time: {}, end time: {}, accumulated dividends: {}",
            ticker, start, end, dividend
        )
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

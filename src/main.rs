use chrono::Offset;
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

use std::{collections::HashMap, hash::Hash};

use time::macros::datetime;
use wallet_opt::wallet::{self, Stock};
use yahoo_finance_api::YResponse;

use wallet_opt::wallet::StockBuilder;

#[derive(Debug, Deserialize)]
struct Tickers {
    symbols: Vec<String>,
}

#[cfg(not(feature = "blocking"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use futures::future::try_join_all;

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config");

    let settings = load_all_toml_from_dir(path)?.ok_or("No config files found.")?;

    let tickers: Tickers = settings.try_deserialize()?;

    let start = datetime!(2000-07-25 00:00:00.00 UTC);

    let stocks: Vec<Stock> = tickers
        .symbols
        .iter()
        .map(|tag| {
            StockBuilder::default()
                .ticker(tag.clone())
                .time_of_buy(Some(start))
                .build()
        }) // Tie of buy needs to be figured out.
        .collect::<Result<Vec<_>, _>>()?;

    let conn = yahoo::YahooConnector::new().unwrap();

    let now = OffsetDateTime::now_utc();

    let values: Vec<(String, f64)> = stream::iter(&stocks)
        .then(|s| {
            let ticker = s.ticker.clone();
            let conn_ref = &conn;
            async move {
                match s.value(now, conn_ref).await {
                    Ok(val) => Some((ticker, val)),
                    Err(_) => None,
                }
            }
        })
        .filter_map(|x| async move { x })
        .collect()
        .await;

    for v in values {
        println!("Ticker: {}, Value: {}", v.0, v.1);
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

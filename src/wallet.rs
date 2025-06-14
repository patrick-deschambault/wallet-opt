use std::collections::HashMap;

use derive_builder::Builder;
use futures::stream;
use serde::{Deserialize, Deserializer};
use time::OffsetDateTime;
use yahoo_finance_api::YahooConnector;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Wallet {
    stocks: Vec<Stock>,
    cash: f64,
}

#[derive(Clone, Default, Debug, Builder)]
pub struct Stock {
    pub ticker: String,
    time_of_buy: Option<OffsetDateTime>,
}

impl Stock {
    pub async fn value(
        &self,
        end: OffsetDateTime,
        conn: &YahooConnector,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        match self.time_of_buy {
            Some(start) => {
                let ticker = &self.ticker;
                let y_response = conn.get_quote_history(ticker, start, end).await?;

                let first_price_closed = y_response
                    .quotes()?
                    .first()
                    .ok_or("First element not found")?
                    .close;

                let last_price_closed = y_response.last_quote()?.close;

                Ok(last_price_closed - first_price_closed)
            }
            None => Err("No start time defined.".into()),
        }
    }
}
